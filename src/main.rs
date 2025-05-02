use async_graphql::EmptySubscription;
use axum::http::{HeaderValue, Method};
use sqlx::PgPool;
use std::sync::Arc;
use time::UtcOffset;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::{
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};

use daily_task::run_daily_task_at_midnight;
use graphql::{Mutation, Query};
use routes::setup_router;

pub mod daily_task;
pub mod graphql;
pub mod models;
pub mod routes;

/// Handles all over environment variables in one place.
// TODO: Replace with `Config.rs` crate.
struct Config {
    env: String,
    secret_key: String,
    database_url: String,
    port: String,
}

impl Config {
    fn from_env() -> Self {
        let _ = dotenv::dotenv();
        Self {
            env: std::env::var("ROOT_ENV").unwrap_or_else(|_| "development".to_string()),
            secret_key: std::env::var("ROOT_SECRET").expect("ROOT_SECRET must be set."),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
            port: std::env::var("ROOT_PORT").expect("ROOT_PORT must be set."),
        }
    }
}

struct OtelGuard {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}

#[tokio::main]
#[tracing::instrument]
async fn main() {
    let config = Config::from_env();
    let guard = setup_tracing(&config.env);

    let pool = setup_database(&config.database_url).await;
    let schema = build_graphql_schema(pool.clone(), config.secret_key);

    tokio::task::spawn(async {
        run_daily_task_at_midnight(pool).await;
    });

    let cors = setup_cors();
    let router = setup_router(schema, cors, config.env == "development");

    info!("Starting Root...");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    drop(guard);
}

#[tracing::instrument]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");

    tracing::info!("Shutdown signal received. Flushing telemetry...");
}

fn resource() -> Resource {
    Resource::builder()
        .with_attributes(vec![
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
        ])
        .with_schema_url(Vec::new(), SCHEMA_URL)
        .build()
}

fn init_meter_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    let stdout_reader =
        PeriodicReader::builder(opentelemetry_stdout::MetricExporter::default()).build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .with_reader(stdout_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

fn init_tracer_provider() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

fn setup_tracing(env: &str) -> OtelGuard {
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meter_provider();
    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    let kolkata_offset = UtcOffset::from_hms(5, 30, 0).expect("Hardcoded offset must be correct");
    let timer = fmt::time::OffsetTime::new(
        kolkata_offset,
        time::format_description::well_known::Rfc2822,
    );
    if env == "production" {
        tracing_subscriber::registry()
            // Don't waste resources writing to unmonitored stdout in production
            .with(
                fmt::layer()
                    .event_format(fmt::format().with_timer(timer.clone()))
                    .pretty()
                    .with_ansi(false) // ANSI encodings are unreadable in the raw file.
                    .with_writer(std::fs::File::create("root.log").unwrap()),
            )
            .with(MetricsLayer::new(meter_provider.clone()))
            .with(OpenTelemetryLayer::new(tracer))
            .with(EnvFilter::new("info"))
            .init();
        info!("Running in production mode.")
    } else {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .event_format(fmt::format().with_timer(timer.clone()))
                    .pretty()
                    .with_writer(std::io::stdout),
            )
            .with(
                fmt::layer()
                    .event_format(fmt::format().with_timer(timer.clone()))
                    .pretty()
                    .with_ansi(false)
                    .with_writer(std::fs::File::create("root.log").unwrap()),
            )
            .with(MetricsLayer::new(meter_provider.clone()))
            .with(OpenTelemetryLayer::new(tracer))
            .with(EnvFilter::new("trace"))
            .init();
        info!("Running in development mode.");
    }

    OtelGuard {
        tracer_provider,
        meter_provider,
    }
}

async fn setup_database(database_url: &str) -> Arc<PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(2)
        .max_connections(3)
        .connect(database_url)
        .await
        .expect("Pool must be initialized properly.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    Arc::new(pool)
}

fn build_graphql_schema(
    pool: Arc<PgPool>,
    secret_key: String,
) -> async_graphql::Schema<Query, Mutation, EmptySubscription> {
    async_graphql::Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool)
        .data(secret_key)
        .finish()
}

fn setup_cors() -> CorsLayer {
    // TODO: Replace hardcoded strings
    let origins: [HeaderValue; 2] = [
        "http://127.0.0.1:3000".parse().unwrap(),
        "https://home.amfoss.in".parse().unwrap(),
    ];

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
}

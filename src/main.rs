use crate::routes::graphiql;
use async_graphql::EmptySubscription;
use async_graphql_axum::GraphQL;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use chrono_tz::Asia::Kolkata;
use graphql::{mutations::MutationRoot, query::QueryRoot};
use root::attendance::daily_task;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::sleep_until;
use tower_http::cors::CorsLayer;
use tracing::info;

mod graphql;
mod routes;

#[tokio::main]
async fn main() {
    // 12/1/25: Going to assume this is only necessary for shuttle
    // 9/1/25: TODO: Explain?
    // env::set_var("PGOPTIONS", "-c ignore_version=true");

    tracing_subscriber::fmt::init();
    // Currently, we need the DATABASE_URL to be loaded in through the .env.
    // In the future, if we use any other configuration (say Github Secrets), we
    // can allow dotenv() to err.
    dotenv::dotenv().expect("Failed to load .env file.");

    let secret_key = std::env::var("ROOT_SECRET").expect("ROOT_SECRET must be set.");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(2) // Maintain at least two connections, one for amD and one for Home. It should be
        .max_connections(3) // pretty unlikely that amD, Home and the web interface is used simultaneously
        .connect(&database_url)
        .await
        .expect("Pool must be initialized properly.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    // Wrap pool in an Arc to share across threads
    let pool = Arc::new(pool);

    let schema = async_graphql::Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .data(secret_key)
        .finish();

    // This thread will sleep until it's time to run the daily task
    // Also takes ownership of pool
    tokio::task::spawn(async {
        run_daily_task_at_midnight(pool).await;
    });

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://home.amfoss.in")) // Only allow requests from Home
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    info!("Starting Root...");
    let router = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .layer(cors);

    // TODO: Replace hardcoded address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn run_daily_task_at_midnight(pool: Arc<PgPool>) {
    loop {
        let now = chrono::Local::now().with_timezone(&Kolkata);
        let next_midnight = (now + chrono::Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let duration_until_midnight = next_midnight.signed_duration_since(now.naive_local());
        let sleep_duration =
            tokio::time::Duration::from_secs(duration_until_midnight.num_seconds() as u64);

        sleep_until(tokio::time::Instant::now() + sleep_duration).await;
        daily_task::execute_daily_task(pool.clone()).await;
    }
}

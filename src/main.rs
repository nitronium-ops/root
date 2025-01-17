use async_graphql::{EmptySubscription, MergedObject};
use async_graphql_axum::GraphQL;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use chrono_tz::Asia::Kolkata;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::sleep_until;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use daily_task::execute_daily_task;
use graphql::{
    mutations::{AttendanceMutations, MemberMutations, StreakMutations},
    queries::{AttendanceQueries, MemberQueries, StreakQueries},
};

/// Daily task contains the function that is executed daily at midnight, using the thread spawned in main().
pub mod daily_task;
/// This module handles all logic for queries and mutations, based on the [`crate::models`]. Each sub-module maps to one table in the DB.
pub mod graphql;
/// These models not only help SQLx map it to the relational DB, but is also used by async_graphql to define its resolvers for queries and mutations.
pub mod models;
/// Since we really only need one route for a GraphQL server, this just holds a function returning the GraphiQL playground. Probably can clean this up later.
pub mod routes;

// This is our main query or QueryRoot. It is made up of structs representing sub-queries, one for each table in the DB. The fields of a relation are exposed via the [`async_graphql::SimpleObject`] directive on the [`models`] themselves. Specific queries, such as getting a member by ID or getting the streak of a member is defined as methods of the sub-query struct. Complex queries, such as those getting related data from multiple tables like querying all members and the streaks of each member, are defined via the [`async_graphql::ComplexObject`] directive on the [`models`] and can be found in the corresponding sub-query module.
#[derive(MergedObject, Default)]
struct Query(MemberQueries, AttendanceQueries, StreakQueries);

// Mutations work the same as Queries, sub-modules for each relation in the DB. However, all methods are directly defined on these sub-module structs. But they use slightly modified versions of the [`models`], marked by the Input in the name, to get input.
#[derive(MergedObject, Default)]
struct Mutations(MemberMutations, AttendanceMutations, StreakMutations);

#[tokio::main]
async fn main() {
    // 12/1/25: Going to assume this is only necessary for shuttle.
    // 9/1/25: TODO: Explain?
    // env::set_var("PGOPTIONS", "-c ignore_version=true");

    // Currently, we need the DATABASE_URL to be loaded in through the .env.
    // In the future, if we use any other configuration (say Github Secrets), we
    // can allow dotenv() to err.
    dotenv::dotenv().expect("Failed to load .env file.");

    // Used to check if it's in production
    let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    let secret_key = std::env::var("ROOT_SECRET").expect("ROOT_SECRET must be set.");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    if env == "development" {
        tracing_subscriber::registry()
            .with(fmt::layer().pretty().with_writer(std::io::stdout))
            .with(
                fmt::layer()
                    .pretty()
                    .with_ansi(false)
                    .with_writer(std::fs::File::create("root.log").unwrap()),
            )
            .with(EnvFilter::new("trace"))
            .init();
        info!("Running in development mode.");
    } else {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .pretty()
                    .with_ansi(false)
                    .with_writer(std::fs::File::create("root.log").unwrap()),
            )
            .with(EnvFilter::new("info"))
            .init();
        info!("Running in production mode.")
    }

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

    let schema =
        async_graphql::Schema::build(Query::default(), Mutations::default(), EmptySubscription)
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
    // TODO: Avoid exposing the GraphiQL interface in prod.
    let router = Router::new()
        .route(
            "/",
            get(routes::graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .layer(cors);

    // TODO: Replace hardcoded address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

/// Sleep till midnight, then run the 'execute_daily_task' function.
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
        execute_daily_task(pool.clone()).await;
    }
}

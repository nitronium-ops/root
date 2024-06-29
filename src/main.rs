use std::{env, sync::Arc};

use async_graphql_axum::GraphQL;
use axum::{routing::get, Router};
use sqlx::PgPool;
use async_graphql::{ Schema, EmptySubscription};

use crate::graphql::mutations::MutationRoot;
use crate::graphql::query::QueryRoot;
use crate::routes::graphiql;


mod db;
mod graphql;
mod routes;

#[derive(Clone)]
struct MyState {
    pool: Arc<PgPool>,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    env::set_var("PGOPTIONS", "-c ignore_version=true");
    
    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

    let pool = Arc::new(pool);
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();

    let state = MyState { pool };
    let router = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema.clone())))
        .with_state(state);

    Ok(router.into())
}

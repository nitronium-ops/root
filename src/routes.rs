use async_graphql::{
    http::GraphiQLSource, EmptySubscription, Response, Schema, ServerError, Value,
};
use async_graphql_axum::{GraphQL, GraphQLRequest};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::api_key::{get_api_key, verify_api_key};
use crate::auth::{github_callback, github_login};
use crate::graphql::{Mutation, Query};

#[derive(Clone)]
pub struct RootState {
    pub schema: Schema<Query, Mutation, EmptySubscription>,
    pub pool: Arc<PgPool>,
}

pub fn setup_router(
    schema: Schema<Query, Mutation, EmptySubscription>,
    cors: CorsLayer,
    is_dev: bool,
    pool: Arc<PgPool>,
) -> Router {
    let state = RootState {
        schema: schema.clone(),
        pool: pool.clone(),
    };

    let router = Router::new()
        .route("/", post(graphql_handler))
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
        .route("/api/get_key", post(get_api_key))
        .with_state(state)
        .layer(cors);

    if is_dev {
        tracing::info!("GraphiQL playground enabled at /graphiql");
        router.route(
            "/graphiql",
            get(graphiql).post_service(GraphQL::new(schema)),
        )
    } else {
        router
    }
}

async fn graphql_handler(
    headers: HeaderMap,
    State(state): State<RootState>,
    req: GraphQLRequest,
) -> impl IntoResponse {
    let schema = state.schema.clone();
    let received_api_key = headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string())
        .unwrap_or_default();

    if let Ok(is_valid) = verify_api_key(received_api_key.clone(), state.pool.clone()).await {
        if !is_valid {
            let mut response = Response::new(Value::Null);
            response
                .errors
                .push(ServerError::new("Invalid API key", None));
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::response::Json(response),
            );
        }
    } else {
        let mut response = Response::new(Value::Null);
        response
            .errors
            .push(ServerError::new("Internal server error", None));
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::response::Json(response),
        );
    }

    let response = schema.execute(req.into_inner()).await;
    (axum::http::StatusCode::OK, axum::response::Json(response))
}

#[axum::debug_handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphiql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

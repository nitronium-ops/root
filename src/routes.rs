use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::graphql::{Mutation, Query};

/// Setups the router with the given Schema and CORSLayer. Additionally, adds the GraphiQL playground if `is_dev` is true.
pub fn setup_router(
    schema: Schema<Query, Mutation, EmptySubscription>,
    cors: CorsLayer,
    is_dev: bool,
) -> Router {
    let router = Router::new()
        .route_service("/", GraphQL::new(schema))
        .layer(cors);

    if is_dev {
        // Add GraphiQL playground only in development mode
        tracing::info!("GraphiQL playground enabled at /graphiql");
        router.route("/graphiql", get(graphiql))
    } else {
        router
    }
}

/// Returns the built-in GraphQL playground from async_graphql.
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphiql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

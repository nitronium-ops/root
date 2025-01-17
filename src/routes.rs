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
    let mut router = Router::new()
        .route_service("/graphql", GraphQL::new(schema.clone()))
        .layer(cors);

    if is_dev {
        // Add GraphiQL playground only in development mode
        router = router.route("/", get(graphiql));
    }

    router
}

// TODO: We do not want to expose GraphiQL unless in dev.
/// Returns the built-in GraphQL playground from async_graphql.
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

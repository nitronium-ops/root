use async_graphql::http::GraphiQLSource;
use axum::response::{Html, IntoResponse};

// TODO: We do not want to expose GraphiQL unless in dev.
/// Returns the built-in GraphQL playground from async_graphql.
pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

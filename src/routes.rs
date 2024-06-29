
use axum::response::{Html, IntoResponse};
use async_graphql::http::GraphiQLSource;

pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

use crate::api::router::GRAPHIQL_ROUTE;
use crate::api::router::SECURE_PREFIX;
use async_graphql::http::GraphiQLSource;
use axum::response::Html;
use axum::response::IntoResponse;

pub(crate) async fn private_graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(&format!("{}{}", SECURE_PREFIX, GRAPHIQL_ROUTE))
            .finish(),
    )
}

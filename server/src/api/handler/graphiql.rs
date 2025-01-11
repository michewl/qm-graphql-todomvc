use crate::api::router::GRAPHIQL_ROUTE;
use async_graphql::http::GraphiQLSource;
use axum::response::Html;
use axum::response::IntoResponse;

pub(crate) async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint(GRAPHIQL_ROUTE).finish())
}

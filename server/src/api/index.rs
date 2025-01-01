use axum::response::Html;
use axum::response::IntoResponse;

/// The handler for the index page.
pub(crate) async fn index_handler() -> impl IntoResponse {
    Html(
        "<html><h1>Quick Microservice GraphQL TodoMVC Server API</h1><div>Visit <a href=\"/api/graphql\">GraphQL Playground</a></html>"
    )
}

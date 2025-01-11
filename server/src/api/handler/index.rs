use axum::response::Html;
use axum::response::IntoResponse;

/// The handler for the index page.
pub(crate) async fn index_handler() -> impl IntoResponse {
    Html(
        "<html style=\"background-color:#282c34;color:#abb2bf;\"><h1>Quick Microservice GraphQL TodoMVC Server API</h1><div>Visit the <a href=\"/api/graphql\" style=\"color:#61afef;\">GraphQL Playground</a></html>"
    )
}

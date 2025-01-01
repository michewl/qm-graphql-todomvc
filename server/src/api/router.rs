use super::index::index_handler;
use axum::Router;

/// Get the router defining the API endpoints.
pub(crate) async fn get(app: qgt_server::App) -> Router {
    Router::new()
        .route("/", axum::routing::get(index_handler))
        .with_state(app)
}

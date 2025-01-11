use super::handler::graphiql::graphiql_handler;
use super::handler::graphql::graphql_handler;
use super::handler::index::index_handler;
use axum::Extension;
use axum::Router;
use qgt_domain::app::App;

pub(crate) const GRAPHIQL_ROUTE: &str = "/api/graphql";

/// Get the router defining the API endpoints.
pub(crate) async fn get(app: App) -> Router {
    let schema = qgt_domain::schema::SchemaBuilder::default().build(app.clone());
    // Write the schema to a file if we run at debug level
    #[cfg(debug_assertions)]
    if tracing::enabled!(tracing::Level::DEBUG) {
        let mut file =
            std::fs::File::create("schema.graphql").expect("creating the schema file should work");
        std::io::Write::write_all(&mut file, schema.sdl().as_bytes())
            .expect("writing the schema file should work");
    }
    Router::new()
        .route("/", axum::routing::get(index_handler))
        .route(
            GRAPHIQL_ROUTE,
            axum::routing::get(graphiql_handler).post(graphql_handler),
        )
        .with_state(app)
        .layer(Extension(schema))
}

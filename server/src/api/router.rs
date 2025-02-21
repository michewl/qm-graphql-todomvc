use super::handler::graphiql::graphiql_handler;
use super::handler::graphql::graphql_handler;
use super::handler::index::index_handler;
use super::handler::private_graphiql::private_graphiql_handler;
use super::handler::private_graphql::private_graphql_handler;
use super::middleware::redirect_if_unauthorized;
use super::middleware::set_authorization_header;
use axum::Extension;
use axum::Router;
use qgt_auth::keycloak_auth_layer;
use qgt_domain::app::App;
use tower::ServiceBuilder;

pub(crate) const GRAPHIQL_ROUTE: &str = "/api/graphql";
pub(crate) const SECURE_PREFIX: &str = "/secure";

/// Get the router defining the API endpoints.
pub(crate) async fn get(app: App) -> Router {
    let schema = qgt_domain::schema::SchemaBuilder::default().build(app.clone());
    let private_schema = qgt_domain::private_schema::SchemaBuilder::default().build(app.clone());
    // Write the schema to a file if we run at debug level
    #[cfg(debug_assertions)]
    if tracing::enabled!(tracing::Level::DEBUG) {
        let mut file =
            std::fs::File::create("schema.graphql").expect("creating the schema file should work");
        std::io::Write::write_all(&mut file, schema.sdl().as_bytes())
            .expect("writing the schema file should work");
        let mut private_file = std::fs::File::create("private_schema.graphql")
            .expect("creating the private schema file should work");
        std::io::Write::write_all(&mut private_file, private_schema.sdl().as_bytes())
            .expect("writing the schema file should work");
    }
    Router::new()
        .route("/", axum::routing::get(index_handler))
        .route(
            GRAPHIQL_ROUTE,
            axum::routing::get(graphiql_handler).post(graphql_handler),
        )
        .nest(
            SECURE_PREFIX,
            Router::new()
                .route(
                    GRAPHIQL_ROUTE,
                    axum::routing::get(private_graphiql_handler).post(private_graphql_handler),
                )
                .layer(
                    ServiceBuilder::new()
                        // TODO: investigate if redirect loop can be avoided (maybe redirect to /secure, not the full route back form Keycloak)
                        .layer(axum::middleware::from_fn_with_state(
                            app.clone(),
                            set_authorization_header,
                        ))
                        .layer(axum::middleware::from_fn_with_state(
                            app.clone(),
                            redirect_if_unauthorized,
                        ))
                        .layer(keycloak_auth_layer(app.auth_ctx().instance())),
                ),
        )
        .with_state(app)
        .layer(Extension(schema))
        .layer(Extension(private_schema))
}

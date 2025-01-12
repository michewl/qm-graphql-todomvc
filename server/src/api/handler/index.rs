use axum::response::Html;
use axum::response::IntoResponse;

/// The handler for the index page.
pub(crate) async fn index_handler() -> impl IntoResponse {
    let content = format!(
        r#"
        <!DOCTYPE html>
        <html style="background-color:#282c34;color:#abb2bf;text-align:center;">
            <head><title>QMG TodoMVC (v{version})</title></head>
            <body>
                <h1>Quick Microservice GraphQL TodoMVC Server API (<code>v{version}</code>)</h1>
                <div>Visit the <a href="/api/graphql" style="color:#61afef;">GraphQL Playground</a></div>
            </body>
        </html>
        "#,
        version = env!("CARGO_PKG_VERSION")
    );
    Html(content)
}

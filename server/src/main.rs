//! # Quick Microservice GraphQL TodoMVC Server
//!
//! Runs a server with the GraphQL API.

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This loads the environment variables from `.env` if it exists
    if let Err(e) = dotenv::dotenv() {
        tracing::info!("The '.env' file could not be loaded.\n{}", &e);
    }

    // Set a global tracing subscriber
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .expect("the global default tracing subscriber should be settable");

    // Load the app
    let app = qgt_domain::app::App::new().await?;

    // Start the server
    tracing::info!(
        "Starting server at address http://{}",
        &app.server_config().address()
    );
    axum::serve(
        tokio::net::TcpListener::bind::<std::net::SocketAddr>(
            app.server_config()
                .address()
                .parse()
                .expect("the server config address should be a valid socket address"),
        )
        .await
        .expect("socket should be bindable"),
        api::router::get(app).await.into_make_service(),
    )
    .await?;

    Ok(())
}

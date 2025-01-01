//! # Quick Microservice GraphQL TodoMVC App
//!
//! The main app container which provides global data structures and initialization functionality.
//!
//! # Examples
//! ```rust
//! let app = qgt_server::App::new();
//! ```

use std::sync::Arc;

struct AppInner {
    server_config: qm::server::ServerConfig,
}

/// The app state.
#[derive(Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        // Uses defaults from the server crate.
        // Can be configured with environment variables with prefix 'SERVER_'.
        // Available settings: 'SERVER_APP_NAME', 'SERVER_HOST', 'SERVER_PORT'
        let server_config = qm::server::ServerConfig::new()?;

        Ok(Self {
            inner: Arc::new(AppInner { server_config }),
        })
    }

    pub fn server_config(&self) -> &qm::server::ServerConfig {
        &self.inner.server_config
    }
}

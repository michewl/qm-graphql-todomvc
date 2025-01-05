//! # Quick Microservice GraphQL TodoMVC App
//!
//! The main app container which provides global data structures and initialization functionality.
//!
//! # Examples
//! ```rust
//! let app = qgt_server::App::new();
//! ```

use db::setup_database;
use std::sync::Arc;

mod db;

struct AppInner {
    server_config: qm::server::ServerConfig,
    db: qm::mongodb::DB,
}

/// The app state.
#[derive(Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        // Uses defaults from the qm server crate.
        // Can be configured with environment variables with prefix 'SERVER_'.
        let server_config = qm::server::ServerConfig::new()?;

        // Uses defaults from the qm mongodb crate.
        // Can be configured with environment variables with prefix 'MONGODB_'.
        let db =
            qm::mongodb::DB::new(server_config.app_name(), &qm::mongodb::DbConfig::new()?).await?;
        // Set up the MongoDB for qgt
        setup_database(&db).await?;

        Ok(Self {
            inner: Arc::new(AppInner { server_config, db }),
        })
    }

    pub fn server_config(&self) -> &qm::server::ServerConfig {
        &self.inner.server_config
    }

    pub fn db(&self) -> &qm::mongodb::DB {
        &self.inner.db
    }
}

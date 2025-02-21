//! # Quick Microservice GraphQL TodoMVC App
//!
//! The main app container which provides global data structures and initialization functionality.
//!
//! # Examples
//! ```rust
//! let app = qgt_domain::app::App::new().await?;
//! ```

use crate::db::setup_database;
use qgt_auth::ctx::AuthContext;
use std::sync::Arc;

struct AppInner {
    auth_ctx: AuthContext,
    db: qm::mongodb::DB,
    server_config: qm::server::ServerConfig,
}

/// The app state.
#[derive(Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

impl App {
    /// Construct a new [App].
    ///
    /// Will initialize
    /// - [qm::server::ServerConfig]
    /// - [qm::mongodb::DB]
    /// - [qgt_auth::ctx::AuthContext]
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

        // Set up the auth context
        let auth_ctx = AuthContext::new()?;

        Ok(Self {
            inner: Arc::new(AppInner {
                auth_ctx,
                db,
                server_config,
            }),
        })
    }

    /// Get the server configuration.
    pub fn server_config(&self) -> &qm::server::ServerConfig {
        &self.inner.server_config
    }

    /// Get the database.
    pub fn db(&self) -> &qm::mongodb::DB {
        &self.inner.db
    }

    /// Get the [AuthContext].
    pub fn auth_ctx(&self) -> AuthContext {
        self.inner.auth_ctx.clone()
    }
}

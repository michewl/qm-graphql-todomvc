use crate::config::AuthConfig;
use axum_keycloak_auth::instance::KeycloakAuthInstance;
use axum_keycloak_auth::instance::KeycloakConfig;
use axum_keycloak_auth::Url;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AuthContext {
    config: AuthConfig,
    instance: Arc<KeycloakAuthInstance>,
    token: Arc<Mutex<Option<String>>>,
}

impl AuthContext {
    pub fn new() -> anyhow::Result<Self> {
        let config = AuthConfig::from_env()?;
        let instance = KeycloakAuthInstance::new(
            KeycloakConfig::builder()
                .server(
                    Url::parse(format!("http://{}", config.address()).as_str())
                        .expect("url should be parsable"),
                )
                .realm(config.realm().to_string())
                .build(),
        );

        Ok(Self {
            config,
            instance: Arc::new(instance),
            token: Arc::new(Mutex::new(None)),
        })
    }

    pub fn config(&self) -> &AuthConfig {
        &self.config
    }

    pub fn instance(&self) -> Arc<KeycloakAuthInstance> {
        self.instance.clone()
    }

    pub fn token(&self) -> Option<String> {
        self.token.lock().expect("token should be lockable").clone()
    }

    pub fn set_token(&mut self, token: Option<String>) {
        let mut guard = self.token.lock().expect("token should be lockable");
        *guard = token;
    }
}

use serde::Deserialize;

/// Authentication configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    auth_host: Option<String>,
    auth_port: Option<u16>,
    auth_address: Option<String>,
    auth_realm: Option<String>,
    auth_client_id: Option<String>,
}

impl AuthConfig {
    const DEFAULT_HOST: &'static str = "localhost";
    const DEFAULT_PORT: u16 = 8080;
    const DEFAULT_REALM: &'static str = "QGT";
    const DEFAULT_CLIENT_ID: &'static str = "qgt";

    /// Get an auth config instance with values from env variables.
    pub fn from_env() -> envy::Result<Self> {
        let mut cfg =
            envy::from_env::<AuthConfig>().expect("auth config should be parsable from env");

        // Set defaults if not provided from environment
        if cfg.auth_host.is_none() {
            cfg.auth_host = Some(String::from(Self::DEFAULT_HOST));
        }
        if cfg.auth_port.is_none() {
            cfg.auth_port = Some(Self::DEFAULT_PORT);
        }
        if cfg.auth_address.is_none() {
            cfg.auth_address = Some(format!(
                "{}:{}",
                cfg.auth_host.as_deref().unwrap(),
                cfg.auth_port.unwrap()
            ));
        }
        if cfg.auth_realm.is_none() {
            cfg.auth_realm = Some(String::from(Self::DEFAULT_REALM));
        }
        if cfg.auth_client_id.is_none() {
            cfg.auth_client_id = Some(String::from(Self::DEFAULT_CLIENT_ID));
        }

        Ok(cfg)
    }

    pub fn address(&self) -> &str {
        self.auth_address.as_deref().unwrap()
    }

    pub fn realm(&self) -> &str {
        self.auth_realm.as_deref().unwrap()
    }

    pub fn client_id(&self) -> &str {
        self.auth_client_id.as_deref().unwrap()
    }
}

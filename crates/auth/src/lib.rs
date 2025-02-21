use std::sync::Arc;

use axum_keycloak_auth::{
    instance::KeycloakAuthInstance, layer::KeycloakAuthLayer, PassthroughMode,
};

pub mod config;
pub mod ctx;

pub fn keycloak_auth_layer(instance: Arc<KeycloakAuthInstance>) -> KeycloakAuthLayer<String> {
    KeycloakAuthLayer::<String>::builder()
        .instance(instance)
        .passthrough_mode(PassthroughMode::Block)
        .persist_raw_claims(false)
        .expected_audiences(vec![String::from("account")])
        .build()
}

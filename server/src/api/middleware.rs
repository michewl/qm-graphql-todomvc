use super::router::GRAPHIQL_ROUTE;
use super::router::SECURE_PREFIX;
use axum::extract::Query;
use axum::extract::Request;
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Redirect;
use qgt_domain::app::App;
use reqwest::Client;
use std::collections::HashMap;

/// Redirect to log-in if the request was unauthorized.
pub(crate) async fn redirect_if_unauthorized(
    State(app): State<App>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let response = next.run(req).await;
    if response.status() == StatusCode::UNAUTHORIZED {
        let redirect_uri = format!(
            "http://{}/realms/{}/protocol/openid-connect/auth?scope=openid&response_type=code&client_id={}&redirect_uri=http://{}{}{}",
            app.auth_ctx().config().address(),
            app.auth_ctx().config().realm(),
            app.auth_ctx().config().client_id(),
            app.server_config().address(),
            SECURE_PREFIX,
            GRAPHIQL_ROUTE
        );
        return Ok(Redirect::to(redirect_uri.as_str()).into_response());
    }

    Ok(response)
}

/// Set the authorization header.
///
/// Does nothing if we already have a [axum::http::header::AUTHORIZATION] header set.
///
/// ## Note
/// This is a very inefficient and unsecure way to handle authentication.
/// It is also not very stable, but it is enough for the demonstration of this application.
///
/// **Do not replicate this in an actual project.**
pub(crate) async fn set_authorization_header(
    State(app): State<App>,
    Query(query_params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Skip if the authorization header is already present
    if headers.contains_key(AUTHORIZATION) {
        return Ok(next.run(req).await);
    }

    if let Some(access_token) = app.auth_ctx().token().as_ref() {
        tracing::info!("Got access token from auth context");
        tracing::debug!("Access token from auth context: {access_token}");
        req.headers_mut().append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {access_token}"))
                .expect("string should be convertable to HeaderValue"),
        );
    } else if query_params.contains_key("code") {
        tracing::info!("Requesting access token from keycloak");
        // Get the JWT
        let response = Client::new()
            .post(format!(
                "http://{}/realms/{}/protocol/openid-connect/token",
                app.auth_ctx().config().address(),
                app.auth_ctx().config().realm()
            ))
            .form(&[
                ("grant_type", "authorization_code"),
                (
                    "code",
                    &query_params
                        .get("code")
                        .expect("the 'code' query param should exist"),
                ),
                ("client_id", app.auth_ctx().config().client_id()),
                (
                    "redirect_uri",
                    format!(
                        "http://{}{}{}",
                        app.server_config().address(),
                        SECURE_PREFIX,
                        GRAPHIQL_ROUTE
                    )
                    .as_str(),
                ),
            ])
            .send()
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to exchange code for token retrieval".to_string(),
                )
            })?;

        if response.status() != StatusCode::OK {
            return Err((response.status(), "Failed to get token".to_string()));
        }

        let token_response: serde_json::Value = response.json().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse token response".to_string(),
            )
        })?;

        // Extract the access token
        let access_token = token_response["access_token"].as_str().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No access token found".to_string(),
        ))?;
        tracing::debug!("Access token from keycloak: {access_token}");

        app.auth_ctx().set_token(Some(access_token.to_string()));
        req.headers_mut().append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {access_token}"))
                .expect("string should be convertable to HeaderValue"),
        );
    } else {
        tracing::info!("Inserting authorization header skipped");
    }

    Ok(next.run(req).await)
}

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    api::{
        error::{ErrorResponse, Status},
        response::TokenClaims,
    },
    state::AppState,
};
use std::sync::Arc;

pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });
    let token = token.ok_or_else(|| {
        let json_err = ErrorResponse {
            status: Status::Error,
            message: "Missing bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_err))
    })?;
    let secret = &state.setting.load().token_secret;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: Status::Error,
            message: "Invalid bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

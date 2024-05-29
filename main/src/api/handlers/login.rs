use anyhow::anyhow;
use argon2::Argon2;
use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::PasswordHash;
use password_hash::PasswordVerifier;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::{
    api::{
        request::login::LoginRequest,
        response::{login::LoginResponse, TokenClaims},
    },
    state::AppState,
};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    match entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&payload.username))
        .all(state.db_conn.load().as_ref())
        .await
    {
        Ok(admins) => {
            if admins.is_empty() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            let admin = &admins[0];
            if validate_password(&payload.password, &admin.password).is_err() {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }

    let secret = &state.setting.load().token_secret;
    let timeout = state.setting.load().token_timeout_seconds;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(timeout)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))
}

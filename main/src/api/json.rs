use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::api::error::Status;

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for CustomJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                Json(json!({
                    "status": Status::Error,
                    "message": rejection.body_text(),
                })),
            )),
        }
    }
}

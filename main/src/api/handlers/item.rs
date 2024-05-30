use axum::{
    extract::{Path, State},
    Extension, Json,
};
use entity::item::ItemCreateRequest;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;

use crate::{
    api::{
        error::{AppError, Status},
        json::CustomJson,
        response::{
            item::{ItemCreateResponse, ItemGetResponse, ItemListResponse},
            TokenClaims,
        },
    },
    state::AppState,
};

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<AppState>>,
    CustomJson(payload): CustomJson<ItemCreateRequest>,
) -> Result<Json<ItemCreateResponse>, AppError> {
    let item_active_model = payload.into_active_model();
    let item_model = item_active_model
        .save(state.db_conn.load().as_ref())
        .await?;
    let item = item_model.try_into_model()?;
    let response = ItemCreateResponse {
        status: Status::Success,
        data: Some(item),
    };

    Ok(Json(response))
}

pub async fn list(State(state): State<Arc<AppState>>) -> Result<Json<ItemListResponse>, AppError> {
    let items = entity::item::Entity::find()
        .all(state.db_conn.load().as_ref())
        .await?;

    let n = items.len();

    let response = ItemListResponse {
        status: Status::Success,
        data: items,
    };

    tracing::info!("number of dogs: {}", n);

    Ok(Json(response))
}

pub async fn get(
    Path(item_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ItemGetResponse>, AppError> {
    let item = entity::item::Entity::find_by_id(item_id)
        .one(state.db_conn.load().as_ref())
        .await?;

    let response = ItemGetResponse {
        status: Status::Success,
        data: item,
    };

    Ok(Json(response))
}

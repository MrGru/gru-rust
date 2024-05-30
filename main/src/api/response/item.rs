use entity::item::Model;
use serde::Serialize;

use crate::api::error::Status;

#[derive(Serialize)]
pub struct ItemCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize)]
pub struct ItemListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize)]
pub struct ItemGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}

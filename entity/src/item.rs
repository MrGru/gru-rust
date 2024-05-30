use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    pub price: i32,
    pub r#type: i32,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize, DeriveIntoActiveModel)]
pub struct ItemCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub r#type: i32,
}

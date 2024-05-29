use std::sync::Arc;

use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;

use crate::setting::Setting;

pub struct AppState {
    pub db_conn: ArcSwap<DatabaseConnection>,
    pub setting: ArcSwap<Setting>,
}

impl AppState {
    pub fn new(setting: &Setting, db_conn: DatabaseConnection) -> anyhow::Result<Self> {
        Ok(Self {
            db_conn: ArcSwap::new(Arc::new(db_conn)),
            setting: ArcSwap::new(Arc::new((*setting).clone())),
        })
    }
}

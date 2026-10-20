use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Item {
    pub id: i64,
    pub node_id: i64,
    pub text: String,
    pub done: bool,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Deserialize)]
pub struct NewItem {
    pub node_id: i64,
    pub text: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemPatch {
    pub text: Option<String>,
    pub done: Option<bool>,
}

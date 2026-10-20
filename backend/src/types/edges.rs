use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Edge {
    pub id: i64,
    pub parent_id: i64,
    pub source_id: i64,
    pub target_id: i64,
    pub label: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Deserialize)]
pub struct NewEdge {
    pub parent_id: i64,
    pub source_id: i64,
    pub target_id: i64,
    #[serde(default)]
    pub label: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgePatch {
    pub label: Option<String>,
}

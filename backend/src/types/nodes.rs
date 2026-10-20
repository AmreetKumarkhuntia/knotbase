use crate::types::{edges::Edge, items::Item};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Node {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub doc: String,
    pub x: f64,
    pub y: f64,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(FromRow, Serialize)]
pub struct Crumb {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
#[derive(Serialize)]
pub struct NodeDetail {
    #[serde(flatten)]
    pub node: Node,
    pub items: Vec<Item>,
    pub breadcrumb: Vec<Crumb>,
}
#[derive(Deserialize)]
pub struct NewNode {
    pub parent_id: i64,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodePatch {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub doc: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

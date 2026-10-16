use crate::{
    edges::Edge,
    error::{AppError, Result},
    items::Item,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Serialize)]
pub struct Node {
    id: i64,
    parent_id: Option<i64>,
    name: String,
    kind: String,
    doc: String,
    x: f64,
    y: f64,
    created_at: String,
    updated_at: String,
}
#[derive(FromRow, Serialize)]
struct Crumb {
    id: i64,
    name: String,
}
#[derive(Serialize)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
#[derive(Serialize)]
struct NodeDetail {
    #[serde(flatten)]
    node: Node,
    items: Vec<Item>,
    breadcrumb: Vec<Crumb>,
}
#[derive(Deserialize)]
struct NewNode {
    parent_id: i64,
    name: String,
    kind: String,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NodePatch {
    name: Option<String>,
    kind: Option<String>,
    doc: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
}

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/nodes", post(create_node))
        .route(
            "/nodes/{id}",
            get(get_node).patch(patch_node).delete(delete_node),
        )
        .route("/nodes/{id}/graph", get(graph))
}

// shortcut: GET /graph on a missing id returns 200 with empty lists (the page's GET /api/nodes/{id} supplies the 404); add an existence check if /graph is ever called alone.
async fn graph(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Graph>> {
    // '' AS doc: the canvas never reads child docs, and they made the response megabytes
    let nodes = sqlx::query_as(
        "SELECT id, parent_id, name, kind, '' AS doc, x, y, created_at, updated_at FROM node WHERE parent_id = ? ORDER BY id",
    )
        .bind(id)
        .fetch_all(&pool)
        .await?;
    let edges = sqlx::query_as("SELECT * FROM edge WHERE parent_id = ? ORDER BY id")
        .bind(id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(Graph { nodes, edges }))
}

async fn get_node(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<NodeDetail>> {
    let node = sqlx::query_as("SELECT * FROM node WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    // shortcut: items are id order and item.node_id, edge(parent_id, source_id|target_id) have no index; cascade deletes scan both tables (2.3 s at ~11k nodes vs 13 ms indexed), add a migration with those three indexes at ~10k rows, and an INTEGER position column when drag-reorder is wanted.
    let items = sqlx::query_as("SELECT * FROM item WHERE node_id = ? ORDER BY id")
        .bind(id)
        .fetch_all(&pool)
        .await?;
    let breadcrumb = sqlx::query_as(
        "WITH RECURSIVE up(id, parent_id, name, depth) AS (
           SELECT id, parent_id, name, 0 FROM node WHERE id = ?
           UNION ALL
           SELECT n.id, n.parent_id, n.name, up.depth + 1 FROM node n JOIN up ON n.id = up.parent_id
         )
         SELECT id, name FROM up ORDER BY depth DESC",
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;
    Ok(Json(NodeDetail {
        node,
        items,
        breadcrumb,
    }))
}

async fn create_node(
    State(pool): State<SqlitePool>,
    Json(b): Json<NewNode>,
) -> Result<(StatusCode, Json<Node>)> {
    // shortcut: no pre-check SELECT to turn a bad parent_id into 404, add if the UI needs to distinguish.
    let node = sqlx::query_as(
        "INSERT INTO node (parent_id, name, kind, x, y) VALUES (?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(b.parent_id)
    .bind(b.name.trim())
    .bind(b.kind.trim())
    .bind(b.x)
    .bind(b.y)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(node)))
}

// shortcut: parent_id is not patchable and sending it is a 422 (no move between graphs; the edge FKs are ON UPDATE NO ACTION so a move with edges would fail anyway); add a move endpoint that deletes the node's edges first when drag-to-reparent is wanted, with a cycle guard (the breadcrumb CTE would loop on a cycle) and id 1 must never move.
async fn patch_node(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(p): Json<NodePatch>,
) -> Result<Json<Node>> {
    let node = sqlx::query_as(
        "UPDATE node SET name = COALESCE(?, name), kind = COALESCE(?, kind), doc = COALESCE(?, doc),
           x = COALESCE(?, x), y = COALESCE(?, y), updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ? RETURNING *",
    )
    .bind(p.name.as_deref().map(str::trim))
    .bind(p.kind.as_deref().map(str::trim))
    .bind(p.doc)
    .bind(p.x)
    .bind(p.y)
    .bind(id)
    .fetch_one(&pool)
    .await?;
    Ok(Json(node))
}

async fn delete_node(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    if id == 1 {
        return Err(AppError::BadRequest("root cannot be deleted".into())); // only the migration can create it
    }
    // shortcut: a subtree deeper than ~1000 levels fails with 500 (SQLite trigger depth limit, rolled back, nothing lost); delete bottom-up if that ever matters.
    sqlx::query("DELETE FROM node WHERE id = ? RETURNING id")
        .bind(id)
        .fetch_one(&pool)
        .await?; // no row -> RowNotFound -> 404
    Ok(StatusCode::NO_CONTENT)
}

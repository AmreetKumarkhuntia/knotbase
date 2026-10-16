use crate::error::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{patch, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Serialize)]
pub struct Edge {
    id: i64,
    parent_id: i64,
    source_id: i64,
    target_id: i64,
    label: String,
    created_at: String,
    updated_at: String,
}
#[derive(Deserialize)]
struct NewEdge {
    parent_id: i64,
    source_id: i64,
    target_id: i64,
    #[serde(default)]
    label: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EdgePatch {
    label: Option<String>,
}

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/edges", post(create_edge))
        .route("/edges/{id}", patch(patch_edge).delete(delete_edge))
}

async fn create_edge(
    State(pool): State<SqlitePool>,
    Json(b): Json<NewEdge>,
) -> Result<(StatusCode, Json<Edge>)> {
    let edge = sqlx::query_as(
        "INSERT INTO edge (parent_id, source_id, target_id, label) VALUES (?, ?, ?, ?) RETURNING *",
    )
    .bind(b.parent_id)
    .bind(b.source_id)
    .bind(b.target_id)
    .bind(b.label.trim())
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(edge)))
}

async fn patch_edge(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(p): Json<EdgePatch>,
) -> Result<Json<Edge>> {
    let edge = sqlx::query_as(
        "UPDATE edge SET label = COALESCE(?, label), updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ? RETURNING *",
    )
    .bind(p.label.as_deref().map(str::trim))
    .bind(id)
    .fetch_one(&pool)
    .await?;
    Ok(Json(edge))
}

async fn delete_edge(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    sqlx::query("DELETE FROM edge WHERE id = ? RETURNING id")
        .bind(id)
        .fetch_one(&pool)
        .await?; // no row -> RowNotFound -> 404
    Ok(StatusCode::NO_CONTENT)
}

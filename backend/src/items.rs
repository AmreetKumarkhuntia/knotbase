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
pub struct Item {
    id: i64,
    node_id: i64,
    text: String,
    done: bool,
    created_at: String,
    updated_at: String,
}
#[derive(Deserialize)]
struct NewItem {
    node_id: i64,
    text: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ItemPatch {
    text: Option<String>,
    done: Option<bool>,
}

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/items", post(create_item))
        .route("/items/{id}", patch(patch_item).delete(delete_item))
}

async fn create_item(
    State(pool): State<SqlitePool>,
    Json(b): Json<NewItem>,
) -> Result<(StatusCode, Json<Item>)> {
    let item = sqlx::query_as("INSERT INTO item (node_id, text) VALUES (?, ?) RETURNING *")
        .bind(b.node_id)
        .bind(b.text.trim())
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn patch_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(p): Json<ItemPatch>,
) -> Result<Json<Item>> {
    let item = sqlx::query_as(
        "UPDATE item SET text = COALESCE(?, text), done = COALESCE(?, done),
           updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ? RETURNING *",
    )
    .bind(p.text.as_deref().map(str::trim))
    .bind(p.done)
    .bind(id)
    .fetch_one(&pool)
    .await?;
    Ok(Json(item))
}

async fn delete_item(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    sqlx::query("DELETE FROM item WHERE id = ? RETURNING id")
        .bind(id)
        .fetch_one(&pool)
        .await?; // no row -> RowNotFound -> 404
    Ok(StatusCode::NO_CONTENT)
}

use crate::{
    error::Result,
    services::{db::Db, items},
    types::items::{Item, ItemPatch, NewItem},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{patch, post},
};

pub fn router() -> Router<Db> {
    Router::new()
        .route("/items", post(create_item))
        .route("/items/{id}", patch(patch_item).delete(delete_item))
}

async fn create_item(
    State(db): State<Db>,
    Json(b): Json<NewItem>,
) -> Result<(StatusCode, Json<Item>)> {
    Ok((StatusCode::CREATED, Json(items::create(&db, b).await?)))
}

async fn patch_item(
    State(db): State<Db>,
    Path(id): Path<i64>,
    Json(p): Json<ItemPatch>,
) -> Result<Json<Item>> {
    Ok(Json(items::patch(&db, id, p).await?))
}

async fn delete_item(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode> {
    items::delete(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

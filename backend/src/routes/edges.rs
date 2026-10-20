use crate::{
    error::Result,
    services::{db::Db, edges},
    types::edges::{Edge, EdgePatch, NewEdge},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{patch, post},
};

pub fn router() -> Router<Db> {
    Router::new()
        .route("/edges", post(create_edge))
        .route("/edges/{id}", patch(patch_edge).delete(delete_edge))
}

async fn create_edge(
    State(db): State<Db>,
    Json(b): Json<NewEdge>,
) -> Result<(StatusCode, Json<Edge>)> {
    Ok((StatusCode::CREATED, Json(edges::create(&db, b).await?)))
}

async fn patch_edge(
    State(db): State<Db>,
    Path(id): Path<i64>,
    Json(p): Json<EdgePatch>,
) -> Result<Json<Edge>> {
    Ok(Json(edges::patch(&db, id, p).await?))
}

async fn delete_edge(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode> {
    edges::delete(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

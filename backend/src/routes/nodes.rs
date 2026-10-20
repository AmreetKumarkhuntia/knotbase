use crate::{
    db::Db,
    error::Result,
    services::nodes::{self, Graph, NewNode, Node, NodeDetail, NodePatch},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};

pub fn router() -> Router<Db> {
    Router::new()
        .route("/nodes", post(create_node))
        .route(
            "/nodes/{id}",
            get(get_node).patch(patch_node).delete(delete_node),
        )
        .route("/nodes/{id}/graph", get(graph))
}

async fn graph(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<Graph>> {
    Ok(Json(nodes::graph(&db, id).await?))
}

async fn get_node(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<NodeDetail>> {
    Ok(Json(nodes::get(&db, id).await?))
}

async fn create_node(
    State(db): State<Db>,
    Json(b): Json<NewNode>,
) -> Result<(StatusCode, Json<Node>)> {
    Ok((StatusCode::CREATED, Json(nodes::create(&db, b).await?)))
}

async fn patch_node(
    State(db): State<Db>,
    Path(id): Path<i64>,
    Json(p): Json<NodePatch>,
) -> Result<Json<Node>> {
    Ok(Json(nodes::patch(&db, id, p).await?))
}

async fn delete_node(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode> {
    nodes::delete(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

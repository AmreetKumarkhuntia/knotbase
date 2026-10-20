// One test binary over the public crate: the helpers here, the tests by area in the sibling files.
mod app;
mod edges;
mod items;
mod nodes;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use knotbase::db::Db;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn app() -> Router {
    app_serving("/nonexistent").await // CI has no frontend/dist; only the SPA test needs a real dir
}

async fn app_serving(dist: &str) -> Router {
    let db = Db::connect("sqlite::memory:", 1).await.unwrap(); // one connection, or every checkout is a fresh empty DB
    knotbase::routes::app(db, dist)
}

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    let req = req
        .body(body.map_or_else(Body::empty, |b| Body::from(b.to_string())))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn node(parent: i64, name: &str) -> Option<Value> {
    Some(json!({"parent_id":parent,"name":name,"kind":"step"}))
}

// One test binary over the public crate: the helpers here, the tests by area in the sibling files.
mod edges;
mod http;
mod items;
mod nodes;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use knotbase::services::db::Db;
use serde_json::{Value, json};
use sqlx::{Postgres, migrate::MigrateDatabase};
use tower::ServiceExt;

async fn app() -> Router {
    app_serving("/nonexistent").await // CI has no frontend/dist; only the SPA test needs a real dir
}

async fn app_serving(dist: &str) -> Router {
    // TEST_DATABASE_URL=postgres://user/postgres runs this suite on Postgres: every test gets its own database,
    // knotbase_test_<test name>, created fresh here; the database in the URL is only logged into to create and drop.
    // shortcut: no teardown (libtest has none); a leftover is recreated on the next run and there is one per test.
    let url = match std::env::var("TEST_DATABASE_URL") {
        Ok(server) => {
            // libtest names each test thread after the test; Postgres cuts identifiers at 63 chars, so keep test names short
            let name: String = std::thread::current()
                .name()
                .unwrap_or("unnamed")
                .replace("::", "__")
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        c.to_ascii_lowercase()
                    } else {
                        '_'
                    }
                })
                .collect();
            let url = format!(
                "{}/knotbase_test_{name}",
                server.rsplit_once('/').unwrap().0
            );
            Postgres::force_drop_database(&url).await.unwrap();
            Postgres::create_database(&url).await.unwrap();
            url
        }
        Err(_) => "sqlite::memory:".into(), // a fresh in-memory database per test
    };
    knotbase::routes::app(Db::connect(&url, 1).await.unwrap(), dist)
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

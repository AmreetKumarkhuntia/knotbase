// Cross-cutting: the error shape, the host guard and the SPA fallback.
use crate::{app, app_serving, send};
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

// parent_id is not patchable: unknown PATCH fields are rejected, not silently ignored.
#[tokio::test]
async fn patch_rejects_unknown_fields() {
    let app = app().await;
    for (uri, body) in [
        ("/api/nodes/1", json!({"parent_id":5})),
        ("/api/edges/1", json!({"source_id":5})),
        ("/api/items/1", json!({"node_id":5})),
    ] {
        let (status, _) = send(&app, "PATCH", uri, Some(body)).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{uri}");
    }
}

// Every failure the UI can hit is JSON {"error"}: unknown route, missing id on all verbs, blank name, bad parent.
#[tokio::test]
async fn errors_are_json() {
    let app = app().await;
    let blank = json!({"parent_id":1,"name":" \t ","kind":"k"});
    let orphan = json!({"parent_id":99,"name":"a","kind":"k"});
    for (method, uri, body, want) in [
        ("GET", "/api/nope", None, StatusCode::NOT_FOUND),
        ("GET", "/api/nodes/999", None, StatusCode::NOT_FOUND),
        (
            "PATCH",
            "/api/nodes/999",
            Some(json!({"name":"x"})),
            StatusCode::NOT_FOUND,
        ),
        ("DELETE", "/api/nodes/999", None, StatusCode::NOT_FOUND),
        ("POST", "/api/nodes", Some(blank), StatusCode::BAD_REQUEST),
        ("POST", "/api/nodes", Some(orphan), StatusCode::BAD_REQUEST),
    ] {
        let (status, v) = send(&app, method, uri, body).await;
        assert_eq!(status, want, "{method} {uri}");
        assert!(
            v["error"].is_string(),
            "{method} {uri} body was not {{\"error\"}} JSON"
        );
    }
}

// A rebound page keeps its own hostname in Host; only loopback names (any port) get through.
#[tokio::test]
async fn host_guard_allows_only_loopback() {
    let app = app().await;
    for (host, want) in [
        ("evil.example:3000", StatusCode::FORBIDDEN),
        ("evil.example", StatusCode::FORBIDDEN),
        ("localhost.evil.example", StatusCode::FORBIDDEN),
        ("localhost:5025", StatusCode::OK),
        ("localhost", StatusCode::OK),
        ("127.0.0.1:3000", StatusCode::OK),
        ("[::1]:5025", StatusCode::OK),
        ("[::1]", StatusCode::OK),
    ] {
        let req = Request::get("/api/nodes/1/graph")
            .header("host", host)
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), want, "{host}");
    }
}

// Reload on /node/6 must get index.html with 200 (not_found_service would make it a 404).
#[tokio::test]
async fn deep_link_gets_index_html_with_200() {
    let dir = std::env::temp_dir().join(format!("knotbase-spa-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("index.html"), "<p>spa</p>").unwrap();
    let app = app_serving(dir.to_str().unwrap()).await;
    let req = Request::get("/node/6").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(res.into_body(), usize::MAX).await.unwrap(),
        "<p>spa</p>"
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

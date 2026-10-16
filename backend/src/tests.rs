use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn app() -> Router {
    app_serving("/nonexistent").await // CI has no frontend/dist; only the SPA test needs a real dir
}

async fn app_serving(dist: &str) -> Router {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    crate::app(pool, dist)
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

#[tokio::test]
async fn post_child_then_graph_and_breadcrumb() {
    let app = app().await;
    let (status, child) = send(&app, "POST", "/api/nodes", node(1, "Backend")).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, v) = send(&app, "GET", "/api/nodes/1/graph", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(v["nodes"][0]["name"], "Backend");
    let (_, v) = send(&app, "GET", &format!("/api/nodes/{}", child["id"]), None).await;
    assert_eq!(
        v["breadcrumb"],
        json!([{"id":1,"name":"System"},{"id":child["id"],"name":"Backend"}])
    );
}

// The canvas never reads child docs, so /graph blanks them; the node's own GET keeps the doc.
#[tokio::test]
async fn graph_blanks_child_docs() {
    let app = app().await;
    let (_, n) = send(&app, "POST", "/api/nodes", node(1, "A")).await;
    let uri = format!("/api/nodes/{}", n["id"]);
    let (status, _) = send(&app, "PATCH", &uri, Some(json!({"doc":"D"}))).await;
    assert_eq!(status, StatusCode::OK);
    let (_, g) = send(&app, "GET", "/api/nodes/1/graph", None).await;
    assert_eq!(g["nodes"][0]["doc"], "");
    let (_, v) = send(&app, "GET", &uri, None).await;
    assert_eq!(v["doc"], "D");
}

#[tokio::test]
async fn delete_root_is_400() {
    let (status, _) = send(&app().await, "DELETE", "/api/nodes/1", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// Both endpoints must be siblings: the deeper node as target and as source.
#[tokio::test]
async fn edge_across_levels_is_400() {
    let app = app().await;
    let (s, _) = send(&app, "POST", "/api/nodes", node(1, "A")).await; // id 2
    assert_eq!(s, StatusCode::CREATED);
    let (s, _) = send(&app, "POST", "/api/nodes", node(2, "Inner")).await; // id 3, child of 2
    assert_eq!(s, StatusCode::CREATED);
    for (source_id, target_id) in [(2, 3), (3, 2)] {
        let edge = json!({"parent_id":1,"source_id":source_id,"target_id":target_id});
        let (status, v) = send(&app, "POST", "/api/edges", Some(edge)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{source_id}->{target_id}");
        assert_eq!(v["error"], "FOREIGN KEY constraint failed");
    }
}

// 0 and "" are values, not NULL, so COALESCE must apply them; null and omitted fields must not.
#[tokio::test]
async fn patch_node_is_partial() {
    let app = app().await;
    let (_, n) = send(&app, "POST", "/api/nodes", node(1, "A")).await;
    let uri = format!("/api/nodes/{}", n["id"]);
    let set = json!({"doc":"D","x":5.0,"y":6.0});
    assert_eq!(send(&app, "PATCH", &uri, Some(set)).await.0, StatusCode::OK);
    let (status, v) = send(&app, "PATCH", &uri, Some(json!({"name":" N ","kind":null}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        (&v["name"], &v["kind"], &v["doc"], &v["x"], &v["y"]),
        (
            &json!("N"),
            &json!("step"),
            &json!("D"),
            &json!(5.0),
            &json!(6.0)
        )
    );
    let (_, v) = send(&app, "PATCH", &uri, Some(json!({"x":0.0,"doc":""}))).await;
    assert_eq!(
        (&v["x"], &v["doc"], &v["y"]),
        (&json!(0.0), &json!(""), &json!(6.0))
    );
}

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

// Deleting a node removes its whole subtree: children, their edges and items.
#[tokio::test]
async fn delete_cascades_subtree() {
    let app = app().await;
    for (parent, name) in [(1, "A"), (2, "B"), (2, "C")] {
        // ids 2, 3 and 4: B and C sit inside A
        let (s, _) = send(&app, "POST", "/api/nodes", node(parent, name)).await;
        assert_eq!(s, StatusCode::CREATED);
    }
    let edge = json!({"parent_id":2,"source_id":3,"target_id":4});
    let (s, e) = send(&app, "POST", "/api/edges", Some(edge)).await;
    assert_eq!(s, StatusCode::CREATED);
    let item = json!({"node_id":3,"text":"t"});
    let (s, i) = send(&app, "POST", "/api/items", Some(item)).await;
    assert_eq!(s, StatusCode::CREATED);
    let (status, _) = send(&app, "DELETE", "/api/nodes/2", None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    for (method, uri) in [
        ("GET", "/api/nodes/3".to_string()),
        ("GET", "/api/nodes/4".to_string()),
        ("DELETE", format!("/api/edges/{}", e["id"])),
        ("DELETE", format!("/api/items/{}", i["id"])),
    ] {
        let (status, _) = send(&app, method, &uri, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
    }
}

// Create, trim, done round trip, list, 400, delete, 404.
#[tokio::test]
async fn item_lifecycle() {
    let app = app().await;
    let new = json!({"node_id":1,"text":" t "});
    let (status, i) = send(&app, "POST", "/api/items", Some(new)).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!((&i["text"], &i["done"]), (&json!("t"), &json!(false)));
    let uri = format!("/api/items/{}", i["id"]);
    let (status, v) = send(&app, "PATCH", &uri, Some(json!({"done":true}))).await;
    assert_eq!(
        (status, &v["done"], &v["text"]),
        (StatusCode::OK, &json!(true), &json!("t"))
    );
    let (_, v) = send(&app, "GET", "/api/nodes/1", None).await;
    assert_eq!(v["items"][0]["done"], true);
    let (status, _) = send(&app, "PATCH", &uri, Some(json!({"text":" "}))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        send(&app, "DELETE", &uri, None).await.0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        send(&app, "DELETE", &uri, None).await.0,
        StatusCode::NOT_FOUND
    );
}

// Edge create (the first insert also proves the migration's UNIQUE (parent_id, id)), PATCH, and the graph returns only its own level's edges.
#[tokio::test]
async fn edge_patch_and_graph_edges() {
    let app = app().await;
    for (parent, name) in [(1, "A"), (1, "B"), (2, "Inner")] {
        let (s, _) = send(&app, "POST", "/api/nodes", node(parent, name)).await;
        assert_eq!(s, StatusCode::CREATED);
    }
    let edge = json!({"parent_id":1,"source_id":2,"target_id":3,"label":" l "});
    let (status, e) = send(&app, "POST", "/api/edges", Some(edge)).await;
    assert_eq!((status, &e["label"]), (StatusCode::CREATED, &json!("l")));
    let uri = format!("/api/edges/{}", e["id"]);
    let (status, v) = send(&app, "PATCH", &uri, Some(json!({"label":" go "}))).await;
    assert_eq!((status, &v["label"]), (StatusCode::OK, &json!("go")));
    let (_, g) = send(&app, "GET", "/api/nodes/1/graph", None).await;
    assert_eq!(g["edges"][0]["label"], "go");
    let (_, g) = send(&app, "GET", "/api/nodes/2/graph", None).await;
    assert_eq!(g["edges"], json!([]));
    assert_eq!(
        send(&app, "DELETE", &uri, None).await.0,
        StatusCode::NO_CONTENT
    );
    let (status, _) = send(&app, "PATCH", &uri, Some(json!({"label":"x"}))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

// The recursive CTE must climb every level, not just one.
#[tokio::test]
async fn breadcrumb_climbs_every_level() {
    let app = app().await;
    let mut parent = 1;
    for name in ["API", "Payment Flow", "Charge"] {
        let (status, n) = send(&app, "POST", "/api/nodes", node(parent, name)).await;
        assert_eq!(status, StatusCode::CREATED);
        parent = n["id"].as_i64().unwrap();
    }
    let (_, v) = send(&app, "GET", &format!("/api/nodes/{parent}"), None).await;
    let names: Vec<_> = v["breadcrumb"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| &c["name"])
        .collect();
    assert_eq!(names, ["System", "API", "Payment Flow", "Charge"]);
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
        ("localhost:5173", StatusCode::OK),
        ("localhost", StatusCode::OK),
        ("127.0.0.1:3000", StatusCode::OK),
        ("[::1]:5173", StatusCode::OK),
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

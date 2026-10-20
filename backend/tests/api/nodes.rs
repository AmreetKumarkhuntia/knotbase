use crate::{app, node, send};
use axum::http::StatusCode;
use serde_json::json;

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

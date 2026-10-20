use crate::{app, node, send};
use axum::http::StatusCode;
use serde_json::json;

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
        let msg = v["error"].as_str().unwrap().to_lowercase(); // SQLite: "FOREIGN KEY constraint failed"; Postgres names the constraint
        assert!(msg.contains("foreign key"), "{msg}");
    }
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

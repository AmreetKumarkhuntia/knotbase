use crate::{app, send};
use axum::http::StatusCode;
use serde_json::json;

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

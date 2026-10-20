mod edges;
mod items;
mod nodes;

use crate::{error::AppError, services::db::Db};
use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header::HOST},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use tower_http::services::{ServeDir, ServeFile};

// DNS-rebinding guard: a rebound page still sends its own hostname in Host. No Host (HTTP/2, tests' oneshot) passes.
async fn loopback_host(req: Request, next: Next) -> Response {
    let ok = req.headers().get(HOST).is_none_or(|h| {
        let h = h.to_str().unwrap_or_default();
        // strip a numeric port only: `[::1]:5025` -> `[::1]`, bare `[::1]` stays whole
        let name = h
            .rsplit_once(':')
            .filter(|(_, port)| port.bytes().all(|b| b.is_ascii_digit()))
            .map_or(h, |(name, _)| name);
        matches!(name, "localhost" | "127.0.0.1" | "[::1]")
    });
    if ok {
        next.run(req).await
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

// The whole app: /api from the resource routers, everything else the built UI with index.html for deep links.
// shortcut: axum extractor rejections are plain text, not {"error"} (malformed JSON 400, missing/mistyped field 422, wrong content-type 415, /api/nodes/abc 400, wrong verb on a known path empty 405, body over 2 MiB 413); wrap Json/Path in a FromRequest that maps to AppError when a client must parse them.
pub fn app(db: Db, dist: &str) -> Router {
    let index = format!("{dist}/index.html");
    if !std::path::Path::new(&index).exists() {
        eprintln!(
            "warning: {index} missing; UI not served on this port (fine in dev, Vite serves it on :5025)"
        );
    }
    let api = Router::new()
        .merge(nodes::router())
        .merge(edges::router())
        .merge(items::router())
        .fallback(|| async { AppError::NotFound }); // unknown /api/* stays JSON 404, not index.html (only the bare `/api/` falls through to the UI)
    Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(dist).fallback(ServeFile::new(index)))
        .with_state(db)
        .layer(middleware::from_fn(loopback_host))
}

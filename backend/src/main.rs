mod edges;
mod error;
mod items;
mod nodes;
#[cfg(test)]
mod tests;

use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header::HOST},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use error::AppError;
use sqlx::SqlitePool;
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

pub fn app(pool: SqlitePool, dist: &str) -> Router {
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
        .fallback(|| async { AppError::NotFound }); // unknown /api/* stays JSON 404, not index.html
    Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(dist).fallback(ServeFile::new(index)))
        .with_state(pool)
        .layer(middleware::from_fn(loopback_host))
}

#[tokio::main]
async fn main() {
    let dist = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist").to_string());
    // shortcut: paths are baked at compile time, so a release binary copied elsewhere points at the build box's paths; set STATIC_DIR/DATABASE_URL explicitly when deploying off this laptop.
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("sqlite:{}/knotbase.db?mode=rwc", env!("CARGO_MANIFEST_DIR")));
    let pool = SqlitePool::connect(&url).await.expect("open db");
    sqlx::migrate!().run(&pool).await.expect("migrate");
    // shortcut: loopback only, single user; switch to 0.0.0.0 / BIND env when it runs in a container
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app(pool, &dist)).await.unwrap();
}

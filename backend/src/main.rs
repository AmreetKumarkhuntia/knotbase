use knotbase::{db::Db, routes};

#[tokio::main]
async fn main() {
    let dist = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist").to_string());
    // shortcut: paths are baked at compile time, so a release binary copied elsewhere points at the build box's paths; set STATIC_DIR/DATABASE_URL explicitly when deploying off this laptop.
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("sqlite:{}/knotbase.db?mode=rwc", env!("CARGO_MANIFEST_DIR")));
    let db = Db::connect(&url, 10).await.expect("open db");
    // shortcut: loopback only, single user; switch to 0.0.0.0 / BIND env when it runs in a container
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, routes::app(db, &dist)).await.unwrap();
}

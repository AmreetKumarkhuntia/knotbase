use knotbase::{config::Config, routes, services::db::Db};

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let db = Db::connect(&cfg.database_url, 10).await.expect("open db");
    let listener = tokio::net::TcpListener::bind(&cfg.bind).await.unwrap();
    println!("listening on http://{}", cfg.bind);
    axum::serve(listener, routes::app(db, &cfg.static_dir))
        .await
        .unwrap();
}

// Every setting the binary takes, from the environment or a constant, with its default, in one place.
pub struct Config {
    pub static_dir: String,
    pub database_url: String,
    pub bind: String,
    pub pool_size: u32,
}

impl Config {
    pub fn from_env() -> Config {
        // shortcut: the defaults are paths baked at compile time, so a release binary copied elsewhere points at the build box's paths; set STATIC_DIR/DATABASE_URL explicitly when deploying off this laptop.
        let port: u16 =
            std::env::var("PORT").map_or(3000, |p| p.parse().expect("PORT must be a number"));
        Config {
            static_dir: std::env::var("STATIC_DIR").unwrap_or_else(|_| {
                concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist").to_string()
            }),
            // the URL scheme picks the engine: sqlite: (default) or postgres:
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                format!("sqlite:{}/knotbase.db?mode=rwc", env!("CARGO_MANIFEST_DIR"))
            }),
            // shortcut: loopback only, single user; add HOST here and widen the Host allowlist in routes/mod.rs together when it runs in a container
            bind: format!("127.0.0.1:{port}"),
            pool_size: 10, // shortcut: a constant; read DB_POOL_SIZE here when a deployment needs it
        }
    }
}

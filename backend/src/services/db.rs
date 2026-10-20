use sqlx::{AssertSqlSafe, PgPool, SqlitePool, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};

// The one database interface: a variant per engine with its native pool inside. DATABASE_URL's scheme picks the variant,
// and each variant runs its own migrations directory (same versions, same schema, that engine's types).
// Adding an engine: its sqlx feature, a variant, an arm in connect(), now() and sql!, and migrations/<engine>/.
#[derive(Clone)]
pub enum Db {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

impl Db {
    // shortcut: no TLS feature, fine for a local Postgres; add sqlx's tls-rustls when the server is remote.
    pub async fn connect(url: &str, max_connections: u32) -> sqlx::Result<Db> {
        Ok(
            if url.starts_with("postgres:") || url.starts_with("postgresql:") {
                let pool = PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(url)
                    .await?;
                sqlx::migrate!("./migrations/postgres").run(&pool).await?;
                Db::Postgres(pool)
            } else if url.starts_with("sqlite:") {
                let pool = SqlitePoolOptions::new()
                    .max_connections(max_connections)
                    .connect(url)
                    .await?;
                sqlx::migrate!("./migrations/sqlite").run(&pool).await?;
                Db::Sqlite(pool)
            } else {
                // sqlx's SQLite driver takes anything else as a file name (an empty URL is a temporary database), so refuse
                // it here; the message never echoes the URL, which may hold a password
                return Err(sqlx::Error::Configuration(
                    "DATABASE_URL must start with sqlite: or postgres:".into(),
                ));
            },
        )
    }

    // A service query with `{now}` filled in: the only dynamic SQL in the crate, built from a literal and the constant below.
    pub(crate) fn with_now(&self, sql: &'static str) -> AssertSqlSafe<String> {
        AssertSqlSafe(sql.replace("{now}", self.now()))
    }

    // The one engine-specific fragment: the current UTC time as the ISO text the schema stores.
    fn now(&self) -> &'static str {
        match self {
            Db::Sqlite(_) => "strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
            Db::Postgres(_) => {
                "to_char(now() AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"')"
            }
        }
    }
}

// Runs one sqlx expression against whichever pool `db` holds: `pool` is the concrete pool in each arm, so native types decode.
// The SQL inside must be portable: `$n` placeholders, RETURNING, `{now}` through `db.with_now()` for the time, nothing else engine-specific.
// The expression (a chain, or a block with a transaction) must end in an engine-independent type: a row struct, a scalar, `()`, never a driver row.
macro_rules! sql {
    ($db:expr, |$pool:ident| $q:expr) => {
        match $db {
            $crate::services::db::Db::Sqlite($pool) => $q,
            $crate::services::db::Db::Postgres($pool) => $q,
        }
    };
}
pub(crate) use sql;

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
            } else {
                // another scheme fails inside sqlx with its own "unsupported scheme" error
                let pool = SqlitePoolOptions::new()
                    .max_connections(max_connections)
                    .connect(url)
                    .await?;
                sqlx::migrate!("./migrations/sqlite").run(&pool).await?;
                Db::Sqlite(pool)
            },
        )
    }

    // A service query with `{now}` filled in: the only dynamic SQL in the crate, and the only thing spliced is the constant below.
    pub fn with_now(&self, sql: &str) -> AssertSqlSafe<String> {
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

// Runs one sqlx chain against whichever pool `db` holds: `$p` is the concrete pool in each arm, so native types decode.
// The SQL inside must be portable: `$n` placeholders in ascending order, RETURNING, `{now}` through `db.with_now()` for the time, nothing else engine-specific.
// The chain must end in an engine-independent type (a row struct, a scalar, `()`), never a driver row or query result.
macro_rules! sql {
    ($db:expr, $p:ident => $q:expr) => {
        match $db {
            $crate::services::db::Db::Sqlite($p) => $q,
            $crate::services::db::Db::Postgres($p) => $q,
        }
    };
}
pub(crate) use sql;

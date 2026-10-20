use crate::{
    db::{Db, sql},
    error::Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Edge {
    id: i64,
    parent_id: i64,
    source_id: i64,
    target_id: i64,
    label: String,
    created_at: String,
    updated_at: String,
}
#[derive(Deserialize)]
pub struct NewEdge {
    parent_id: i64,
    source_id: i64,
    target_id: i64,
    #[serde(default)]
    label: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgePatch {
    label: Option<String>,
}

pub async fn create(db: &Db, b: NewEdge) -> Result<Edge> {
    Ok(sql!(db, p => sqlx::query_as(
        "INSERT INTO edge (parent_id, source_id, target_id, label) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(b.parent_id)
    .bind(b.source_id)
    .bind(b.target_id)
    .bind(b.label.trim())
    .fetch_one(p)
    .await)?)
}

pub async fn patch(db: &Db, id: i64, p: EdgePatch) -> Result<Edge> {
    Ok(
        sql!(db, pool => sqlx::query_as(db.with_now("UPDATE edge SET label = COALESCE($1, label), updated_at = {now} WHERE id = $2 RETURNING *"))
        .bind(p.label.as_deref().map(str::trim))
        .bind(id)
        .fetch_one(pool)
        .await)?,
    )
}

pub async fn delete(db: &Db, id: i64) -> Result<()> {
    sql!(db, p => sqlx::query("DELETE FROM edge WHERE id = $1 RETURNING id")
        .bind(id)
        .fetch_one(p)
        .await)?; // no row -> RowNotFound -> 404
    Ok(())
}

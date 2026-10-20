use crate::{
    error::Result,
    services::db::{Db, sql},
    types::edges::{Edge, EdgePatch, NewEdge},
};

pub async fn create(db: &Db, b: NewEdge) -> Result<Edge> {
    Ok(sql!(db, |pool| {
        sqlx::query_as(
        "INSERT INTO edge (parent_id, source_id, target_id, label) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(b.parent_id)
    .bind(b.source_id)
    .bind(b.target_id)
    .bind(b.label.trim())
    .fetch_one(pool)
    .await
    })?)
}

pub async fn patch(db: &Db, id: i64, p: EdgePatch) -> Result<Edge> {
    Ok(sql!(db, |pool| sqlx::query_as(db.with_now(
        "UPDATE edge SET label = COALESCE($1, label), updated_at = {now} WHERE id = $2 RETURNING *"
    ))
    .bind(p.label.as_deref().map(str::trim))
    .bind(id)
    .fetch_one(pool)
    .await)?)
}

pub async fn delete(db: &Db, id: i64) -> Result<()> {
    sql!(db, |pool| sqlx::query(
        "DELETE FROM edge WHERE id = $1 RETURNING id"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map(|_| ()))?; // no row -> RowNotFound -> 404; the raw row differs per engine, so drop it inside the arm
    Ok(())
}

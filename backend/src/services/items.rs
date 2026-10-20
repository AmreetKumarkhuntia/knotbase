use crate::{
    error::Result,
    services::db::{Db, sql},
    types::items::{Item, ItemPatch, NewItem},
};

pub async fn create(db: &Db, b: NewItem) -> Result<Item> {
    Ok(sql!(db, |pool| sqlx::query_as(
        "INSERT INTO item (node_id, text) VALUES ($1, $2) RETURNING *"
    )
    .bind(b.node_id)
    .bind(b.text.trim())
    .fetch_one(pool)
    .await)?)
}

pub async fn patch(db: &Db, id: i64, p: ItemPatch) -> Result<Item> {
    Ok(sql!(db, |pool| sqlx::query_as(db.with_now(
        "UPDATE item SET text = COALESCE($1, text), done = COALESCE($2, done), updated_at = {now}
         WHERE id = $3 RETURNING *"
    ))
    .bind(p.text.as_deref().map(str::trim))
    .bind(p.done)
    .bind(id)
    .fetch_one(pool)
    .await)?)
}

pub async fn delete(db: &Db, id: i64) -> Result<()> {
    sql!(db, |pool| sqlx::query(
        "DELETE FROM item WHERE id = $1 RETURNING id"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map(|_| ()))?; // no row -> RowNotFound -> 404; the raw row differs per engine, so drop it inside the arm
    Ok(())
}

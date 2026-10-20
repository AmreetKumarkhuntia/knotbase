use crate::{
    error::{AppError, Result},
    services::db::{Db, sql},
    types::nodes::{Graph, NewNode, Node, NodeDetail, NodePatch},
};

// shortcut: a missing id returns 200 with empty lists (the frontend's node fetch supplies the 404); add an existence check if graph is ever called alone.
pub async fn graph(db: &Db, id: i64) -> Result<Graph> {
    // '' AS doc: the canvas never reads child docs, and they made the response megabytes
    let nodes = sql!(db, |pool| {
        sqlx::query_as(
        "SELECT id, parent_id, name, kind, '' AS doc, x, y, created_at, updated_at FROM node WHERE parent_id = $1 ORDER BY id",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    })?;
    let edges = sql!(db, |pool| sqlx::query_as(
        "SELECT * FROM edge WHERE parent_id = $1 ORDER BY id"
    )
    .bind(id)
    .fetch_all(pool)
    .await)?;
    Ok(Graph { nodes, edges })
}

pub async fn get(db: &Db, id: i64) -> Result<NodeDetail> {
    let node = sql!(db, |pool| sqlx::query_as(
        "SELECT * FROM node WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await)?;
    // shortcut: items are id order; add an INTEGER position column (one migration per engine) when drag-reorder is wanted.
    let items = sql!(db, |pool| sqlx::query_as(
        "SELECT * FROM item WHERE node_id = $1 ORDER BY id"
    )
    .bind(id)
    .fetch_all(pool)
    .await)?;
    let breadcrumb = sql!(db, |pool| sqlx::query_as(
        "WITH RECURSIVE up(id, parent_id, name, depth) AS (
           SELECT id, parent_id, name, 0 FROM node WHERE id = $1
           UNION ALL
           SELECT n.id, n.parent_id, n.name, up.depth + 1 FROM node n JOIN up ON n.id = up.parent_id
         )
         SELECT id, name FROM up ORDER BY depth DESC",
    )
    .bind(id)
    .fetch_all(pool)
    .await)?;
    Ok(NodeDetail {
        node,
        items,
        breadcrumb,
    })
}

// shortcut: no pre-check SELECT to turn a bad parent_id into 404, add if the UI needs to distinguish.
pub async fn create(db: &Db, b: NewNode) -> Result<Node> {
    Ok(sql!(db, |pool| sqlx::query_as(
        "INSERT INTO node (parent_id, name, kind, x, y) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(b.parent_id)
    .bind(b.name.trim())
    .bind(b.kind.trim())
    .bind(b.x)
    .bind(b.y)
    .fetch_one(pool)
    .await)?)
}

// shortcut: parent_id is not patchable and sending it is a 422 (no move between graphs; the edge FKs are ON UPDATE NO ACTION so a move with edges would fail anyway); add a move endpoint that deletes the node's edges first when drag-to-reparent is wanted, with a cycle guard (the breadcrumb CTE would loop on a cycle) and id 1 must never move.
pub async fn patch(db: &Db, id: i64, p: NodePatch) -> Result<Node> {
    Ok(sql!(db, |pool| {
        sqlx::query_as(db.with_now("UPDATE node SET name = COALESCE($1, name), kind = COALESCE($2, kind), doc = COALESCE($3, doc),
           x = COALESCE($4, x), y = COALESCE($5, y), updated_at = {now}
         WHERE id = $6 RETURNING *"))
        .bind(p.name.as_deref().map(str::trim))
        .bind(p.kind.as_deref().map(str::trim))
        .bind(p.doc)
        .bind(p.x)
        .bind(p.y)
        .bind(id)
        .fetch_one(pool)
        .await
    })?)
}

pub async fn delete(db: &Db, id: i64) -> Result<()> {
    if id == 1 {
        return Err(AppError::BadRequest("root cannot be deleted".into())); // only the migration can create it
    }
    // shortcut: item.node_id and edge(parent_id, source_id|target_id) have no index, so a cascade delete scans both tables (2.3 s at ~11k nodes on SQLite vs 13 ms indexed); add those three indexes, one migration per engine, at ~10k rows.
    // shortcut: a subtree deeper than ~1000 levels fails with 500 on SQLite (trigger depth limit, rolled back, nothing lost); delete bottom-up if that ever matters.
    sql!(db, |pool| sqlx::query(
        "DELETE FROM node WHERE id = $1 RETURNING id"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map(|_| ()))?; // no row -> RowNotFound -> 404; the raw row differs per engine, so drop it inside the arm
    Ok(())
}

# knotbase — Implementation Plan (v2: recursive model, Svelte 5)

> **Status: built and verified.** Written before the code. Where it disagrees with backend/ and frontend/, the code wins; CLAUDE.md is the current summary. Known drifts: `req()` in `api.ts` (unwraps `{"error"}`), `NodePage` `reload()`/`act()` (sequence guard, refetch on failure, sequential multi-delete, one reload per drag), the seed (`.bail on` plus a transaction), `backend/tests/api/` (`send()` helper, many more tests), the backend layout (`lib.rs`, `src/routes/`, `src/services/`, `src/config.rs`, `migrations/<engine>/`, `Db` over SQLite or Postgres instead of `SqlitePool`; see CLAUDE.md ", `app(pool, dist)` with a Host guard, `/graph` returns `doc` `""`, PATCH rejects unknown fields, `CardNode` has an open link.

## Context

`docs/Initial-plan.md` describes a graph-first system exploration and documentation tool with lightweight work tracking: a system map of applications, each with internal flows, each flow with markdown docs and a checklist of items.

The first version of this plan (v1, in `docs/Implementation-plan.md`) hard-coded two levels: node/edge for the system map, then flow/flow_node/flow_edge inside a node. The user then asked for nesting to any depth: a flow contains nodes, a flow can sit inside another flow, and so on. v2 replaces the two fixed levels with **one recursive `node` table**: everything is a node (the root "System", a service, a flow, a step), a node's children are its graph, and a node carries its own markdown `doc` and `items`. The frontend becomes **Svelte 5 + `@xyflow/svelte`** per the user's choice. The backend stack, single-port topology and tooling from v1 are unchanged.

The repo had no code when this was written (no commits), so nothing is migrated; this is the plan to build from. Ponytail (lazy senior dev) mode applies: smallest complete version, no speculative abstractions, limits marked in code as `shortcut: <limit>, <when to upgrade>`.

## Fixed decisions

- **Layout**: monorepo, `backend/` (Rust, Cargo conventions) and `frontend/` (Vite/Svelte conventions). Different structures by design. No root workspace.
- **Backend**: Rust, Axum 0.8, tokio, sqlx 0.9 + bundled SQLite (`sqlx::migrate!`), serde. One binary. No auth, single user.
- **Frontend**: Svelte 5 (runes) + TypeScript + Vite 8, plain Vite (no SvelteKit), `@xyflow/svelte` for the graph canvas.
- **Single port**: dev = Vite on 5173 proxies `/api/*` to Rust on 3000; prod = Rust on 3000 serves `frontend/dist` with SPA fallback and mounts `/api/*`. No separate proxy process, no CORS.
- **Data model**: one recursive `node` table (id 1 = root "System", every other node has a parent), `edge` linking siblings only, `item` attached to any node. A "flow" is a node whose children are its steps; a step may have children too. `kind` is free text with UI suggestions.
- **UI**: one page for every level at `/node/:id` (`/` = `/node/1`): breadcrumb, header, canvas of children, doc panel, items.

## 1. Data model & API contract

**Decisions**
- Ids: `INTEGER PRIMARY KEY AUTOINCREMENT` (no id reuse after delete, which would resurrect stale client state).
- Timestamps: `created_at`/`updated_at` TEXT ISO-8601 UTC via `DEFAULT (strftime(...))`; every PATCH sets `updated_at`. No chrono.
- Positions `REAL NOT NULL DEFAULT 0` (keeps `f64` in Rust and COALESCE PATCH uniform).
- No nullable text: `label`, `doc` are `NOT NULL DEFAULT ''`.
- The schema is the validator (CHECK + FK). Rust maps a constraint violation to 400; no hand-written validators. Foreign keys must stay on (sqlx sets `foreign_keys=ON` per connection; never disable it or cascades silently stop).
- Siblings-only edges are enforced by a composite FK `(parent_id, source_id) REFERENCES node(parent_id, id)`.

**`backend/migrations/0001_init.sql`** (verified on local sqlite3 3.51 in-memory with this exact DDL)
```sql
-- One recursive tree. Row 1 is the root ("System"); every other row has a parent.
CREATE TABLE node (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_id  INTEGER REFERENCES node(id) ON DELETE CASCADE,
  name       TEXT NOT NULL CHECK (trim(name) <> ''),
  kind       TEXT NOT NULL CHECK (trim(kind) <> ''),
  doc        TEXT NOT NULL DEFAULT '',
  x          REAL NOT NULL DEFAULT 0,
  y          REAL NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  CHECK (parent_id IS NOT NULL OR id = 1),
  UNIQUE (parent_id, id)  -- parent key for edge's pair FKs; also the covering index behind WHERE parent_id = ?
);
CREATE TABLE edge (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_id  INTEGER NOT NULL,
  source_id  INTEGER NOT NULL,
  target_id  INTEGER NOT NULL,
  label      TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  CHECK (source_id <> target_id),
  -- pair FKs: endpoints must exist AND be children of parent_id, i.e. siblings
  FOREIGN KEY (parent_id, source_id) REFERENCES node(parent_id, id) ON DELETE CASCADE,
  FOREIGN KEY (parent_id, target_id) REFERENCES node(parent_id, id) ON DELETE CASCADE
);
CREATE TABLE item (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  node_id    INTEGER NOT NULL REFERENCES node(id) ON DELETE CASCADE,
  text       TEXT NOT NULL CHECK (trim(text) <> ''),
  done       INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
INSERT INTO node (id, parent_id, name, kind) VALUES (1, NULL, 'System', 'root');
```
SQLite facts this leans on (all verified):
- The composite FK needs `UNIQUE (parent_id, id)`. Without it `CREATE TABLE` succeeds but the first edge insert or node delete fails with `foreign key mismatch` (an `Other` error → 500, and the node-only smoke test would not catch it; the edge test does). Keep the `(parent_id, id)` order: `EXPLAIN QUERY PLAN` shows it gives a covering index search for the graph query, reversed it does not.
- Root's NULL `parent_id` is fine in the unique index, and because `edge.parent_id` is NOT NULL no edge can ever have root as an endpoint.
- Self-FK `ON DELETE CASCADE` recurses through the subtree with `foreign_keys=ON` (tested at depth 3: node → flow → steps, edges at both levels, items).
- `CHECK (parent_id IS NOT NULL OR id = 1)` with AUTOINCREMENT: the explicit `id = 1` seed passes and sets `sqlite_sequence`; after a root delete nothing could ever pass the CHECK again, so **the DELETE handler refuses id 1** (Rust one-liner; a trigger's `RAISE(ABORT)` maps to `ErrorKind::Other` → 500 instead of 400).
- `DEFAULT strftime(...)` without parentheses is a syntax error; sqlx runs the whole file in one transaction, so multi-statement is fine.
- `shortcut: no index on edge.parent_id; add CREATE INDEX when a level has thousands of edges.`

**JSON shapes** (snake_case, ids are numbers, lists ordered `id ASC`)
- `Node {id, parent_id: number|null, name, kind, doc, x, y, created_at, updated_at}` (`parent_id` null only for id 1)
- `Edge {id, parent_id, source_id, target_id, label, created_at, updated_at}`
- `Item {id, node_id, text, done: boolean, created_at, updated_at}`
- `Graph {nodes: Node[], edges: Edge[]}`
- `NodeDetail = Node & {items: Item[], breadcrumb: {id, name}[]}`, root first, self last; for id 1 it is `[{id:1,name:"System"}]`. No `children_count` (the page fetches `/graph` alongside).

**Endpoints** (11 routes; axum 0.8 path syntax is `{id}`)

| Method | Path | Body | 2xx | Codes |
|---|---|---|---|---|
| GET | `/api/nodes/{id}/graph` | – | Graph | 200 |
| GET | `/api/nodes/{id}` | – | NodeDetail | 200 404 |
| POST | `/api/nodes` | `{parent_id, name, kind, x?, y?}` | Node | 201 400 |
| PATCH | `/api/nodes/{id}` | `{name?, kind?, doc?, x?, y?}` | Node | 200 400 404 |
| DELETE | `/api/nodes/{id}` | – | empty (subtree cascades) | 204 400 (id 1) 404 |
| POST | `/api/edges` | `{parent_id, source_id, target_id, label?}` | Edge | 201 400 |
| PATCH | `/api/edges/{id}` | `{label?}` | Edge | 200 400 404 |
| DELETE | `/api/edges/{id}` | – | empty | 204 404 |
| POST | `/api/items` | `{node_id, text}` | Item | 201 400 |
| PATCH | `/api/items/{id}` | `{text?, done?}` | Item | 200 400 404 |
| DELETE | `/api/items/{id}` | – | empty | 204 404 |

`shortcut: parent_id is not patchable (no move between graphs; the edge FKs are ON UPDATE NO ACTION so a move with edges would fail anyway); add a move endpoint that deletes the node's edges first when drag-to-reparent is wanted.` `shortcut: GET /graph on a missing id returns 200 {nodes:[],edges:[]}; the page's sibling GET /api/nodes/{id} supplies the 404.`

**SQL**
```sql
-- graph (two statements, same bind)
SELECT * FROM node WHERE parent_id = ? ORDER BY id;
SELECT * FROM edge WHERE parent_id = ? ORDER BY id;

-- breadcrumb (verified: for a depth-3 step returns System, API, Payment Flow, Charge)
WITH RECURSIVE up(id, parent_id, name, depth) AS (
  SELECT id, parent_id, name, 0 FROM node WHERE id = ?
  UNION ALL
  SELECT n.id, n.parent_id, n.name, up.depth + 1 FROM node n JOIN up ON n.id = up.parent_id
)
SELECT id, name FROM up ORDER BY depth DESC;

-- PATCH node (binds: name, kind, doc, x, y, id; all Option except id); edge/item PATCH follow the same shape
UPDATE node SET name = COALESCE(?, name), kind = COALESCE(?, kind), doc = COALESCE(?, doc),
  x = COALESCE(?, x), y = COALESCE(?, y), updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
WHERE id = ? RETURNING *;

-- creates
INSERT INTO node (parent_id, name, kind, x, y) VALUES (?, ?, ?, ?, ?) RETURNING *;
```
PATCH is partial; `{}` is a valid no-op; zero rows → 404.

**Errors**: `{ "error": "<message>" }`. 400 for any constraint violation with SQLite's message (verified: an edge whose endpoints aren't both children of `parent_id` → `FOREIGN KEY constraint failed`; a node POST with a nonexistent `parent_id` → same; blank name/kind → `CHECK constraint failed`), 404 `"not found"`, 500 `"internal error"` with the cause logged. Handlers trim strings before binding. Malformed JSON gets Axum's own rejection. `shortcut: no pre-check SELECT to turn a bad parent_id into 404, add if the UI needs to distinguish.`

**Deletes**: all DB cascades. Deleting a node removes its whole subtree (children recursively, edges at every level, items). DELETE returns 204, 404 when zero rows affected, 400 for id 1.

**Skipped**: multi-root/multi-system, auth, moving nodes between parents, manual reordering (`shortcut: lists are id order, add an INTEGER position column when drag-reorder is requested`), soft delete/undo (UI confirm instead), pagination/search, per-node child counts, cycle prevention (structural: one parent per node, root fixed), edge uniqueness, renaming or hiding root.

**Seed** (`backend/seed.sql`, never a migration; root comes from the migration): `backend/seed.sql` is the source, not this doc. It opens with `.bail on`, `PRAGMA foreign_keys = ON;` and `BEGIN;` and ends with `COMMIT;`. Without those, the sqlite3 CLI (FKs off, keeps going after an error) duplicates the edges and items on a rerun.

## 2. Rust backend

**`backend/Cargo.toml`** (versions verified Oct 2026; rustc 1.94 local, sqlx 0.9 MSRV 1.86)
```toml
[package]
name = "knotbase"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.9"
tokio = { version = "1.53", features = ["rt-multi-thread", "macros"] }
sqlx = { version = "0.9.0", features = ["runtime-tokio", "sqlite"] }   # sqlite = bundled libsqlite3; defaults keep migrate + derive
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.7.1", features = ["fs"] }

[dev-dependencies]
tower = { version = "0.5.3", features = ["util"] }   # ServiceExt::oneshot
```

**Layout**: flat, one file per resource. No `lib.rs`, no `models/`/`handlers/`/`routes/` trees. Request/response structs live beside the handler that uses them. Each resource file exposes `pub fn router() -> Router<SqlitePool>`.
```
backend/
  Cargo.toml
  migrations/0001_init.sql   # section 1; sqlx::migrate!() embeds ./migrations at compile time
  seed.sql
  src/
    main.rs     # env defaults, pool, migrate, `pub fn app(pool) -> Router`, serve; `#[cfg(test)] mod tests;`
    error.rs    # AppError + IntoResponse + From<sqlx::Error>
    nodes.rs    # Node/Crumb/Graph/NodeDetail structs + 5 handlers (graph, get, create, patch, delete)
    edges.rs    # Edge struct + 3 handlers
    items.rs    # Item struct + 3 handlers
    tests.rs    # oneshot smoke tests (cfg(test) module of the binary, so crate::app is reachable)
```

**State**: no `AppState` struct. `SqlitePool` is `Clone`; handlers take `State(pool): State<SqlitePool>`.

**`error.rs`**
```rust
pub enum AppError { BadRequest(String), NotFound, Internal(sqlx::Error) }
pub type Result<T> = std::result::Result<T, AppError>;

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            // CHECK / FK / NOT NULL / UNIQUE violations: the schema is the validator (section 1)
            sqlx::Error::Database(ref d) if d.kind() != sqlx::error::ErrorKind::Other => Self::BadRequest(d.message().to_string()),
            e => Self::Internal(e),
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::Internal(e) => { eprintln!("db error: {e}"); (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into()) }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
```

**`main.rs`**
```rust
pub fn app(pool: SqlitePool) -> Router {
    let dist = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist").to_string());
    let index = format!("{dist}/index.html");
    if !std::path::Path::new(&index).exists() {
        eprintln!("warning: {index} missing; UI not served on this port (fine in dev, Vite serves it on :5173)");
    }
    let api = Router::new()
        .merge(nodes::router()).merge(edges::router()).merge(items::router())
        .fallback(|| async { AppError::NotFound });   // unknown /api/* stays JSON 404, not index.html
    Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(&dist).fallback(ServeFile::new(index)))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| concat!("sqlite:", env!("CARGO_MANIFEST_DIR"), "/knotbase.db?mode=rwc").to_string());
    let pool = SqlitePool::connect(&url).await.expect("open db");
    sqlx::migrate!().run(&pool).await.expect("migrate");
    // shortcut: loopback only, single user; switch to 0.0.0.0 / BIND env when it runs in a container
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app(pool)).await.unwrap();
}
```

**Query style**: runtime `sqlx::query_as::<_, T>("...")` with `#[derive(FromRow, Serialize)]` row structs. No `query!` macros, so no `DATABASE_URL` at build, no `.sqlx/`, no sqlx-cli. Type map: INTEGER→`i64`, nullable INTEGER→`Option<i64>`, REAL→`f64`, TEXT→`String`, `done` INTEGER→`bool`.
```rust
// nodes.rs
#[derive(FromRow, Serialize)] pub struct Node { id: i64, parent_id: Option<i64>, name: String, kind: String, doc: String, x: f64, y: f64, created_at: String, updated_at: String }
#[derive(FromRow, Serialize)] struct Crumb { id: i64, name: String }
#[derive(Serialize)] struct Graph { nodes: Vec<Node>, edges: Vec<crate::edges::Edge> }
#[derive(Serialize)] struct NodeDetail { #[serde(flatten)] node: Node, items: Vec<crate::items::Item>, breadcrumb: Vec<Crumb> }
#[derive(Deserialize)] struct NewNode { parent_id: i64, name: String, kind: String, #[serde(default)] x: f64, #[serde(default)] y: f64 }
#[derive(Deserialize)] struct NodePatch { name: Option<String>, kind: Option<String>, doc: Option<String>, x: Option<f64>, y: Option<f64> }

async fn delete_node(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    if id == 1 { return Err(AppError::BadRequest("root cannot be deleted".into())); } // only the migration can create it
    let n = sqlx::query("DELETE FROM node WHERE id = ?").bind(id).execute(&pool).await?.rows_affected();
    if n == 0 { Err(AppError::NotFound) } else { Ok(StatusCode::NO_CONTENT) }
}
// edges.rs
#[derive(FromRow, Serialize)] pub struct Edge { id: i64, parent_id: i64, source_id: i64, target_id: i64, label: String, created_at: String, updated_at: String }
#[derive(Deserialize)] struct NewEdge { parent_id: i64, source_id: i64, target_id: i64, #[serde(default)] label: String }
// items.rs
#[derive(FromRow, Serialize)] pub struct Item { id: i64, node_id: i64, text: String, done: bool, created_at: String, updated_at: String }
#[derive(Deserialize)] struct NewItem { node_id: i64, text: String }
```
Patterns: `get_node` = `fetch_one` node (RowNotFound → 404 for free), then `fetch_all` items, then `fetch_all` breadcrumb CTE. `graph` = two `fetch_all`. Creates = `INSERT ... RETURNING *` with `fetch_one`. PATCH = `Option<T>` + COALESCE + `updated_at` in one static statement (section 1); trim with `p.name.as_deref().map(str::trim)`. DELETE = `execute()` → `rows_affected() == 0` → 404 else 204.

**Tests**: router-level tests in `backend/src/tests.rs`, built on an in-memory `app()` helper and a `send()` helper; read them there rather than in this doc.
Add an edge test in M3 (POST edge between two children of root → 201; POST edge across levels → 400) because a missing `UNIQUE (parent_id, id)` only shows up there.

**Skipped**: tracing (`eprintln!` on 500), graceful shutdown, pool/WAL tuning (sqlx defaults: `busy_timeout` 5s, FKs on), request ids, OpenAPI, AppState struct, repository/service layers, dotenvy, rust-embed, chrono, compile-time `query!`. `shortcut: axum extractor rejections (malformed JSON, /api/nodes/abc) are plain-text 4xx not {error}; upgrade with #[derive(FromRequest)] + rejection(AppError) when the UI needs to display them`.

## 3. Frontend (Svelte 5 + @xyflow/svelte)

One page for every depth, a 15-line hand router, plain `fetch`, one stylesheet. Versions = npm `latest` on 2026-10-15, checked against the published `.d.ts` and svelteflow.dev.

**`frontend/package.json`**: scaffold `npm create vite@latest frontend -- --template svelte-ts`, then `npm i @xyflow/svelte marked`. Keep the template's devDeps as generated.
```json
"dependencies":    { "@xyflow/svelte": "^1.7.0", "marked": "^18.1.1" },
"devDependencies": { "@sveltejs/vite-plugin-svelte": "^7.3.1", "@tsconfig/svelte": "^5.0.8", "@types/node": "^24.13.3",
                     "svelte": "^5.57.2", "svelte-check": "^4.7.6", "typescript": "~6.0.2", "vite": "^8.3.4" },
"scripts": { "dev": "vite", "build": "svelte-check --tsconfig ./tsconfig.app.json && vite build", "preview": "vite preview" }
```
- **Do not bump TypeScript to 7.** `svelte-check` peer-depends on `typescript ^5 || ^6` (it drives the TS language service in-process; TS 7 is the Go compiler with no JS API). The template pins `~6.0.2` for this reason.
- `@xyflow/svelte` 1.x is Svelte 5 / runes native and ships `dist/style.css`.
- Markdown: `marked` (zero deps) + `{@html}`; `marked.parse(doc, { async: false })` returns `string`. `shortcut: {@html} renders unsanitised markdown, fine for a single-user local tool; add dompurify if docs ever come from another person.`

**Routing: hand router.** One route pattern needs 15 lines; any router library adds a dependency plus its own `<Link>` for the same result. `shortcut: one route; switch to a router lib when nested layouts or more than ~3 routes appear.`
- `src/nav.svelte.ts`: `export const nav = $state({ path: location.pathname })` (export an object, not a bare `$state`, since exported state can't be reassigned across modules); `addEventListener('popstate', () => nav.path = location.pathname)`; `export function navigate(to) { history.pushState(null, '', to); nav.path = to }`.
- `src/Link.svelte`: `let { href, children, ...rest }: { href: string; children: Snippet } & HTMLAnchorAttributes = $props()`; renders a real `<a {href} {...rest} onclick={click}>`; `click` returns early if `e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey`, else `preventDefault(); navigate(href)`. Cmd-click and copy-link keep working.
- `src/App.svelte`: `const id = $derived(Number(nav.path.match(/^\/node\/(\d+)$/)?.[1] ?? 1))` → `<NodePage {id} />`. `/` and anything unmatched show node 1. Vite's default `appType: 'spa'` serves index.html for `/node/7` in dev; section 4 covers prod.

**`src/api.ts`** (~40 lines). Types verbatim from section 1 plus `Graph` and `NodeDetail`.
```ts
async function req<T>(url: string, method = 'GET', body?: unknown): Promise<T> {
  const res = await fetch(url, { method, headers: { 'content-type': 'application/json' }, body: JSON.stringify(body) }) // undefined body stays undefined
  if (!res.ok) {
    // backend answers {"error": msg}; Axum rejections are plain text; the Vite proxy answers an empty 502 when Rust is down
    const text = await res.text()
    let msg = text
    try { msg = JSON.parse(text).error } catch { /* not JSON, keep the raw text */ }
    throw new Error(msg || `${method} ${url} → ${res.status}`)
  }
  return res.status === 204 ? (undefined as T) : res.json()
}
export const api = {
  graph: (id: number) => req<Graph>(`/api/nodes/${id}/graph`),
  node: (id: number) => req<NodeDetail>(`/api/nodes/${id}`),
  createNode: (b: { parent_id: number; name: string; kind: string; x?: number; y?: number }) => req<Node>('/api/nodes', 'POST', b),
  updateNode: (id: number, b: Partial<Pick<Node, 'name' | 'kind' | 'doc' | 'x' | 'y'>>) => req<Node>(`/api/nodes/${id}`, 'PATCH', b),
  deleteNode: (id: number) => req<void>(`/api/nodes/${id}`, 'DELETE'),
  createEdge: (b: { parent_id: number; source_id: number; target_id: number; label?: string }) => req<Edge>('/api/edges', 'POST', b),
  updateEdge: (id: number, b: { label?: string }) => req<Edge>(`/api/edges/${id}`, 'PATCH', b),   // unused by the UI for now
  deleteEdge: (id: number) => req<void>(`/api/edges/${id}`, 'DELETE'),
  createItem: (b: { node_id: number; text: string }) => req<Item>('/api/items', 'POST', b),
  updateItem: (id: number, b: { text?: string; done?: boolean }) => req<Item>(`/api/items/${id}`, 'PATCH', b),
  deleteItem: (id: number) => req<void>(`/api/items/${id}`, 'DELETE'),
}
```

**Data loading: inline in `NodePage.svelte`** (one page, so a helper would have one caller).
```ts
let { id }: { id: number } = $props()
let data = $state.raw<{ node: NodeDetail; graph: Graph }>()   // raw: never mutated in place, always replaced
let error = $state('')
async function reload() {
  const my = id   // stale-response guard when crumbs are clicked quickly
  try { const [node, graph] = await Promise.all([api.node(my), api.graph(my)]); if (my === id) { data = { node, graph }; error = '' } }
  catch (e) { error = (e as Error).message }
}
async function act(p: Promise<unknown>) { try { await p; await reload() } catch (e) { error = (e as Error).message } }
$effect(() => { id; reload() })   // re-runs when the route id changes
```
Every mutation is `act(api.xxx(...))`: refetch after mutation, no optimistic state. Old `data` stays on screen while the next id loads; "Loading…" only on first paint. `shortcut: extract to load.svelte.ts when a second page appears.`

**File tree `frontend/src/`**
- `main.ts`: template's `mount(App, …)`; imports `@xyflow/svelte/dist/style.css` then `./app.css`.
- `App.svelte`: the regex above → `<NodePage {id} />`.
- `nav.svelte.ts`, `Link.svelte`: above.
- `TopBar.svelte`: 44px bar, `<Link href="/">knotbase</Link>` wordmark + `{#each crumbs}` as Links, last crumb a plain `<span aria-current="page">`; props `crumbs` straight from `data.node.breadcrumb`.
- `api.ts`: above.
- `NodePage.svelte`: the one page. TopBar, error banner (`{#if error}`), header (name + kind chip), add-node form (`<input list="kinds">` + `<datalist>` with `frontend backend database external service flow step`; posts `{parent_id: id, name, kind, x: 40 + 32*n, y: 40 + 32*n}`, n = child count), `GraphCanvas`, `DocPanel`, `ItemList`, empty state "No nodes inside yet. Add a part, a flow or a step." Layout: graph band 45vh, then doc 2fr | items 1fr, stacking on narrow screens. Identical at every depth.
- `GraphCanvas.svelte`: SvelteFlow wrapper (below).
- `CardNode.svelte`: custom node, two Handles, name + kind chip, `style:--kind="var(--kind-{data.kind}, var(--kind-default))"` so unknown kinds fall back to muted.
- `DocPanel.svelte`: props `doc, onSave(doc)`; view = `{@html marked.parse(doc, {async:false})}` or empty-state copy; Edit → `<textarea bind:value>` + Save/Cancel.
- `ItemList.svelte`: props `items, onAdd, onToggle, onDelete`; checkbox → PATCH done, × → DELETE, add input + button.
- `app.css`: the only stylesheet; replaces the template's wholesale. Delete `assets/`, `lib/Counter.svelte`, `public/vite.svg`.
- `index.html`: template + Google Fonts link `css2?family=Atkinson+Hyperlegible+Next:wght@400;500;600;700&display=swap` + title `knotbase`.

**@xyflow/svelte 1.7 wiring** (verified against the published types)
- State: `let rfNodes = $state.raw<Node[]>([])` + `<SvelteFlow bind:nodes={rfNodes} bind:edges={rfEdges}>`. Docs recommend `$state.raw`; SvelteFlow writes new arrays back through the binding on drag/select/delete.
- Resync after refetch: a `$effect` that reads the `nodes`/`edges` props and assigns the mapped arrays. It re-runs only when the page passes a new array (after every `reload()`), so server truth overwrites local state.
- Custom node: `type Card = Node<{ name: string; kind: string }, 'card'>`; `let { data, selected }: NodeProps<Card> = $props()`; `const nodeTypes: NodeTypes = { card: CardNode }` type-checks without a cast. Custom nodes must render their own `<Handle type="target" position={Position.Left} />` and `<Handle type="source" position={Position.Right} />`.
- Events (all lowercase props): `onnodedragstop={({ nodes }) => …}` (`nodes` = the dragged set, so multi-drag persists every moved node; `nodeDragThreshold` defaults to 1 so a plain click never fires it). `onconnect={(c: Connection) => …}` with string `c.source`/`c.target`. `onbeforedelete` must return a `Promise<boolean | {nodes, edges}>` → an `async` arrow returning `confirm(...)`. `ondelete={({ nodes, edges }) => …}` fires after local removal and includes edges attached to deleted nodes. **There is no double-click node event in v1**: use `onnodeclick` and check `event.detail === 2`.
- Delete key default `'Backspace'`. `fitView` is a boolean prop (initial viewport only). `<Background />` defaults to dots; `<Controls />`; no MiniMap. `<SvelteFlow>` inherits its parent's size, so the wrapper div needs an explicit height.
```svelte
<!-- GraphCanvas.svelte -->
<script lang="ts">
  import { SvelteFlow, Background, Controls, MarkerType, type Node, type Edge as FlowEdge, type NodeTypes, type Connection } from '@xyflow/svelte'
  import CardNode from './CardNode.svelte'
  import type { Node as ApiNode, Edge as ApiEdge } from './api'
  let { nodes, edges, onMove, onConnect, onDelete, onOpen }: {
    nodes: ApiNode[]; edges: ApiEdge[]
    onMove: (id: number, x: number, y: number) => void; onConnect: (sourceId: number, targetId: number) => void
    onDelete: (nodeIds: number[], edgeIds: number[]) => void; onOpen: (id: number) => void
  } = $props()
  const nodeTypes: NodeTypes = { card: CardNode }
  let rfNodes = $state.raw<Node[]>([])
  let rfEdges = $state.raw<FlowEdge[]>([])
  $effect(() => {   // server truth wins after every refetch
    rfNodes = nodes.map(n => ({ id: String(n.id), type: 'card', position: { x: n.x, y: n.y }, data: { name: n.name, kind: n.kind } }))
    rfEdges = edges.map(e => ({ id: String(e.id), source: String(e.source_id), target: String(e.target_id), label: e.label || undefined,
                                type: 'smoothstep', markerEnd: { type: MarkerType.ArrowClosed } }))
  })
</script>
<div class="canvas">
  <SvelteFlow bind:nodes={rfNodes} bind:edges={rfEdges} {nodeTypes} fitView
    onnodedragstop={({ nodes }) => nodes.forEach(n => onMove(Number(n.id), n.position.x, n.position.y))}
    onconnect={(c: Connection) => onConnect(Number(c.source), Number(c.target))}
    onbeforedelete={async ({ nodes }) => nodes.length === 0 || confirm(`Delete ${nodes.length} node(s) and everything inside them?`)}
    ondelete={({ nodes, edges }) => { const gone = new Set(nodes.map(n => n.id))   // cascaded edges must not be deleted twice
      onDelete(nodes.map(n => Number(n.id)), edges.filter(e => !gone.has(e.source) && !gone.has(e.target)).map(e => Number(e.id))) }}
    onnodeclick={({ node, event }) => { if (event.detail === 2) onOpen(Number(node.id)) }}>
    <Background /><Controls />
  </SvelteFlow>
</div>
<style>.canvas { height: 45vh; }</style>
```
Page wiring: `onMove={(nid, x, y) => act(api.updateNode(nid, { x, y }))}`, `onConnect={(s, t) => act(api.createEdge({ parent_id: id, source_id: s, target_id: t }))}`, `onDelete={(ns, es) => act(Promise.all([...ns.map(api.deleteNode), ...es.map(api.deleteEdge)]))}`, `onOpen={cid => navigate(`/node/${cid}`)}`.

**TS setup**: keep the template's `svelte.config.js`, `tsconfig.json` (project references), `tsconfig.app.json` (`extends @tsconfig/svelte`, `types: ["svelte","vite/client"]`, `verbatimModuleSyntax` inherited, so `import type` for types), `tsconfig.node.json`. Nothing to add.

**Visual direction** (unchanged from v1, from the frontend-design skill): an engineer's computation pad, index cards on pale green graph paper. The canvas is the hero; colour only ever means kind.
- Palette: paper `#EEF1EB`, card `#FFFFFF`, ink `#22302A`, rule `#C8D0C4`, muted `#5F6E65`. Kinds: frontend `#3A66C9`, backend `#2E7D5B`, database `#B8862B`, external `#7B5EA7`, service `#B4455A`, flow `#2B7A8C`, step `#6B7F2E`, default = muted. Kind shown as colour AND a word chip.
- Type: Atkinson Hyperlegible Next, system-ui fallback. 13px chrome/forms/node names (500), 11px kind chip, 15px/1.6 doc body at max 70ch, 18px/600 page title, lowercase `knotbase` wordmark 15px/700.
- Density: compact; spacing 4/8/12/16/24; no shadows; one hairline rule; radii cards 4px, buttons/inputs 2px. Primary button ink fill / paper text; secondary bordered. Focus 2px ink outline, offset 2. Edges 1.5px ink, selected 2.5px.
- Copy: sentence case, verbs on buttons. Errors: one-line banner under the bar with the `req` message. No motion beyond SvelteFlow's own.

**Skipped**: undo (`confirm()` instead), search/filter, keyboard shortcuts beyond Backspace, dark mode, MiniMap, `shortcut: no layout engine, add dagre/elk when a level outgrows hand placement`, `shortcut: refetch after every mutation, no optimistic state; revisit if the backend is ever remote`, `shortcut: no inline rename or edge-label editing (updateEdge is wired in api.ts but unused); add when it bites`, `shortcut: a node cannot be moved to another parent; add a "move to" action when re-filing is needed`, `shortcut: deleting the current node happens from its parent's canvas only`, `shortcut: single user, no polling; a refetch landing mid-drag resets that drag`, `shortcut: new nodes stagger from top-left; use screenToFlowPosition via useSvelteFlow() when that annoys`, tests beyond `svelte-check && vite build`.

## 4. Single-port dev/prod topology

**Approach**: dev = Vite on :5173 is the front door and proxies `/api` to Rust on :3000. Prod = Rust on :3000 is the front door, mounts `/api`, serves `frontend/dist` with SPA fallback. Same-origin in both modes, so no CORS layer ever. Rejected: Rust reverse-proxying to Vite (needs a proxy crate + HMR WebSocket passthrough for zero benefit) and nginx/caddy (a third binary to install and run).

**`frontend/vite.config.ts`** (svelte plugin; exact import in section 3)
```ts
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
export default defineConfig({
  plugins: [svelte()],
  server: { proxy: { '/api': 'http://localhost:3000' } },
})
```
String shorthand = prefix match, no rewrite (`/api/nodes/1/graph` → `localhost:3000/api/nodes/1/graph`). No `changeOrigin` (Axum doesn't route on Host), no `ws`. While `cargo run` compiles, `/api` calls log `ECONNREFUSED` in the Vite terminal; refresh once the backend prints its listen line.

**Axum router (prod)**: see `app()` in section 2 (tower-http 0.7 with `features = ["fs"]`).
- Use `ServeDir::fallback`, NOT `not_found_service`: verified in the tower-http source that `not_found_service` wraps the fallback in `SetStatus(404)`, so `/node/6` would return index.html with a 404. `.fallback` gives 200.
- The one-line 404 fallback on the API router stops `GET /api/typo` from falling through to ServeDir and returning HTML to a `fetch().json()` call.
- Missing `dist` in dev → plain 404 on :3000, which nobody hits directly. The single `eprintln!` is a debugging aid.

**Config**: two env vars, both defaulting relative to `env!("CARGO_MANIFEST_DIR")` (= `backend/`) because `cargo run` does not chdir, so a plain relative path silently 404s depending on where you typed the command.
- `STATIC_DIR` default `<manifest>/../frontend/dist`
- `DATABASE_URL` default `sqlite:<manifest>/knotbase.db?mode=rwc`
`shortcut: paths are baked at compile time, so a release binary copied elsewhere points at the build box's paths; set STATIC_DIR/DATABASE_URL explicitly when deploying off this laptop.`

**Embed vs disk**: serve from disk. `shortcut: UI is served from disk, so the binary needs frontend/dist beside it; switch to rust-embed when you want single-file distribution.`

**One-command dev**: `dev.sh` at repo root (zero installs, one file, clean Ctrl-C). Beats root `package.json` + `concurrently` and `justfile` (`just` is not installed here).
```sh
#!/bin/sh
# Dev front door is Vite on :5173; it proxies /api to the Rust server on :3000.
cd "$(dirname "$0")"
trap 'kill 0' EXIT
(cd backend && cargo run) &
(cd frontend && npm run dev)
```
Vite is foreground so its `q`/`o`/`r` shortcuts work; `trap 'kill 0' EXIT` takes the backend down on Ctrl-C or `q`. Backend auto-reload: skip for v1 (cargo-watch is deprecated by its author; neither it nor bacon is installed). When restarts get annoying: `cargo install bacon`, swap `cargo run` for `bacon run-long`.

**Prod build + run**
```sh
cd frontend && npm run build        # Vite default outDir = dist/, no `base` needed
cd ../backend && cargo run --release
open http://localhost:3000
```

**Skipped**: HTTPS, compression/caching headers (add tower-http `compression-gzip` + `Cache-Control: immutable` on `/assets/*` when it matters), Docker, CORS, dotenvy, `strictPort`/`allowedHosts`.

## 5. Repo structure, tooling, CI, milestones

**Root files**
- `.gitignore`: `target/ node_modules/ dist/ *.db *.db-* .env* .DS_Store` (depth-agnostic; delete the Vite-scaffolded `frontend/.gitignore`).
- `README.md` (~15 lines): what it is, `./dev.sh` → :5173, prod build + `cargo run --release` → :3000, seed/reset DB.
- `dev.sh`: section 4.
- `CLAUDE.md`: layout (backend = Cargo, frontend = Vite/Svelte, no root workspace), run commands, "add an endpoint = handler + route in the resource file + one line in `frontend/src/api.ts`", "the UI is one page, `NodePage.svelte`, rendered at every depth", ponytail rules + `shortcut:` convention, "runtime sqlx queries only", "TypeScript stays on 6.x for svelte-check".
- `.github/workflows/ci.yml`: two jobs. Backend: `dtolnay/rust-toolchain@stable` (rustfmt, clippy) + `Swatinem/rust-cache@v2` → `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Frontend: `actions/setup-node@v4` (node 24, npm cache on `frontend/package-lock.json`) → `npm ci`, `npm run build` (runs `svelte-check` then `vite build`).
- Skipped: `.editorconfig`, prettier, ESLint (`svelte-check` is the gate). `shortcut: no JS lint; add eslint-plugin-svelte when a reactivity bug costs an hour`.

**Testing v1**
- Backend: `src/tests.rs` (section 2): post child + graph + breadcrumb, delete-root 400; M3 adds the edge tests. In-memory pool with `max_connections(1)`.
- Frontend: `npm run build` only. `shortcut: zero frontend tests; add vitest when the first pure helper grows a branch, playwright when a click path regresses twice`.
- Runtime `sqlx::query_as`, never `query!`, so CI needs no `DATABASE_URL` or `.sqlx/`.

**Seed data**: `backend/seed.sql` (section 1). Apply: run the backend once to create the schema and root, then `sqlite3 backend/knotbase.db < backend/seed.sql`. Reset: `rm backend/knotbase.db*`. `shortcut: seed needs the sqlite3 CLI (present on this Mac); add --seed when a dev box lacks it`.

**Milestones (one commit each; CI wired from M1)**
- **M1 backend nodes**: Cargo project, `migrations/0001_init.sql` (3 tables + root), `src/main.rs`, `src/error.rs`, `src/nodes.rs` (graph, get with breadcrumb, create, patch, delete with root guard), `src/tests.rs` (2 tests), `seed.sql`, root `.gitignore`, `dev.sh`, `CLAUDE.md`, `ci.yml` (backend job), commit `docs/`. Verify: `cargo test`; `curl :3000/api/nodes/1/graph` → `{"nodes":[],"edges":[]}`; POST `{"parent_id":1,"name":"API","kind":"backend"}` → 201; `curl :3000/api/nodes/2` → breadcrumb `[System, API]`; `curl -X DELETE :3000/api/nodes/1` → 400.
- **M2 frontend read-only**: `npm create vite@latest frontend -- --template svelte-ts`, prune, `npm i @xyflow/svelte marked`, proxy, `api.ts`, `nav.svelte.ts` + `Link.svelte` + `TopBar.svelte`, `App.svelte`, `NodePage.svelte` (read-only canvas + breadcrumb + header), `GraphCanvas.svelte` + `CardNode.svelte`, `app.css`. Add the frontend CI job. Verify: seed, `./dev.sh`, open `:5173` → 4 cards 3 edges; double-click API → `/node/3` shows Payment Flow; refresh there keeps the view; `npm run build`, `cargo run --release`, open `:3000/node/3` → same page.
- **M3 editing**: backend `src/edges.rs` (3 routes) + edge tests (sibling edge → 201, cross-level edge → 400); frontend add-node form, `onnodedragstop` → PATCH x/y, `onconnect` → POST edge with `parent_id`, Backspace → confirm → DELETE node(s) + non-cascaded edges. Verify: add, drag, connect, delete, reload persists; `curl` a cross-level edge → 400.
- **M4 doc + items**: backend `src/items.rs` (3 routes); frontend `DocPanel.svelte` (marked) + `ItemList.svelte` wired into NodePage. Verify: on Payment Flow edit the doc with a heading and code block, Save → rendered; add/toggle/delete items; refresh persists.
- **M5 polish**: empty states, error banner, README, CLAUDE.md endpoint list, push → CI green. Verify: wipe DB → root with empty states; stop backend, tick an item → banner; restart, click → clears.

**Skipped**: Docker, release pipeline, pre-commit hooks, dependabot, CODEOWNERS, changelog, Cargo workspace (single crate), tracing, CORS, dotenv.

## Verification (end to end, after M5)

Automated:
```sh
cd backend && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
cd ../frontend && npm ci && npm run build
```

Dev path (`./dev.sh`, DB seeded):
1. `curl -s localhost:5173/api/nodes/1/graph` → JSON through the Vite proxy; `curl -si localhost:5173/api/nope | head -1` → 404.
2. Open http://localhost:5173/ → TopBar `knotbase` + crumb `System`; 4 cards with kind chips, 3 arrowed edges; no CORS errors.
3. Add node `Queue` kind `service` → card appears. Drag it; `PATCH /api/nodes/9 {x,y}` in the network tab; reload → same position. Drag a connection API → Queue → `POST /api/edges {parent_id: 1, …}`.
4. Double-click `API` → `/node/3`, crumbs `System / API`, Payment Flow card. Double-click `Payment Flow` → `/node/6`, crumbs `System / API / Payment Flow`, two steps with an edge. Add step `Record order`, connect it. Edit the doc, Save → rendered heading. Add, tick, untick, delete items. Every action is one request and survives F5.
5. Hard refresh on `/node/6` → same page (crumbs from `GET /api/nodes/6`). Cmd-click the `API` crumb → new tab on `/node/3`, current tab unchanged. Plain-click `System` → `/node/1` with no document request. Browser Back/Forward walk the levels.
6. On `/node/3` select `Payment Flow`, Backspace → confirm; Cancel → nothing; OK → node and its edges gone, only `DELETE /api/nodes/6` fires. Visit `/node/6` directly → banner with the backend's 404 text.
7. Stop the Rust server, tick an item → banner `PATCH /api/items/1 → 500`; restart, click again → banner clears.
8. `rm backend/knotbase.db*`, restart → `/node/1` with empty states.

Prod path (`cd frontend && npm run build; cd ../backend && cargo run --release`):
- `curl -si localhost:3000/ | head -1` → 200 index.html; `curl -si localhost:3000/node/6 | head -1` → 200 index.html (SPA fallback via `ServeDir::fallback`).
- `curl -s localhost:3000/api/nodes/1/graph` → JSON; `curl -si localhost:3000/api/nope | head -1` → 404 JSON, not HTML.
- `curl -si localhost:3000/assets/$(ls frontend/dist/assets | head -1) | head -1` → 200.

Push to `main` → both CI jobs green.

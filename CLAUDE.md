# knotbase

Graph-first system explorer: a tree of nodes, each with a child graph, markdown doc and checklist. Full design: `docs/Implementation-plan.md`.

## Layout
- `backend/`: Rust + Axum + sqlx over SQLite (default) or Postgres, Cargo conventions; folders under "## Backend".
- `frontend/`: Svelte 5 + Vite + `@xyflow/svelte`, Vite/Svelte conventions.
- No root workspace. The root `package.json` only holds `npm start` (runs `dev.sh`); it has no dependencies. Repo root also holds `dev.sh`, README, this file, `docs/` and CI.

## Run, test, build
- Setup (once): `cd frontend && npm ci`
- Dev: `npm start` from the repo root, same as `./dev.sh` (Vite :5025 proxies `/api` to Rust :3000), open http://localhost:5025. `PORT=3100 npm start` moves the Rust port and the proxy together.
- Backend (in `backend/`): `cargo run`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
- Backend on Postgres: `createdb knotbase` once, then `DATABASE_URL=postgres://$USER@localhost/knotbase cargo run`. Tests on Postgres: `TEST_DATABASE_URL=postgres://$USER@localhost/postgres cargo test` (the login needs CREATEDB; every test creates its own `knotbase_test_<test name>` database and leaves it for the next run to recreate, and the database named in the URL is never written; CI runs the suite on both engines).
- Frontend (in `frontend/`): `npm run dev`, `npm run lint` (Prettier check then ESLint; CI runs it), `npm run format` (Prettier write), `npm run build` (`svelte-check` then `vite build`; CI runs it). `npm run check` also type-checks `vite.config.ts`, which CI does not.
- Prod: `cd frontend && npm run build`, then `cd backend && cargo run --release`, open http://localhost:3000.
- Seed: `sqlite3 backend/knotbase.db < backend/seed.sql` after one backend run, on an empty DB (all-or-nothing; a rerun fails cleanly). Reset: `rm backend/knotbase.db*`. Postgres: run the backend once (it creates the tables), then `psql -1 -v ON_ERROR_STOP=1 -d knotbase -f backend/seed.postgres.sql`; reset: `dropdb knotbase && createdb knotbase`.
- Env (read in `backend/src/config.rs`): `PORT` (default 3000; the backend listens on 127.0.0.1 only, and the Vite proxy reads the same variable), `STATIC_DIR` (default `<repo>/frontend/dist`) and `DATABASE_URL` (default `sqlite:<repo>/backend/knotbase.db?mode=rwc`). The URL scheme picks the engine: `sqlite:` (without `?mode=rwc` a missing file is not created) or `postgres://user@host/db` (the database must exist; sqlx does not take the OS user, so name one in the URL or set `PGUSER`). The two path defaults are absolute (baked from `CARGO_MANIFEST_DIR`); a value you set is used as written, relative to the working directory (`backend/` under `cargo run`), so prefer absolute paths.
- `.playwright-mcp/` at the repo root is output from browser tools; it is gitignored, so never committed.

## Backend
`backend/src/` is a library (`lib.rs`) plus the `main.rs` binary (config, connect, listen), so `backend/tests/api/` drives the real app: one test binary, `main.rs` holds the helpers, one file per area.

| Folder | Holds | May import |
|---|---|---|
| `src/routes/` | Axum: `app()` (the `/api` router, the static UI, the host guard) and one file per resource with its `router()` and thin handlers: extract, call a service, answer with a status and `Json` | services, db, error |
| `src/services/` | one file per resource: the row structs (they are the API bodies too) and `pub async fn`s over `&Db`; the only SQL in the crate | db, error, sibling services |
| `src/services/db.rs` | the one database interface: `enum Db`, a variant per engine with its native pool; `connect(url, max)` picks the variant from the URL scheme and runs that engine's `migrations/<engine>/`; `with_now()` and the `sql!` macro | sqlx |
| `src/config.rs` | `Config::from_env()`: every environment variable the binary reads, with its default; the only place `std::env` is read | |
| `migrations/<engine>/` | the same schema per engine, same version numbers (`sqlite/`, `postgres/`) | |

Portable SQL: a service runs one sqlx expression through `sql!(db, |pool| …)`, which expands once per engine, so the SQL text must work on every engine: `$n` placeholders, `RETURNING`, `COALESCE`, `WITH RECURSIVE`, `{now}` through `db.with_now()` for the current time, nothing else engine-specific; the expression (a chain, or a block with a transaction) ends in a row struct, a scalar or `()`, never a driver row. The Postgres test run is the lint. Engine traps: booleans in SQL text (bind a `bool` or write bare `WHERE done`, never `= 1`; count with `COUNT(*) FILTER (WHERE done)`, never `SUM`), `COALESCE($n, col)` can never set a column back to NULL (keep columns `NOT NULL` with a default), and the `graph` query names the node columns, so a new column goes there too. Adding an engine: its sqlx feature, a `Db` variant, an arm in `connect()`, `now()` and `sql!`, and a migrations directory; services stay untouched.

## Data model
- `node`: recursive table, id 1 is the root "System", every other node has a `parent_id`. Carries `doc` (markdown) and x/y.
- `edge`: links siblings only, enforced by a composite FK `(parent_id, source_id|target_id) -> node(parent_id, id)`.
- `item`: checklist entry attached to any node. `done` is INTEGER 0/1 in SQLite and BOOLEAN in Postgres, `bool` in Rust; timestamps are ISO-8601 text (`2026-10-20T01:27:33.123Z`) in both engines.

## API (JSON, snake_case)
| Method | Path | Body | Codes |
|---|---|---|---|
| GET | `/api/nodes/{id}/graph` | | 200 |
| GET | `/api/nodes/{id}` | | 200 404 |
| POST | `/api/nodes` | `{parent_id, name, kind, x?, y?}` | 201 400 422 |
| PATCH | `/api/nodes/{id}` | `{name?, kind?, doc?, x?, y?}` | 200 400 404 422 |
| DELETE | `/api/nodes/{id}` | | 204 400 (id 1) 404 |
| POST | `/api/edges` | `{parent_id, source_id, target_id, label?}` | 201 400 422 |
| PATCH | `/api/edges/{id}` | `{label?}` | 200 400 404 422 |
| DELETE | `/api/edges/{id}` | | 204 404 |
| POST | `/api/items` | `{node_id, text}` | 201 400 422 |
| PATCH | `/api/items/{id}` | `{text?, done?}` | 200 400 404 422 |
| DELETE | `/api/items/{id}` | | 204 404 |

- Handler errors are `{"error": "..."}`: 400 for a constraint violation, 404 `"not found"`, 500 `"internal error"`.
- Axum's extractor rejections are plain text, not JSON:
  - malformed JSON: 400
  - missing or mistyped field, or an unknown PATCH field (e.g. `{"parent_id":5}`): 422
  - no `content-type: application/json`: 415
  - non-numeric `{id}`: 400
  - body over 2 MiB: 413 (Axum's default body limit)
  - a wrong verb on a known path: empty 405
- A foreign `Host` on any route (not `localhost`, `127.0.0.1` or `[::1]`, any port) gets an empty 403 (DNS-rebinding guard); a missing Host is allowed.
- `GET /api/nodes/{id}/graph` returns `doc: ""` for children; use `GET /api/nodes/{id}` for a doc. On a missing id it returns 200 with empty lists; the page's `GET /api/nodes/{id}` supplies the 404.
- `frontend/src/lib/services/api.ts` `req()` handles both error shapes.

## Add an endpoint
A `pub async fn` in `backend/src/services/<resource>.rs`, a handler plus a route in `backend/src/routes/<resource>.rs`, and one line in `frontend/src/lib/services/api.ts`. A new resource also needs `pub(crate) mod x;` in `services/mod.rs` and `mod x;` plus `.merge(x::router())` in `routes/mod.rs`; a new table or column is one migration per engine.

## UI
`frontend/src/` follows the SvelteKit idiom without SvelteKit (`$lib` alias in vite and tsconfig). `App.svelte` is the hand router: `/` → `routes/+page.svelte`, `/node/:id` → `routes/node/[id]/+page.svelte`, both inside `routes/+layout.svelte`; `/node/1` redirects to `/` because systems are the children of root 1 and the root is never shown.

| Folder | Holds | May import |
|---|---|---|
| `routes/` | the layout (the one navbar, fed by the shell store) and one file per page; a page composes widgets and sets its navbar chrome in the shell store, owns nothing reusable | widgets, components, stores |
| `lib/components/` | presentational: props in, callbacks out, scoped `<style>`; no store or service imports | components, utils |
| `lib/widgets/` | composites wired to state: read stores, call services, compose components (NodeFlow, SystemGrid, `inspector/*`) | everything below |
| `lib/services/` | the central event listener: `api.ts` (HTTP), one handler module per resource (`nodes`, `edges`, `items`, `systems`) plus `nav`, `ui` and `telemetry`, registered by `listen.ts` from `main.ts`; the only code that calls the API, `navigate()` or `toggleTheme()` (lint-enforced) | `api.ts`, stores, events |
| `lib/stores/` | reactive state as `.svelte.ts`: `data` (the one read path: a cache of nodes, graphs and systems; a miss emits `get:*`), `router`, `theme`, `notify`, `shell`, and per node page `LevelState` (selection, popover; provided via context) | events, utils |
| `lib/events/` | `bus.ts` (the typed request/response bus: `on`, `emit`, `tap`) and `types.ts` (the event contract) | nothing |
| `lib/utils/` | pure helpers and the form helpers | nothing |
| `lib/styles/` | `tokens.css` (the two palettes), `base.css` (reset, element defaults, the closed set of utilities), `xyflow.css` (library overrides): the only global CSS | |

Data flow: the UI reads only `data.node(id)`, `data.graph(id)`, `data.systems()`; a miss schedules the matching `get:*` event; a write is `bus.emit('post:|patch:|delete:…', payload)` and resolves to what the UI needs next. Event names mirror the API table: `get:node` `get:graph` `get:systems` `post:node` `patch:node` `patch:nodes` `delete:node` `delete:many` `post:edge` `patch:edge` `delete:edge` `post:item` `patch:item` `delete:item` `post:system`; Navigation and UI actions ride the same bus: `nav:to` `nav:node` `nav:home` `theme:toggle`, handled in `lib/services/nav.ts` and `ui.ts` (the only callers of `navigate()` and `toggleTheme()`; a same-origin `<a>` click is turned into `nav:to` by the router store). The contract is `lib/events/types.ts`, the bus `lib/events/bus.ts` (one handler per event, `emit` resolves to its result, `tap()` observers see every event with its result or error and duration; `lib/services/telemetry.ts` is the one tap, a dev console line for now). A mutation handler calls the API through `act()` and then `data.refresh()`es the keys it touched (its graph; Home when a system's top level changed; cached descendants on a rename, dropped on a delete).

Rules: no inline `<svg>` outside `Icon.svelte`; navigation is a plain `<a href>` or a `nav:*` event, never `navigate()` outside `lib/services` (lint-enforced); forms use `submitGuard`, `escape` and `focus`; mutation handlers clear the banner first and set it on failure, get handlers only set a per-key status in `data`; `:global` only for xyflow internals and rendered markdown.
## Rules
- Runtime `sqlx::query`/`query_as` only, never `query!` macros (no `DATABASE_URL` at build, no `.sqlx/`).
- The schema is the validator: CHECK and FK constraints, mapped to 400. No hand-written validators.
- A committed migration is never edited (every database stores its checksum); a schema change is a new version in every `migrations/<engine>/`.
- Never disable SQLite foreign keys; cascades silently stop working.
- TypeScript stays on 6.x: `svelte-check` does not support 7.
- Ponytail mode: smallest complete change. Mark known limits in code as `shortcut: <limit>, <when to upgrade>`.

## Commits
Conventional Commits: `type(scope): imperative subject`, body says why. Types `feat` `fix` `refactor` `chore` `docs` `test` `ci`; scopes `frontend` `backend` `dev` `ci` `docs` (none for repo-wide). No Co-Authored-By or other trailers.

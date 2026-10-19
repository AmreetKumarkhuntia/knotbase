# knotbase

Graph-first system explorer: a tree of nodes, each with a child graph, markdown doc and checklist. Full design: `docs/Implementation-plan.md`.

## Layout
- `backend/`: Rust + Axum + SQLite, Cargo conventions. One file per resource in `backend/src/`.
- `frontend/`: Svelte 5 + Vite + `@xyflow/svelte`, Vite/Svelte conventions.
- No root workspace. The root `package.json` only holds `npm start` (runs `dev.sh`); it has no dependencies. Repo root also holds `dev.sh`, README, this file, `docs/` and CI.

## Run, test, build
- Setup (once): `cd frontend && npm ci`
- Dev: `npm start` from the repo root, same as `./dev.sh` (Vite :5025 proxies `/api` to Rust :3000), open http://localhost:5025.
- Backend (in `backend/`): `cargo run`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
- Frontend (in `frontend/`): `npm run dev`, `npm run lint` (Prettier check then ESLint; CI runs it), `npm run format` (Prettier write), `npm run build` (`svelte-check` then `vite build`; CI runs it). `npm run check` also type-checks `vite.config.ts`, which CI does not.
- Prod: `cd frontend && npm run build`, then `cd backend && cargo run --release`, open http://localhost:3000.
- Seed: `sqlite3 backend/knotbase.db < backend/seed.sql` after one backend run, on an empty DB (all-or-nothing; a rerun fails cleanly). Reset: `rm backend/knotbase.db*`.
- Env: `STATIC_DIR` (default `<repo>/frontend/dist`) and `DATABASE_URL` (default `sqlite:<repo>/backend/knotbase.db?mode=rwc`; must be a `sqlite:` URL, and without `?mode=rwc` a missing file is not created). Both defaults are absolute (baked from `CARGO_MANIFEST_DIR`); a value you set is used as written, relative to the working directory (`backend/` under `cargo run`), so prefer absolute paths.
- `.playwright-mcp/` at the repo root is output from browser tools; it is gitignored, so never committed.

## Data model
- `node`: recursive table, id 1 is the root "System", every other node has a `parent_id`. Carries `doc` (markdown) and x/y.
- `edge`: links siblings only, enforced by a composite FK `(parent_id, source_id|target_id) -> node(parent_id, id)`.
- `item`: checklist entry attached to any node.

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
  - a wrong verb on a known path: empty 405
- A foreign `Host` on any route (not `localhost`, `127.0.0.1` or `[::1]`, any port) gets an empty 403 (DNS-rebinding guard); a missing Host is allowed.
- `GET /api/nodes/{id}/graph` returns `doc: ""` for children; use `GET /api/nodes/{id}` for a doc. On a missing id it returns 200 with empty lists; the page's `GET /api/nodes/{id}` supplies the 404.
- `frontend/src/api.ts` `req()` handles both error shapes.

## Add an endpoint
A handler plus a route in the resource file under `backend/src/`, plus one line in `frontend/src/api.ts`.

## UI
- Two pages under `frontend/src/pages/`, routed by `App.svelte`: `HomePage.svelte` at `/` (the systems grid: a system is a child of root 1 with kind `system`; root 1 itself is never shown and `/node/1` redirects to `/`) and `NodePage.svelte` at `/node/:id` (canvas left, inspector right, rendered at every depth).
- Components by area: `components/` (Button, Link, Swatch, TopBar, ThemeToggle), `graph/` (xyflow canvas, card, add-node popover, legend, help), `inspector/` (node and edge inspectors and their sections), `home/` (system card, new-system form).
- Shared state: `ui.svelte.ts` (error banner; `act()` wraps mutations and clears the banner first, `load()` wraps fetches and never clears it), `theme.svelte.ts` (notebook/console, persisted in localStorage), `nav.svelte.ts` (history). Look tokens live in `app.css`: `:root` is the light "notebook", `[data-theme=console]` the dark one.

## Rules
- Runtime `sqlx::query_as` only, never `query!` macros (no `DATABASE_URL` at build, no `.sqlx/`).
- The schema is the validator: CHECK and FK constraints, mapped to 400. No hand-written validators.
- Never disable SQLite foreign keys; cascades silently stop working.
- TypeScript stays on 6.x: `svelte-check` does not support 7.
- Ponytail mode: smallest complete change. Mark known limits in code as `shortcut: <limit>, <when to upgrade>`.

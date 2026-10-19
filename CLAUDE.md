| `lib/stores/` | reactive state as `.svelte.ts`: `data` (the one read path: a cache of nodes, graphs and systems; a miss emits `get:*`), `router`, `theme`, `notify`, `shell`, and per node page `LevelState` (selection, popover; provided via context) | events, utils || `lib/services/` | the central event listener: `api.ts` (HTTP) and one handler module per resource (`nodes`, `edges`, `items`, `systems`) registered by `listen.ts` from `main.ts`; the only code that calls the API (lint-enforced) | `api.ts`, stores, events |# knotbase

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
`frontend/src/` follows the SvelteKit idiom without SvelteKit (`$lib` alias in vite and tsconfig). `App.svelte` is the hand router: `/` → `routes/+page.svelte`, `/node/:id` → `routes/node/[id]/+page.svelte`, both inside `routes/+layout.svelte`; `/node/1` redirects to `/` because systems are the children of root 1 and the root is never shown.

| Folder | Holds | May import |
|---|---|---|
| `routes/` | the layout (the one navbar, fed by the shell store) and one file per page; a page composes widgets and sets its navbar chrome in the shell store, owns nothing reusable | widgets, components, stores |
| `lib/components/` | presentational: props in, callbacks out, scoped `<style>`; no store or service imports | components, utils |
| `lib/widgets/` | composites wired to state: read stores, call services, compose components (NodeFlow, SystemGrid, `inspector/*`) | everything below |
| `lib/services/` | data access and domain operations, plain TS, no runes (`api`, `nodes`, `systems`) | `api.ts` |
| `lib/stores/` | reactive state as `.svelte.ts`: `router`, `theme`, `notify`, `shell`, and per node page `LevelState` (data, selection, mutations; provided via context) over `NodeData` | services, utils |
| `lib/utils/` | pure helpers and the form helpers | nothing |
| `lib/styles/` | `tokens.css` (the two palettes), `base.css` (reset, element defaults, the closed set of utilities), `xyflow.css` (library overrides): the only global CSS | |

Data flow: the UI reads only `data.node(id)`, `data.graph(id)`, `data.systems()`; a miss schedules the matching `get:*` event; a write is `bus.emit('post:|patch:|delete:…', payload)` and resolves to what the UI needs next. Event names mirror the API table: `get:node` `get:graph` `get:systems` `post:node` `patch:node` `patch:nodes` `delete:node` `delete:many` `post:edge` `patch:edge` `delete:edge` `post:item` `patch:item` `delete:item` `post:system`; the contract is `lib/events/types.ts`, the bus `lib/events/bus.ts` (one handler per event, `emit` returns its promise, dev logs `[event] …`). A mutation handler calls the API through `act()` and then `data.refresh()`es the keys it touched.

Rules: no inline `<svg>` outside `Icon.svelte`; navigation is a plain `<a href>` (the router store delegates same-origin clicks); forms use `submitGuard`, `escape` and `focus`; mutation handlers clear the banner first and set it on failure, get handlers only set a per-key status in `data`; `:global` only for xyflow internals and rendered markdown.
## Rules
- Runtime `sqlx::query_as` only, never `query!` macros (no `DATABASE_URL` at build, no `.sqlx/`).
- The schema is the validator: CHECK and FK constraints, mapped to 400. No hand-written validators.
- Never disable SQLite foreign keys; cascades silently stop working.
- TypeScript stays on 6.x: `svelte-check` does not support 7.
- Ponytail mode: smallest complete change. Mark known limits in code as `shortcut: <limit>, <when to upgrade>`.

## Commits
Conventional Commits: `type(scope): imperative subject`, body says why. Types `feat` `fix` `refactor` `chore` `docs` `test` `ci`; scopes `frontend` `backend` `dev` `ci` `docs` (none for repo-wide). No Co-Authored-By or other trailers.

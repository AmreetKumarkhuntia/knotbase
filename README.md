# knotbase

Explore a system as nested graphs: every node holds a graph of child nodes, markdown docs and a checklist.

**Setup (once):** `cd frontend && npm ci`

**Dev:** `npm start` (or `./dev.sh`), then open http://localhost:5025.

**Prod:**
```sh
cd frontend && npm run build
cd ../backend && cargo run --release
```
Then open http://localhost:3000.

**Seed:** run the backend once, then `sqlite3 backend/knotbase.db < backend/seed.sql`. It seeds an empty DB once; a second run fails and changes nothing. **Reset:** `rm backend/knotbase.db*`.

**Env vars** (defaults are absolute paths baked in at compile time from the repo location):
- `STATIC_DIR`: built UI to serve. Default `<repo>/frontend/dist`.
- `DATABASE_URL`: SQLite database, a `sqlite:` URL ending in `?mode=rwc`. Default `sqlite:<repo>/backend/knotbase.db?mode=rwc`.

Values you set yourself are used as written (relative to the working directory, i.e. backend/ under cargo run); use absolute paths and keep ?mode=rwc so a new DB file is created. Seeding needs the sqlite3 CLI.

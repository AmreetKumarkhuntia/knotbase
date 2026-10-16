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
-- shortcut: no index on edge.parent_id; add CREATE INDEX when a level has thousands of edges.
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

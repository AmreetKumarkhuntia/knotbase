-- Seed an empty DB once. FKs on (the sqlite3 CLI defaults them off) and all-or-nothing, so a second run fails and changes nothing.
.bail on
PRAGMA foreign_keys = ON;
BEGIN;
INSERT INTO node (id, parent_id, name, kind, x, y) VALUES
  (2, 1, 'Web App',  'frontend', 0,   100),
  (3, 1, 'API',      'backend',  300, 100),
  (4, 1, 'Postgres', 'database', 600, 100),
  (5, 1, 'Stripe',   'external', 600, 300);
INSERT INTO edge (parent_id, source_id, target_id, label) VALUES (1, 2, 3, 'HTTPS/JSON'), (1, 3, 4, 'sqlx'), (1, 3, 5, 'Payments API');
INSERT INTO node (id, parent_id, name, kind, doc) VALUES
  (6, 3, 'Payment Flow', 'flow', '# Payment Flow

Creates a Stripe PaymentIntent, then records the order.');
INSERT INTO node (id, parent_id, name, kind, x, y) VALUES (7, 6, 'POST /checkout', 'step', 0, 100), (8, 6, 'Stripe charge', 'step', 300, 100);
INSERT INTO edge (parent_id, source_id, target_id, label) VALUES (6, 7, 8, 'create PaymentIntent');
INSERT INTO item (node_id, text, done) VALUES (6, 'Document the retry mechanism', 0), (3, 'Add health endpoint', 1);
COMMIT;

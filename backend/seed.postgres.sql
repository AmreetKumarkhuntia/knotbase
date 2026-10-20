-- Seed an empty Postgres DB once, as one transaction so a rerun fails on id 2 and changes nothing:
--   psql -1 -v ON_ERROR_STOP=1 -d knotbase -f seed.postgres.sql
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
INSERT INTO item (node_id, text, done) VALUES (6, 'Document the retry mechanism', false), (3, 'Add health endpoint', true);
SELECT setval(pg_get_serial_sequence('node', 'id'), (SELECT max(id) FROM node));  -- explicit ids above do not advance the identity

#!/bin/sh
# Dev front door is Vite on :5025; it proxies /api to the Rust server on :3000 (PORT=<n> moves the Rust port and the proxy; both children inherit the environment).
cd "$(dirname "$0")"
# run from a terminal: kill 0 signals the whole process group, so launched via sh -c or an IDE task it can kill the launcher too.
trap 'kill 0' EXIT
(cd backend && cargo run) &
(cd frontend && npm run dev)

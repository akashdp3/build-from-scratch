# URL Shortener Playground

## Prerequisites

- Rust toolchain (stable) with `cargo`
- Node.js >= 20.19 (or Node 22.12+) and npm
- Frontend dependencies installed once via `npm install` inside `frontend/`

## Running backend + frontend together

1. Make the helper script executable (one-time):
   ```bash
   chmod +x dev.sh
   ```
2. Start both services from the repo root:
   ```bash
   ./dev.sh
   ```
   - Backend: runs `cargo run` in `backend/`
   - Frontend: runs `npm run dev` in `frontend/`
3. Stop everything with `Ctrl+C`. The script cleans up both child processes.

## Individual commands

- Backend only: `cargo run --manifest-path backend/Cargo.toml`
- Frontend only: `cd frontend && npm run dev`

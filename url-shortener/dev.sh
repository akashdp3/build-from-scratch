#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_PID=""
FRONTEND_PID=""

cleanup() {
    status=$?
    if [[ -n "${BACKEND_PID}" ]]; then
        kill "${BACKEND_PID}" >/dev/null 2>&1 || true
    fi
    if [[ -n "${FRONTEND_PID}" ]]; then
        kill "${FRONTEND_PID}" >/dev/null 2>&1 || true
    fi
    wait >/dev/null 2>&1 || true
    exit ${status}
}

trap cleanup EXIT INT TERM

(
    cd "${ROOT_DIR}/backend"
    cargo run
) &
BACKEND_PID=$!

echo "Backend running (pid=${BACKEND_PID})"

(
    cd "${ROOT_DIR}/frontend"
    npm run dev
) &
FRONTEND_PID=$!

echo "Frontend running (pid=${FRONTEND_PID})"

wait -n "${BACKEND_PID}" "${FRONTEND_PID}"

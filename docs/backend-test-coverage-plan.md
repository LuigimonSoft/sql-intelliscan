# Backend and frontend tests with 80% CI coverage

This project validates frontend and backend line coverage in CI with `cargo llvm-cov` and `scripts/check-coverage.py`.

## What CI enforces today

The workflow computes frontend line coverage with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --no-report --lib --test frontend`
- `cargo llvm-cov --workspace --no-run --json > frontend-coverage.json`

It filters coverage to `src`, excludes `src/main.rs`, and fails below `80%`.

The workflow computes backend line coverage with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --no-report --test backend_contracts`
- `cargo llvm-cov --workspace --no-run --json > backend-coverage.json`

It filters coverage to `src-tauri/src`, excludes `src-tauri/src/main.rs`, and fails below `80%`.

Reference: `.github/workflows/buildtest.yml` (steps **Frontend coverage** and **Backend coverage**).

## Current local verification (2026-04-25)

Measured locally with the same metrics used by CI:

- **Frontend coverage**: `89.13%` (41/46)
- **Backend coverage**: `84.48%` (49/58)

So today both ends pass the CI requirement (`>= 80%`).

## Why this is dynamic now

- Backend tests are authored under `tests/backend`.
- `src-tauri/build.rs` auto-discovers `*.rs` under `tests/backend` and generates include modules for `src-tauri/tests/backend_contracts.rs`.
- Adding a new backend test file under `tests/backend` no longer requires changes to `Cargo.toml` or workflow file lists.
- The generated Tauri/bootstrap `main.rs` files are excluded from coverage gates; behavior is checked through the library entry points.

## Suggested local commands

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --no-report --lib --test frontend
cargo llvm-cov --workspace --no-run --json > frontend-coverage.json
python3 ./scripts/check-coverage.py \
  --report frontend-coverage.json \
  --root src \
  --exclude src/main.rs \
  --label Frontend \
  --min 80

cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --no-report --test backend_contracts
cargo llvm-cov --workspace --no-run --json > backend-coverage.json
python3 ./scripts/check-coverage.py \
  --report backend-coverage.json \
  --root src-tauri/src \
  --exclude src-tauri/src/main.rs \
  --label Backend \
  --min 80
```

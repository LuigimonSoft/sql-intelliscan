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

The workflow also runs each architectural layer crate independently:

- `cargo test -p sql-intelliscan-common`
- `cargo test -p sql-intelliscan-repository`
- `cargo test -p sql-intelliscan-services`

Each layer crate has its own `tests` folder and its own coverage gate:

- `crates/sql-intelliscan-common/src` must be at least `80%`.
- `crates/sql-intelliscan-repository/src` must be at least `80%`.
- `crates/sql-intelliscan-services/src` must be at least `80%`.

Reference: `.github/workflows/buildtest.yml` (steps **Frontend coverage**, **Backend coverage**, **Crate layer tests**, and **Crate layer coverage**).

## Current local verification (2026-04-25)

Measured locally with the same metrics used by CI:

- **Frontend coverage**: `89.13%` (41/46)
- **Backend coverage**: `84.48%` (49/58)
- **Common crate coverage**: `100.00%` (3/3)
- **Repository crate coverage**: `100.00%` (3/3)
- **Services crate coverage**: `100.00%` (8/8)

So today the frontend, backend, and each layer crate pass the CI requirement (`>= 80%`).

## Why this is dynamic now

- Backend tests are authored under `tests/backend`.
- `src-tauri/build.rs` auto-discovers `*.rs` under `tests/backend` and generates include modules for `src-tauri/tests/backend_contracts.rs`.
- Adding a new backend test file under `tests/backend` no longer requires changes to `Cargo.toml` or workflow file lists.
- Layer crate tests are authored under each crate's own `tests` folder and run independently from the Tauri backend/frontend harnesses.
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

cargo test -p sql-intelliscan-common
cargo test -p sql-intelliscan-repository
cargo test -p sql-intelliscan-services

cargo llvm-cov clean --workspace
cargo llvm-cov -p sql-intelliscan-common --no-report
cargo llvm-cov -p sql-intelliscan-common --no-run --json > common-coverage.json
python3 ./scripts/check-coverage.py \
  --report common-coverage.json \
  --root crates/sql-intelliscan-common/src \
  --label Common \
  --min 80

cargo llvm-cov clean --workspace
cargo llvm-cov -p sql-intelliscan-repository --no-report
cargo llvm-cov -p sql-intelliscan-repository --no-run --json > repository-coverage.json
python3 ./scripts/check-coverage.py \
  --report repository-coverage.json \
  --root crates/sql-intelliscan-repository/src \
  --label Repository \
  --min 80

cargo llvm-cov clean --workspace
cargo llvm-cov -p sql-intelliscan-services --no-report
cargo llvm-cov -p sql-intelliscan-services --no-run --json > services-coverage.json
python3 ./scripts/check-coverage.py \
  --report services-coverage.json \
  --root crates/sql-intelliscan-services/src \
  --label Services \
  --min 80
```

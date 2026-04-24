# Backend unit tests (Tauro/Tauri) and 80% CI coverage

This project validates backend coverage in CI using only backend contract tests from `tests/backend` and backend source files under `src-tauri/src`.

## What CI enforces today

The workflow computes backend line coverage with:

- `cargo llvm-cov clean --workspace` + two backend-only test runs (`--test backend` and `--test backend_contracts`) + `cargo llvm-cov --workspace --no-run --json`

Then it dynamically filters coverage entries to files under `src-tauri/src` and fails when combined line coverage is below `80.0%`.

Reference: `.github/workflows/buildtest.yml` (step **Backend coverage**).

## Current local verification (2026-04-24)

Measured locally with the same backend metric used by CI:

- `src-tauri/src/lib.rs`: `77.78%` (28/36)
- `src-tauri/src/main.rs`: `87.50%` (21/24)
- **Combined backend coverage**: `81.67%` (49/60)

So today the backend passes the CI requirement (`>= 80%`), with a margin of `+1.67pp`.

## Why this is dynamic now

- Backend tests are authored under `tests/backend`.
- `src-tauri/build.rs` auto-discovers `*.rs` under `tests/backend` and generates include modules for `src-tauri/tests/backend_contracts.rs`.
- Adding a new backend test file under `tests/backend` no longer requires changes to `Cargo.toml` or workflow file lists.

## Suggested local command for backend-only gate

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --no-report --test backend
cargo llvm-cov --workspace --no-report --test backend_contracts
cargo llvm-cov --workspace --no-run --json > backend-coverage.json
python3 - <<'PY'
import json
import pathlib
import sys

report = json.loads(pathlib.Path("backend-coverage.json").read_text())
files = report["data"][0]["files"]
backend_root = pathlib.Path("src-tauri/src").resolve()

backend = []
for f in files:
    file_path = pathlib.Path(f["filename"]).resolve()
    if backend_root in file_path.parents:
        backend.append(f)

count = sum(f["summary"]["lines"]["count"] for f in backend)
covered = sum(f["summary"]["lines"]["covered"] for f in backend)
coverage = (covered / count) * 100 if count else 0.0
print(f"Backend coverage: {coverage:.2f}% ({covered}/{count})")
if coverage < 80.0:
    sys.exit("Backend coverage is below 80%")
PY
```

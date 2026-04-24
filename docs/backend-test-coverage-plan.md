# Backend unit tests (Tauro/Tauri) and 80% CI coverage

This project validates backend coverage in CI from `src-tauri/src/lib.rs` and `src-tauri/src/main.rs` only.

## What CI enforces today

The workflow computes backend line coverage with `cargo llvm-cov --workspace --tests --json` and fails if the combined coverage for:

- `src-tauri/src/lib.rs`
- `src-tauri/src/main.rs`

is below `80.0%`.

Reference: `.github/workflows/buildtest.yml` (step **Backend coverage**).

## Current local verification (2026-04-24)

Measured locally with the same backend metric used by CI:

- `src-tauri/src/lib.rs`: `77.78%` (28/36)
- `src-tauri/src/main.rs`: `87.50%` (21/24)
- **Combined backend coverage**: `81.67%` (49/60)

So today the backend already passes the CI requirement (`>= 80%`), but with limited margin (`+1.67pp`).

## Where coverage is missing right now

From `backend-coverage.json`, uncovered backend lines are concentrated in:

- `src-tauri/src/lib.rs`: lines around command wiring (`greet_command`) and runtime launch (`run_builder`), which are intentionally hard to unit-test without booting a Tauri runtime.
- `src-tauri/src/main.rs`: `main()` bootstrap lines.

## How to add safer tests to keep us above 80%

1. Prioritize tests for pure/injectable seams already present:
   - `greet`
   - `run_with`
   - `set_run_hooks` / `reset_run_hooks`
   - `run_application`
   - `set_backend_runner` / `reset_backend_runner` / `start_application`

2. Avoid tests that launch the real UI/runtime in unit tests. Keep using injected builders/runners (as done in `tests/backend/*`).

3. Add tests that verify hook lifecycle and state isolation:
   - override -> execute -> reset -> override again
   - repeated reset calls are safe
   - override replacement uses the latest injected function

4. Keep backend coverage checks reproducible locally by running the same command CI runs.

## Suggested local command for backend-only gate

```bash
cargo llvm-cov --workspace --tests --json > backend-coverage.json
python3 - <<'PY'
import json, pathlib, sys
text = pathlib.Path("backend-coverage.json").read_text()
report = json.loads(text[text.find("{"):])
files = report["data"][0]["files"]
backend = [
    f for f in files
    if f["filename"].endswith("src-tauri/src/lib.rs")
    or f["filename"].endswith("src-tauri/src/main.rs")
]
count = sum(f["summary"]["lines"]["count"] for f in backend)
covered = sum(f["summary"]["lines"]["covered"] for f in backend)
coverage = (covered / count) * 100 if count else 0.0
print(f"Backend coverage: {coverage:.2f}% ({covered}/{count})")
if coverage < 80.0:
    sys.exit("Backend coverage is below 80%")
PY
```

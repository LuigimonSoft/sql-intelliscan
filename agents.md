# Testing Rules (Tauri Project)

## 0. General Behavior
- Always read this file before modifying code or tests.
- After any code or test change, re-run all tests and verify coverage.
- Coverage must never decrease. Prefer increasing it, especially in modified areas.

---

## 1. Test Structure

### 1.1 Global Test Directories
- Backend tests must be located in: `tests/backend`
- Frontend tests must be located in: `tests/frontend`

### 1.2 Harness Files
- The following files are allowed to exist without direct tests:
  - `tests/backend.rs`
  - `tests/frontend.rs`
- Their only purpose is to automatically load test files from their respective folders.

### 1.3 Auto-discovery Rules
- All test files inside:
  - `tests/backend`
  - `tests/frontend`
  must be automatically discovered.
- Do NOT add new `[[test]]` entries in `Cargo.toml`.

---

## 2. Test File Conventions

### 2.1 File Naming
- Each tested file MUST have a corresponding test file.
- The test file MUST:
  - Match the base name of the source file
  - Be located in the appropriate test directory

**Examples:**
- `src/foo.rs` → `tests/backend/foo.rs`
- `frontend/bar.ts` → `tests/frontend/bar.ts`

---

## 3. Test Design

### 3.1 Naming Convention
- All test functions MUST follow:
```
Given<Context>_When<Action>_Then<Result>_Should<Expectation>
```

### 3.2 Isolation
- Always isolate the unit under test.
- Use mocks, stubs, or test doubles whenever possible.

### 3.3 Frontend Rules
- Frontend tests MUST mock all backend calls.
- No real external dependencies are allowed in frontend tests.

---

## 4. Coverage Requirements

### 4.1 Backend
- Minimum required coverage: **80%**
- Preferred target: **90%+**

### 4.2 Crates
- Each crate MUST:
  - Have its own `tests/` directory
  - Maintain independent tests and coverage
  - Crate tests:
    - Are independent from `tests/backend` and `tests/frontend`
    - Must be executed and validated separately
    - Must meet at least **80% coverage**

---

## 5. Enforcement Rules
- Any new or modified code MUST include corresponding tests.
- If no test exists, it MUST be created.
- If coverage drops below thresholds, tests MUST be added or improved.
- Never leave untested logic.

---

## 6. Priority Rules
When rules conflict, follow this priority:
1. Test isolation and correctness
2. Coverage requirements
3. Naming and structure conventions
4. Auto-discovery constraints

---

## 7. Mocks and Stubs
- Use mocks to simulate lower layers and isolate the unit under test.
- Frontend tests MUST use mocks to simulate backend calls and avoid external dependencies.
- Frontend tests MUST use mocks to simulate Tauri commands and avoid external dependencies.
- Backend tests MAY use mocks to simulate external services or dependencies when necessary to isolate the unit under test.

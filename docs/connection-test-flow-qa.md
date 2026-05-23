# SQL Server Connection Test Flow QA

This checklist validates issue #69: the complete SQL Server connection test path from the Leptos UI to the existing Tauri `test_connection` command and back to safe UI feedback.

## Scope

- User enters SQL Server connection values in the Leptos form.
- The form builds a `ConnectionTestRequest` only after client-side validation passes.
- The parent connection view calls the frontend connection service.
- The service invokes the existing Tauri `test_connection` command.
- The UI shows loading, success, and safe error states.
- Credentials are not displayed, logged, persisted, or assembled into a frontend connection string.

## Automated Validation

Run the same core commands used by CI before manual QA:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets
cargo test --workspace
bash ./scripts/wasm-test-local.sh
```

Run coverage checks from the CI workflow when `cargo-llvm-cov` is installed:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --json --output-path frontend-coverage.json --lib --test frontend
python3 ./scripts/check-coverage.py --report frontend-coverage.json --root src --exclude src/main.rs --label Frontend --min 80
```

## Boundary Checks

```bash
rg "invoke" src/components
rg "invoke" src/services
rg "localStorage|sessionStorage" src
rg "connection_string|connectionString" src
rg "password" src/components
rg "stack|trace|backtrace" src/components
```

Expected results:

- No component calls Tauri `invoke` directly.
- `test_connection` is invoked only from `src/services/tauri_client.rs`.
- No frontend code assembles or displays a full connection string.
- Password appears only in the connection form input and validation text.
- No stack traces or raw backend driver details are rendered in components.
- Only theme preference storage is allowed; credentials are not persisted.

## Manual QA

### Valid Connection

- Start a reachable SQL Server instance.
- Open the connection test UI.
- Enter valid host, port, database, username, and password.
- Click **Test Connection**.
- Confirm the submit action changes to the loading state and duplicate clicks are ignored.
- Confirm success feedback appears.
- Confirm safe metadata appears only when returned by the backend.
- Confirm password and full connection string are not displayed.

### Invalid Credentials

- Enter a reachable host, port, and database.
- Enter invalid username or password.
- Click **Test Connection**.
- Confirm loading appears first.
- Confirm the UI shows the friendly authentication failure message.
- Confirm raw SQL driver errors, password, and stack traces are not displayed.
- Submit again with corrected credentials and confirm the new result replaces the old one.

### Invalid Host

- Enter an unreachable host.
- Click **Test Connection**.
- Confirm loading appears first.
- Confirm the UI shows the friendly connection failure message.
- Confirm no raw driver error or full connection string is displayed.

### Timeout

- Enter values that force a timeout.
- Click **Test Connection**.
- Confirm loading appears while the request runs.
- Confirm the timeout message is safe and user-friendly.
- Confirm the submit action becomes usable again after failure.

### Client-Side Validation

- Leave host empty and submit.
- Repeat with an invalid port, empty database, empty username, empty password, invalid timeout, and whitespace-only application name.
- Confirm each invalid submission shows validation feedback.
- Confirm invalid submissions do not call the backend command.

### Retry

- Run a successful test, then submit again.
- Run a failed test, then submit again.
- Confirm each retry enters loading state, hides stale feedback, and replaces the previous result.
- Confirm duplicate submissions remain blocked while loading.

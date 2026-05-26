# Backend Logging

## Decision

The backend logging stack is:

- `tracing` for structured log emission in `src-tauri`, `services`, and `repository`.
- `tracing-subscriber` for the single global subscriber initialized by `src-tauri`.
- No `tracing-appender` for this story. File logging is deferred until a requirement exists.

`services` and `repository` must only emit logs with `tracing` macros. They must not initialize a global subscriber or depend on `tracing-subscriber`.

## Initialization

Logging is initialized once from `src-tauri` through `init_logging`.

Runtime environment detection is centralized in `src-tauri` and reads
`SQL_INTELLISCAN_ENV`.

Supported environments and default filters:

- `development`: `debug`
- `test`: `warn`
- `staging`: `info`
- `production`: `warn`

If `SQL_INTELLISCAN_ENV` is missing, the app assumes `development` for local
developer ergonomics. If it is invalid or not valid Unicode, the app falls back
to `production`.

The selected environment default can be overridden. Precedence is:

1. `RUST_LOG`
2. `SQL_INTELLISCAN_LOG_LEVEL`
3. environment default

Examples:

```bash
SQL_INTELLISCAN_ENV=development
SQL_INTELLISCAN_ENV=production
RUST_LOG=debug
RUST_LOG=sql_intelliscan_lib::logging=debug,warn
SQL_INTELLISCAN_LOG_LEVEL=info
```

By default, `tracing` uses the Rust module path as the event target, such as `sql_intelliscan_lib::logging::logging_config`. Use `target: "sql_intelliscan::..."` in macros when you want logs to match the stable application targets shown below.

Invalid filter values fall back to the current environment default without
crashing. If another embedded component has already installed a global
subscriber, backend startup treats that as non-fatal and continues.

## Conventions

Use structured fields and stable targets:

```rust
use tracing::{debug, error, info, warn};

info!(target: "sql_intelliscan::startup", "Application startup started");
debug!(target: "sql_intelliscan::connection", host = %safe_host, port = port, "Starting connection validation");
warn!(target: "sql_intelliscan::connection", error_code = "VALIDATION_FAILED", "Connection validation failed");
error!(target: "sql_intelliscan::repository", error_code = "CONNECTION_FAILED", "Repository operation failed");
```

Level guidance:

- `trace`: deep diagnostics, never credentials or raw payloads.
- `debug`: developer diagnostics with non-sensitive fields.
- `info`: normal lifecycle events.
- `warn`: recoverable or expected failures.
- `error`: unexpected failures.

## Sensitive Data

Never log passwords, full connection strings, raw credential payloads, secret environment variable values, authentication tokens, private keys, or raw request objects that contain secrets.

Safe fields include operation name, layer, result category, stable error code, duration, host, port, and database when they are considered non-sensitive.

Safe example:

```rust
info!(
    target: "sql_intelliscan::connection",
    host = %safe_host,
    port = port,
    database = %database,
    "Starting SQL Server connection test"
);
```

Unsafe examples:

```rust
debug!("Connection request: {:?}", request);
error!("Connection failed using password: {}", password);
info!("Connection string: {}", connection_string);
```

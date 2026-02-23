AGENTS
======
always use just command to verify etc
You find documentation on how to to do:
- styling for the html inside skills/t.md
- for htmx inside skills/t.md


This file contains essential commands and code-style guidance for autonomous agents working in this repository.

Always prefer using `just` recipes when possible — they wrap common flows (build, lint, test, docker, hurl).

Quick reference
---------------
- **Run app (dev):** `just run` or `cargo run`
- **Build:** `cargo build --release` (or `just run` for dev runs)
- **Format (fix):** `just fmt` or `cargo fmt`
- **Format (check):** `cargo fmt -- --check` (present in `just verify`)
- **Lint:** `cargo clippy` (and `cargo clippy --fix --allow-dirty` in `just fmt`)
- **Full verification:** `just verify` (runs fmt check, check, clippy, tests, docker acceptance tests)
- **Unit tests:** `cargo test`
- **Run a single Rust test:** `cargo test <TEST_NAME>` or `cargo test --test <INTEGRATION_TEST_NAME>`; for exact-match use `-- --exact` or `-- --nocapture` to see output
- **Acceptance tests (HTTP):** `just hurl test` or `just tests` via `tests/hurl.just`; run a single hurl file with `hurl tests/<file>.hurl`
- **Start dependencies / docker stack:** `just docker run` / `just docker stop`

Repository commands (examples)
------------------------------
Use these exact commands where appropriate. Prefer `just` when available.

- Build and run locally (dev):

```sh
just run
```

- Full verification (format-check, clippy, tests, acceptance tests):

```sh
just verify
```

- Format + attempt autofixes:

```sh
just fmt
# or
cargo fmt && cargo clippy --fix --allow-dirty
```

- Run only unit tests or a single test:

```sh
cargo test                 # all tests
cargo test my_test_name    # runs tests with "my_test_name" in the name
cargo test my_test_name -- --exact    # run exact named test
cargo test --test my_integration    # run specific integration test file
```

- Run HTTP acceptance tests (hurl):

```sh
just hurl test             # runs the suite via tests/hurl.just
hurl tests/health.hurl     # run a single hurl file (requires hurl installed)
```

Project layout notes
---------------------
- Rust project using Cargo. See `Cargo.toml` for deps (actix-web, maud, serde_json, env_logger).
- `justfile` contains convenient recipes: `run`, `verify`, `fmt`, and a `tests/hurl.just` module.
- Acceptance tests live under `tests/*.hurl` and are orchestrated through `tests/hurl.just`.
- Docker helper recipes live in `docker.just` and are referenced from the top-level `justfile`.

Code style & conventions
------------------------
Follow idiomatic Rust conventions; the repo enables clippy warnings (see `main.rs` top level attribute). Below are repository-specific conventions agents should follow when editing or adding code.

- Formatting
  - Run `cargo fmt` before committing changes. Use `cargo fmt -- --check` in CI and in `just verify`.
  - Keep line lengths reasonable (~100 columns); rustfmt will handle most details.

- Linting
  - Run `cargo clippy` and fix warnings. The project sets `#![warn(clippy::all, clippy::pedantic)]` in `src/main.rs` — address pedantic lints when practical.
  - If you intentionally silence a clippy lint, add a short comment explaining why and scope it narrowly (function/module).

- Imports & module order
  - Group imports by origin: standard library first (`std::`), external crates second, local crate modules last.
  - Use grouped imports where it improves readability: `use actix_web::{App, HttpServer, web};` (as already used).
  - Keep `mod` declarations near the top of `main.rs` after crate-level attributes and use statements.

- Naming
  - Types and structs: CamelCase (e.g., `KeyValueStore`, `Server`).
  - Functions, variables, module files: snake_case (e.g., `get_kv`, `kv_store.rs`).
  - Constants: SCREAMING_SNAKE_CASE (e.g., `TCSS`, `HTMX`) — repo already uses these conventions.

- Error handling
  - Prefer propagating errors (the `?` operator) in non-handler code; in HTTP handlers prefer returning appropriate `HttpResponse` or `Result<T, actix_web::Error>` instead of panicking.
  - Avoid `unwrap()` and `expect()` in production paths. If `unwrap()` is used for truly impossible cases, include a short explanatory comment.
  - When interacting with locks (`Mutex`), handle poisoning appropriately. Currently code uses `lock().unwrap()`; prefer `match store.lock()` and map poisoning to a safe error path or `HttpResponse::InternalServerError()` in handlers.

- Pattern matching & early returns
  - Use `let Some(x) = … else { ... }` for concise early returns (used in this repo). Prefer explicit matches when you need to log or map errors.

- Thread-safety & concurrency
  - Shared state is `web::Data<T>` (actix-web). Keep interior mutability minimal and guard with `Mutex` or better primitives as needed. Consider `RwLock` when concurrent reads dominate.

- Types & API surface
  - Keep handler signatures small and explicit (the repo commonly uses `req: HttpRequest, key: web::Path<String>, store: web::Data<KeyValueStore>`).
  - For public helper functions prefer returning concrete `Result<T, E>` where callers can decide how to convert to HTTP responses.

- Tests
  - Unit tests should live in `src` files in `#[cfg(test)]` modules; integration tests in `tests/` directory.
  - For HTTP acceptance tests, use the provided `.hurl` files and the `just` recipes. Ensure services are running (`just docker run` may be needed). Use `just hurl test` to run the suite.

- Maud templates
  - Keep maud markup functions pure where possible and return `maud::Markup` directly (see `src/view/mod.rs`). Avoid heavy business logic in template code — prepare data in the handler.

- Logging
  - Use `env_logger` + `log` macros (`info!`, `debug!`, `error!`). Configure verbosity via `LOG_LEVEL` env var — `config::from_env()` reads this in the project.

- Documentation & comments
  - Write short doc comments (`///`) for public functions and types. Inline comments are allowed when explaining non-obvious decisions. Do not over-comment trivial code.

Cursor / Copilot rules
----------------------
- Cursor rules: none found under `.cursor/rules/` or `.cursorrules` in repository root.
- GitHub Copilot instructions: none found at `.github/copilot-instructions.md`.

If you find repository-specific Cursor or Copilot rules later, include them in this file and follow them.

Commit & CI guidance for agents
-------------------------------
- Run `just verify` locally before creating a PR. This runs formatting checks, clippy, unit tests, and acceptance tests (docker + hurl).
- Make minimal commits with focused changes and a clear commit message describing "why" rather than "what".

Practical examples
-------------------
- Run a single handler's unit test (example):

```sh
# run tests containing `kv_get` in their name
cargo test kv_get

# run exact integration test file (if present)
```

- Run a single hurl test file (example):

```sh
hurl tests/kv_plaintext_string.hurl --test --very-verbose --variables-file tests/variables
```

When in doubt
------------
- Prefer using `just` recipes. They encode project intent and reduce mistakes.
- If a change affects runtime behaviour, add or update tests (unit or acceptance) and update `just verify` flows if needed.

Contact
-------
If the repository owner (or maintainers) provided project guidelines elsewhere (README, CONTRIBUTING, or CI), follow those first and update this file to match.

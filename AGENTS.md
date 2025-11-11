# Repository Guidelines

## Project Structure & Module Organization
Core library code lives in `src/`, with `metrics.rs` defining boxed metric abstractions and `registry/` holding backend adapters (one module per supported crate version). Shared helpers sit in `src/utils.rs`, and doctest-friendly fixtures live in `src/test_utils.rs`. Reference runnable code sits in `examples/basic`, while binary artifacts (logos, etc.) are under `etc/`. Workspace metadata such as `Cargo.toml`, `Makefile`, and `CHANGELOG.md` stay at the root; keep new assets equally discoverable.

## Build, Test, and Development Commands
Use the provided Makefile targets to keep workflows consistent:
```bash
make check      # typos + cargo sort/fmt/clippy over all targets/features
make test       # cargo nextest, doc tests, and docs.rs warning gate
make examples   # runs cargo run --example basic --features prometheus_0_14
make ffmt       # nightly rustfmt using rustfmt.nightly.toml
make machete    # verifies feature-flag completeness via cargo machete
```
Run `cargo fmt --all` or `cargo +nightly fmt --all -- --config-path rustfmt.nightly.toml` before pushing.

## Coding Style & Naming Conventions
Follow idiomatic Rust 2021 style enforced by rustfmt. Prefer descriptive module paths (e.g., `registry::prometheus_0_14`) and snake_case function names; reserve UpperCamelCase for types like `BoxedRegistry`. Keep feature names synchronized with backend crate/major versions (`prometheus_0_14`, `opentelemetry_0_31`). Document new public APIs with `///` comments so doctests remain valuable.

## Testing Guidelines
Favor `cargo nextest` for fast iterations and keep unit tests colocated with implementation modules using `#[cfg(test)]`. Add doctests whenever showcasing registry wiring so `cargo test --doc --all-features` continues to validate the README snippets. When adding a backend feature, cover both the happy path and the noop registry in tests, and ensure docs build cleanly with `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --all-features --no-deps`.

## Commit & Pull Request Guidelines
Recent history mixes conventional prefixes (`feat:`, `fix:`, `chore:`) with imperative summaries (e.g., `Improve docs for docs.rs`). Match that style, reference issues/PRs in parentheses when relevant, and keep each commit focused on one concern. PRs should describe the motivation, list new commands or features, call out feature flags touched, and include test evidence (e.g., `make check && make test`). Add screenshots or metrics dumps only when UI/telemetry output changes. Ensure CI-critical commands pass locally before requesting review.

## Feature Flag Tips
Every backend addition should expose a clearly named Cargo feature and register itself through `registry::` modules. Document the feature in `README.md`, gate example code (as with `--features prometheus_0_14`), and keep `Cargo.lock` updated when adding new optional dependencies.

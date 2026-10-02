# Agent guide

## Map
- `rust/`: Cargo workspace; Rust 2024, documented/CI minimum 1.85.
  `lib/known-errors/src/sysexits.rs` holds the exit-code API and integrations;
  `lib.rs` gates modules; `features.rs` exports `FEATURES`.
- `python/`: `uv_build` package, Python >=3.10 (development pin: 3.13);
  `src/known_errors/__init__.py` is a scaffold.
- `ruby/`: Ruby >=3.2 gem; `lib/known-errors.rb` loads `lib/known/errors.rb`;
  `Known::Errors` is a scaffold. `dart/` is empty.

## Change rules
- Stay inside this repository. Inspect the diff; preserve existing edits.
- Keep Rust `#![no_std]` and `#![deny(unsafe_code)]`. Gate standard-library APIs
  with `#[cfg(feature = "std")]`; keep integrations optional.
- Features: `default = ["all", "std"]`, `all = ["sysexits"]`.
  `all` excludes integrations; `--all-features` includes them. `serde` currently
  aliases `serde-json`; it does not implement serialization traits.
- Preserve standard `EX_*` names and values (0, 64–78), symbolic `Display`,
  and name/decimal parsing. Validate integer inputs before narrowing casts.
- Use `$crate::...` paths for crate-owned items in exported macros.
- Document every new/changed public symbol in rustdoc: behavior, feature gates,
  conversion policies, runnable examples. README additions require clear value.
- Add behavioral regression tests for fixes, including affected feature gates;
  Rust integration tests belong in `rust/lib/known-errors/tests/`.
- Versions are separate: root `VERSION` and `rust/Cargo.toml` track Rust;
  Python uses `python/pyproject.toml`, Ruby uses `ruby/VERSION`.
  Rust/Python `CHANGES.md` and `UNLICENSE` are symlinks to root files.

## Checks
For Rust changes, run **from `rust/`** (no root Cargo manifest):

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace --no-default-features
cargo check --workspace --no-default-features --features all
cargo check --workspace --no-default-features --features std
cargo check --workspace --all-features
cargo clippy --workspace --all-targets -- -D warnings
```

- Check changed integrations individually, including supported no-std builds.
  `cargo test --workspace` includes doctests; `--tests` alone skips them.
- Known failures: `gofer` lacks required `std`; packaged doctests reference a
  workspace-only README.
  Recheck/report failures; do not suppress them to pass checks.
- Stable rustfmt warns about nightly-only `imports_granularity`.
- Python smoke check, from `python/` with Python >=3.10:
  `PYTHONPATH=src python3 -B -c 'import known_errors'`.
- Ruby smoke check, from `ruby/`: `ruby -Ilib -rknown-errors -e 'p Known::Errors'`.
- Rust error-stack integration tests cover builds with and without `std`.
  Python and Ruby imports/builds are only smoke checks.
- Packaging changes: `cargo package` in `rust/`, `uv build` in `python/`,
  `gem build known-errors.gemspec` in `ruby/`. Test built artifacts too.

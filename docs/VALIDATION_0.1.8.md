# Rust Industries 0.1.8 package validation record

Date: 2026-11-27

## Checks performed on the supplied HMOD

- Archive opened successfully as a ZIP-based HMOD package.
- 338 entries; 183,166 bytes uncompressed.
- 243 JSON files parsed successfully.
- `Cargo.toml` and `horizon-mod.toml` parsed successfully.
- No absolute paths, traversal entries, backslashes, or symlink entries.
- No nested list/table values under `[native]`; `native.game`, `native.horizon`, and `native.mixins` are string values.
- The only Cargo dependency is the Horizon Mod API path dependency.
- No Cargo build script, build dependencies, or custom feature table.
- SHA-256: `b04d0ed41d1ef3856fa60c9ab5a32b3edd0006e71e03df6841ee8f4cbb4db898`.

## Not performed

- Rust compiler was not available in the preparation environment; `cargo check` and `cargo test` were not run.
- No Starling remote Native Build job was launched.
- No Android in-game tests were performed.

Static package validation is not proof of successful compilation or runtime compatibility.

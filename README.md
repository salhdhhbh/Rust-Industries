# Rust Industries

Industrial machines, processing, crops, and energy infrastructure for Horizon / Starling on Android.

## Current development version

- Version: `0.1.8` — Native energy-network preview
- Mod ID: `rust_industries`
- Declared game: `26.3`
- Declared Horizon Mod API: `0.2`
- Native source snapshot: [`mod/v0.1.8-native/`](mod/v0.1.8-native/)
- Package notes and validation: [`releases/v0.1.8/README.md`](releases/v0.1.8/README.md)

Version 0.1.8 adds FE generation, storage, and cable transfer while retaining the mechanical content included in the supplied package. It is an experimental Native package, not a runtime-verified release.

## 0.1.8 additions

- Coal Generator: 20,000 FE capacity; 50 FE generated per active fuel tick.
- Basic Energy Cube: 4,000,000 FE bidirectional storage.
- Basic Universal Cable: 8,000 FE buffer and neighbour capability transfer.
- Crushing Wheel: 20,000 FE internal storage and 50 FE per processing tick; a 40-tick cycle consumes 2,000 FE.
- The 0.1.8 package includes the mechanical machines, Depot, crop families, mechanical components, assets, and data definitions described in its Persian guide.

Fuel durations and known limitations are documented in [`releases/v0.1.8/README.md`](releases/v0.1.8/README.md).

## Native Build compatibility notes

The package manifest declares only string-valued fields under `[native]`; it does not contain nested `[native.mixin]`, `[native.bridge]`, or `[native.module]` tables. Its Cargo manifest has no build script, build dependencies, custom features, or third-party crates; its only dependency is the Horizon Mod API path used by the package.

Those properties appear to satisfy the published basic eligibility checks for Starling's per-mod Native Build route. This is a static source inspection only: it is not proof that the remote build service will accept or successfully compile this version. If the launcher requests the whole-game Compile route, use the exact error returned by Starling rather than changing the manifest blindly.

The phone does not compile the Rust code locally. Starling sends the mod to its configured build service, and the resulting native library is returned by that service.

## Validation status

The supplied HMOD archive was inspected as a ZIP-based package:
- 338 entries; 183,166 bytes uncompressed.
- 243 JSON files parsed successfully.
- `Cargo.toml` and `horizon-mod.toml` parsed successfully.
- No absolute paths, path traversal entries, backslashes, or symlink entries were found.
- SHA-256: `b04d0ed41d1ef3856fa60c9ab5a32b3edd0006e71e03df6841ee8f4cbb4db898`.

Rust compilation and in-game validation have not yet been completed. Structural validation must not be treated as a successful Native Build.

## Historical versions

The existing 0.1.3 release and 0.1.5 Native source remain in their original paths. New work is kept in the versioned `v0.1.8-native` directory; no historical source snapshot is overwritten.

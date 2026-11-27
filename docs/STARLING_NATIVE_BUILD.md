# Starling Native Build compatibility notes — Rust Industries 0.1.8

Date checked: 2026-11-27

## Current package properties

The supplied `RustIndustries_0.1.8_NATIVE_ENERGY_MERGED.hmod` was inspected as a ZIP package.

- `horizon-mod.toml` contains a `[native]` table with plain string-valued entries: `game`, `horizon`, and `mixins`.
- There are no nested `[native.mixin]`, `[native.bridge]`, or `[native.module]` tables.
- `Cargo.toml` contains one dependency: the pinned `horizon-mod-api` path dependency.
- No `build.rs`, Cargo build-dependencies, or custom `[features]` table was found in the manifest/package.
- The package declares game `26.3`, API `0.2`, mod ID `rust_industries`, and side `both`.

These facts appear compatible with the per-mod Native Build v2 eligibility checks currently documented in Starling's build repository.

## Important distinction: per-mod build vs whole-game Compile

Starling's current `tools/android_native_sdk.py mods` path builds a native HMOD as a separate library when it has a `[native]` table but no nested `[native.*]` tables. The path also refuses build scripts, build-dependencies, custom Cargo features, and dependencies that are not the Horizon API or available to the game's build.

A mod with nested native tables such as mixins, bridges, or modules is directed to a staged/whole-game build route instead. The plain `mixins = "..."` string in this package is not itself a nested table according to the current checker.

Official references:
- Starling Native SDK implementation: https://github.com/starling-client/starling/blob/starling/tools/android_native_sdk.py
- Starling mod-build service and Compile v2 overview: https://github.com/starling-client/mod-builds/blob/main/README.md
- Per-mod build eligibility and refusal logic: https://github.com/starling-client/mod-builds/blob/main/scripts/mod_lib.py

## What this does and does not establish

This is a static compatibility assessment from the package and published source code. It does **not** prove that a remote build succeeds, that the installed Starling APK exposes the Compile action, or that the resulting library works in-game.

The remote builder compiles the mod code in a sandbox. A successful package check is not a substitute for a build result. If Starling reports a refusal or compiler error, preserve the complete error text and use that exact response to diagnose the next change; do not randomly modify the manifest.

## User-side test

1. Place the HMOD under Starling's `horizon-native-mods` directory.
2. Check that the installed Starling build supports the current per-mod Native Build v2 workflow.
3. Use the launcher/game's Compile action when it is present.
4. Record the build result and any exact error.

The compile service does the Rust compilation remotely; the Android phone does not compile this Rust project locally.

# Rust Industries

**Rust Industries** is a lightweight industrial and factory-themed content mod for Horizon Android. Its design goal is practical mechanical automation with a small client and world-performance footprint.

## Project status

- Current installable build: **0.1.3**
- Target milestone: **1.0.0**
- Mod format: Horizon `.hmod`, content API `0.2`
- Runtime validation: pending on the target Horizon Android build
- The `release/1.0.0` branch is a development and planning branch, not a published 1.0.0 release.

## What is in the current build

The 0.1.3 package contains industrial blocks and items, resource crops, models, blockstates, loot tables, language files, and a custom **Rust Industries** Creative tab. The Creative tab is already declared at:

`data/rust_industries/horizon/creative_tab/industry.json`

If the tab does not appear in-game, investigate mod loading, manifest/API compatibility, and the Horizon log before duplicating the tab definition.

## Design direction for 1.0.0

- A compact mechanical/factory identity: shafts, cogwheels, gearboxes, motors, belts/transport, processing machines, and simple production chains.
- Keep Create-like inspiration at the level of readable mechanical systems; do not copy Create's expensive visual simulation.
- Prefer static or low-state models, shared geometry, simple textures, and vanilla templates where they fit.
- Avoid always-running block entities, per-tick world scans, particle-heavy effects, animated textures, and large numbers of moving render objects.
- Separate visual content from actual machine behaviour. Content API 0.2 defines registries and assets; real processing, rotation, item transfer, and machine GUIs need a supported code/runtime implementation. Do not describe decorative blocks as working machines.

## Repository structure

- `mod/` — source package and development assets.
- `releases/v0.1.3/` — current installable release files.
- `docs/` — architecture, performance rules, and 1.0.0 acceptance plan.

## Install current version

1. Download the Android install ZIP from `releases/v0.1.3/`.
2. Extract it and import the included `.hmod` into Horizon.
3. Remove/disable older duplicate Rust Industries copies before testing.
4. Restart the game and check the Mods list and log if content is missing.

## Validation before a release

A package is not considered released merely because it is a valid ZIP. Each candidate must pass archive/JSON/schema checks and an in-game test on the intended Horizon Android version. See [the 1.0.0 plan](docs/V1_SPEC.md) and [performance rules](docs/PERFORMANCE.md).

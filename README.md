# Rust Industries

A Horizon Android mod focused on industrial machines, processing, resources, and automation.

## Current development version

- Version: `0.1.5`
- Target: Starling 0.9.0 / Horizon Mod API 0.2
- Native source: `mod/v0.1.5-native/`
- Status: source committed; native compilation and in-game validation are still pending.

## Repository layout

- `mod/` — mod source and resource files.
- `mod/v0.1.5-native/` — Rust Native source for the Crusher.
- `releases/v0.1.3/` — historical 0.1.3 release files (preserved).

## Build 0.1.5 Native

Open `mod/v0.1.5-native/` as a Native mod project in Starling/Horizon and run the launcher's Native Build for the target game instance. The Cargo dependency points to the pinned SDK at `.horizon/sdk/api/crates/horizon-mod-api`; the Horizon build tool fetches the SDK when it is missing.

The source adds a two-slot Crusher menu and server-side processing for iron, copper and gold inputs. Each supported input produces two matching dust after 40 ticks.

**Important:** this repository commit contains the Native source files, not a precompiled Android binary. Native compilation is performed by the launcher for the target instance. The source has not yet been compiled or runtime-tested, so do not treat this commit as a verified working build.

## Historical compatibility

Versions 0.1.3 FIXED and 0.1.4 COMPAT5_FIXED are preserved as historical baselines. New machine development continues on 0.1.5.

## Development notes

The `random_ticks` property was removed from the four crop block definitions because the Horizon error report showed that this property is not accepted by its `BlockProperties` schema.

This repository contains mod definitions and assets; it is not a standalone Android application.

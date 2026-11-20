# Rust Industries

A Horizon Android mod focused on industrial machines, processing, resources, and automation.

## Current version

- Version: `0.1.3`
- Target: Horizon Android
- Release status: structurally packaged; in-game runtime validation is still pending.

## Repository layout

- `mod/` — unpacked mod data and resource files used for development.
- `releases/v0.1.3/` — installable HMOD and Android installer ZIP.

## Install

1. Download `RustIndustries_Android_Install_v0.1.3_FIXED.zip` from `releases/v0.1.3/`.
2. Extract it.
3. Import `RustIndustries_0.1.3.hmod` into Horizon, replacing older Rust Industries copies.

## Development notes

The `random_ticks` property was removed from the four crop block definitions because the Horizon error report showed that this property is not accepted by its `BlockProperties` schema.

This repository contains JSON-based mod definitions and assets; it is not a standalone Android application.

## Compatibility

Designed for Horizon Android. Confirm compatibility against the exact Horizon build in-game before distributing a release.

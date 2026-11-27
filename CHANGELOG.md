# Changelog

## 0.1.8 — Native energy network preview
- Added Coal Generator, Basic Energy Cube, and Basic Universal Cable Native implementations.
- Added FE generation, bounded storage, cable buffering, and neighbour transfer using ENERGY capabilities.
- Connected Crushing Wheel processing to FE consumption: 50 FE per tick for 40 ticks (2,000 FE per cycle).
- Retained the mechanical machines, 9-slot Depot, processing definitions, crop families, textures, and mechanical parts included in the supplied 0.1.8 package.
- Added generator and powered-machine energy/status readouts.
- Documented remaining limitations: Millstone/Drill/Press/Mixer do not yet consume FE; the Energy Cube has no dedicated charge GUI; cable connection states and collision shapes are not dynamic; multi-branch transfer priority depends on neighbour iteration order.
- HMOD structure, JSON, and TOML parsing validated. Rust compilation and in-game testing remain pending.

## 0.1.5 — Native Crusher source
- Added the Native mod manifest pinned to the Starling 0.9.0 / Horizon 26.3 build.
- Added Rust source for Crusher block-entity processing and its two-slot screen.
- Added a 40-tick processing loop for iron, copper and gold inputs.
- Native source is committed under `mod/v0.1.5-native/`.
- Compilation and in-game runtime validation are still pending.

## 0.1.4 COMPAT5_FIXED
- Preserved as a historical compatibility baseline.

## 0.1.3 FIXED
- Removed unsupported `random_ticks` properties from four crop block definitions.
- Preserved the remaining mod files.
- Runtime testing in Horizon is pending.

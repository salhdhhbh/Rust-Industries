# Rust Industries

An industrial Minecraft mod for Horizon on Android: machines, material processing, crops and automation.

## Development version

- Current feature branch: `feature/0.1.6-full-textures-machines`
- Version: `0.1.6`
- Target: Starling 0.9.0 / Horizon Mod API 0.2 / Game 26.3
- Native source: `mod/v0.1.6-native/`
- Version 0.1.5 remains preserved on `main`; versions 0.1.3 and 0.1.4 remain historical baselines.

## 0.1.6 content

The 0.1.6 source includes dedicated block and item textures, separate growth stages for four crop families, custom machine models, four additional Native processing machines (Millstone, Industrial Drill, Mechanical Press, Mechanical Mixer), a nine-slot Depot, and machine-specific screens and recipes.

Support-block behaviours include Hopper-template item transport for Factory Chute and Factory Funnel, Lever-template redstone behaviour for Hand Crank, Cauldron-template water storage for Basin, and a movement-speed factor on Mechanical Fan. Shaft, Cogwheel, Gearbox, Motor and Water Wheel have their own models/textures and crafting roles; a complete rotational power network between those components is not implemented yet.

## Native Build

Open `mod/v0.1.6-native/` as a Native mod project in Starling/Horizon and run Native Build for the target game instance. The Cargo dependency uses the Horizon SDK path `.horizon/sdk/api/crates/horizon-mod-api`.

**Validation note:** the HMOD archive and its JSON/model/texture references have been checked. Native compilation and in-game behaviour must still be tested in the target launcher. This repository branch contains source code; the installable HMOD is distributed separately.

## Repository layout

- `mod/v0.1.5-native/` — previous Native Crusher source.
- `mod/v0.1.6-native/` — expanded Native source.
- `releases/v0.1.3/` — preserved historical release.

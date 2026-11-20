# Rust Industries 0.1.4 — test protocol

## Purpose

Confirm that Horizon Android accepts and loads the Rust Industries content before any feature work. This is a diagnostic build, not a feature release.

## Diagnostic finding

The first 0.1.4 test was recognized by Horizon and shown as ON, but its content was not loaded. The diagnostic log reported:

```text
Horizon: content mods not loaded:
rust_industries:crushing_wheel:
template minecraft:chain is not a block
```

The same invalid template appeared in six block definitions. The affected blockstate files use `axis=x`, `axis=y` and `axis=z` variants.

## Correction in 0.1.4 FIXED

In these six files, the block template was changed from `minecraft:chain` to `minecraft:oak_log`:

- `crushing_wheel.json`
- `gearbox.json`
- `large_cogwheel.json`
- `shaft.json`
- `small_cogwheel.json`
- `water_wheel.json`

The log template supplies the `axis` state used by the existing blockstate variants. The existing models, textures, recipes, loot tables, Creative tab and all other block/item definitions remain unchanged. The fixed build remains version `0.1.4`.

## Structural validation completed

- HMOD archive entries: 277
- JSON files: 207
- JSON syntax errors: 0
- Creative tab entries: 33
- Creative tab path: `data/rust_industries/horizon/creative_tab/industry.json`
- Manifest API: `0.2`
- Capability: `registries`
- Invalid `minecraft:chain` template references remaining: 0
- Corrected axis-oriented block definitions: 6 of 6

These checks validate archive structure and JSON syntax only. They do not prove that Horizon accepts the replacement template on-device, registers every item, displays the Creative tab, renders all assets or runs the mod correctly.

## User test steps

1. Back up the world.
2. Disable or remove every older Rust Industries copy, including the earlier 0.1.4 TEST package, to avoid stale or duplicate content.
3. Install `RustIndustries_0.1.4.hmod` from the `0.1.4_FIXED` Android ZIP.
4. Fully close Horizon and launch it again. Content mods are read when the game starts.
5. Open the Mods list. Confirm Rust Industries is ON and that no red load error is shown on its row.
6. Enter a Creative world. Check for the Rust Industries tab after the vanilla tabs, then search for `Machine Casing`.
7. Place several representative blocks: machine casing, shaft, gearbox, motor, millstone and funnel. Also test the six corrected axis-oriented blocks in all three orientations where possible.
8. If content is missing, export a fresh Horizon diagnostics ZIP and send it with screenshots of the Mods and Creative screens.

## Report back

Send:
- Screenshot of the Mods row showing Rust Industries 0.1.4 and whether a red error is present.
- Screenshot of the Creative inventory and search result for `Machine Casing`.
- Fresh Horizon diagnostics ZIP if the content still does not load.
- Device model and Horizon version.

## Pass criteria

- [ ] Horizon identifies the mod as 0.1.4 and keeps it ON.
- [ ] No registry load error appears for Rust Industries.
- [ ] Rust Industries Creative tab is visible.
- [ ] `Machine Casing` appears in Creative search.
- [ ] Representative blocks can be placed and broken.
- [ ] Axis-oriented blocks load without blockstate errors.
- [ ] Test world can save, exit and reopen.

Do not label the build stable until these in-game checks pass. Content mods do not gain machine processing or mechanical behaviour merely from their block models and recipes.

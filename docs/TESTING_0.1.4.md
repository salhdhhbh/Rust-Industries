# Rust Industries 0.1.4 — test protocol

## Purpose

Find out whether Horizon Android installs and loads Rust Industries correctly before making feature changes. This is a diagnostic build, not a feature release.

## Controlled build

The test HMOD is derived from the supplied 0.1.3 package. Only `version = "0.1.3"` in `horizon-mod.toml` is changed to `version = "0.1.4"`. All gameplay definitions and assets remain unchanged to keep the test controlled.

## Structural validation completed

- HMOD archive entries: 277
- JSON files: 207
- JSON syntax errors: 0
- Creative tab entries: 33
- Creative tab path: `data/rust_industries/horizon/creative_tab/industry.json`
- Creative icon: `rust_industries:machine_casing`
- Creative item IDs checked against the supplied block/item registry definitions: 33 of 33 present
- Manifest API: `0.2`
- Capability: `registries`

These checks validate archive structure and JSON syntax only. They do not prove that Horizon accepts every registry field, loads the Creative tab, renders assets, or runs the mod correctly.

## User test steps

1. Back up the world.
2. Disable or remove every older Rust Industries copy, especially 0.1.2 and 0.1.3, to avoid duplicate IDs or cached old packages.
3. Import `RustIndustries_0.1.4.hmod` into Horizon.
4. Fully close Horizon and launch it again.
5. Open the installed Mods list and check that the version reads 0.1.4.
6. Enter a Creative world and search for `Rust Industries`; verify the custom tab and its entries.
7. Place several representative blocks: machine casing, shaft, gearbox, motor, millstone and funnel.
8. Check the Horizon log for unknown properties, failed registry entries, missing models or missing textures.
9. Record FPS/RAM only after the mod loads successfully, using the same world and graphics settings as the vanilla baseline.

## Report back

Send:
- Screenshot of the Mods list showing the installed version.
- Screenshot of the Creative inventory (or the missing-tab result).
- Horizon error log or screenshot of the exact error.
- Device model and Horizon version.

## Pass criteria

- [ ] Horizon identifies the mod as 0.1.4.
- [ ] No duplicate older copy is enabled.
- [ ] Rust Industries Creative tab is visible.
- [ ] All expected entries are present.
- [ ] Representative blocks can be placed and broken.
- [ ] No mod-loading or registry errors appear.
- [ ] Test world can save, exit and reopen.

Only after these checks should we decide which content needs repair. Do not label the build stable until the in-game tests pass.

# Rust Industries

Industrial-themed content mod for Horizon Android.

## Active development target

**0.1.4 fixed test build only.** The 1.0.0 roadmap is not active. Version 0.1.3 remains the baseline and must be preserved unchanged.

The first 0.1.4 diagnostic package only changed the manifest version. Horizon's diagnostic log then exposed a registry failure: `rust_industries:crushing_wheel` used `minecraft:chain` as its template, but the running Horizon build did not recognize that template as a block.

The corrected 0.1.4 build replaces `minecraft:chain` with `minecraft:oak_log` in six axis-oriented block definitions: crushing wheel, gearbox, large cogwheel, shaft, small cogwheel and water wheel. These blocks already use `axis=x/y/z` model variants, which are provided by the log template. No new gameplay features or mechanical behaviour are added.

## 0.1.4 fixed test

- HMOD: `RustIndustries_0.1.4_FIXED.hmod`
- Android install ZIP: `RustIndustries_Android_Install_v0.1.4_FIXED.zip`
- Structural checks: 277 archive entries, 207 JSON files parsed, zero JSON syntax errors, 33 Creative tab entries.
- Registry correction: six invalid `minecraft:chain` templates replaced with `minecraft:oak_log`.
- Runtime status: **awaiting re-test in Horizon**. The earlier 0.1.4 build was recognized and enabled, but its content failed registry loading.

Install only after disabling/removing older Rust Industries copies, then fully restart Horizon. Check the Mods row for a red load error, then check the Creative tab and search for `Machine Casing`. If content is still missing, export a fresh Horizon diagnostics ZIP.

## Development rules

- Work only on the `release/0.1.4` branch for this test cycle.
- Do not alter or overwrite 0.1.3 release files.
- Do not publish 1.0.0 or add large features during this cycle.
- Fix only issues that block installation, registration, Creative visibility or basic stability.
- Do not claim FPS/RAM improvements without measurements on the target Android device.

See [0.1.4 test protocol](docs/TESTING_0.1.4.md).

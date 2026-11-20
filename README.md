# Rust Industries

Industrial-themed content mod for Horizon Android.

## Active development target

**0.1.4 test build only.** The 1.0.0 roadmap is not active. Version 0.1.3 remains the baseline and must be preserved unchanged.

The 0.1.4 diagnostic package is based on the supplied 0.1.3 HMOD. Its only content change is the manifest version string from `0.1.3` to `0.1.4`; blocks, items, recipes, assets and Creative tab are intentionally unchanged so installation and mod discovery can be isolated.

## 0.1.4 test

- HMOD: `RustIndustries_0.1.4.hmod`
- Android install ZIP: `RustIndustries_Android_Install_v0.1.4_TEST.zip`
- Structural checks: 277 archive entries, 207 JSON files parsed, zero JSON syntax errors, 33 Creative tab entries.
- Runtime status: **not yet tested in Horizon**.

Install only after disabling/removing older Rust Industries copies, then fully restart Horizon. Check the displayed mod version and the Creative tab. If it still shows an older version or content is missing, capture the Mods screen, Creative inventory and Horizon error log.

## Development rules

- Work only on the `release/0.1.4` branch for this test cycle.
- Do not alter or overwrite 0.1.3 release files.
- Do not publish 1.0.0 or add large features during this cycle.
- Fix only issues that block installation, registration, Creative visibility or basic stability.
- Do not claim FPS/RAM improvements without measurements on the target Android device.

See [0.1.4 test protocol](docs/TESTING_0.1.4.md).

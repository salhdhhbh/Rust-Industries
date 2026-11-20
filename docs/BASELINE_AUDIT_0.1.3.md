# 0.1.3 baseline audit

Audit input supplied in this conversation:
- `RustIndustries_0.1.3_FIXED.hmod`
- `RustIndustries_Android_Install_v0.1.3_FIXED.zip`

## Structural results

- HMOD archive entries: **277**
- JSON files parsed: **207**
- JSON parse errors: **0**
- Registered block definitions: **22**
- Registered item definitions: **11**
- Creative tab entries: **33**
- Creative entries without a matching registered block/item: **0**
- Creative tab file: `data/rust_industries/horizon/creative_tab/industry.json`
- Manifest version: `0.1.3`
- Manifest API: `0.2`
- Capability: `registries`

## Creative Mode finding

The 0.1.3 package already declares a dedicated `Rust Industries` Creative tab. Its icon is `rust_industries:machine_casing`. Its 33 entries match the 22 registered blocks and 11 registered items. Therefore, do not add a duplicate tab. If it is missing in-game, investigate the actual Horizon build, mod enablement, manifest/API acceptance and runtime log.

## Integrity hashes

SHA-256 of supplied HMOD:
`cb55418e37f4fb90b8f201345a25e0626f2b132980b06893bb52a38a016661d4`

SHA-256 of supplied Android install ZIP:
`0bc1afda2ab90b0d20580df86e60e9ca9aa6e97bcdcb578e0e69a3048abd6106`

## What this audit does not prove

- It does not prove the game accepts every block property or state definition.
- It does not prove models render correctly or that textures resolve in-game.
- It does not prove any machine rotates, processes items, transfers inventory or has a GUI.
- It does not measure RAM, FPS, frame time or battery use.
- It does not prove installation on the user's exact Horizon Android build.

## Next tests

1. Install only this HMOD in a clean test instance.
2. Confirm the Creative tab and all 33 entries.
3. Check the log for rejected manifest, block properties, states, models and assets.
4. Place representative blocks and inspect collision, orientation, breaking and drops.
5. Record a repeatable vanilla-vs-mod performance baseline on the target phone.

# Rust Industries 0.1.8 — Native energy network preview

## Package status

The original installable HMOD was supplied for this project and inspected locally. **The binary HMOD has not yet been committed to this GitHub branch.** The connector's repository-write actions currently available in this session create UTF-8 files and Git objects; there is no direct binary attachment/release-upload action exposed here.

Expected artifact name:
`RustIndustries_0.1.8_NATIVE_ENERGY_MERGED.hmod`

Expected repository path:
`releases/v0.1.8/RustIndustries_0.1.8_NATIVE_ENERGY_MERGED.hmod`

The original file is available separately in the conversation download. Do not substitute a rebuilt or renamed binary without checking its hash.

## Package contents

The supplied HMOD is a ZIP-based package containing 338 entries:
- `assets/` — block states, block/item models, language files, and textures.
- `data/` — Horizon block, item, menu, loot-table, and recipe definitions.
- `src/` — six Rust source modules.
- `Cargo.toml`, `horizon-mod.toml`, and `README_FA.md`.

- File size: 129,700 bytes
- Uncompressed archive size: 183,166 bytes
- SHA-256: `b04d0ed41d1ef3856fa60c9ab5a32b3edd0006e71e03df6841ee8f4cbb4db898`

## Energy specifications

| Component | Capacity | Declared behavior |
|---|---:|---|
| Coal Generator | 20,000 FE | 50 FE per active fuel tick; extraction up to 1,000 FE per call |
| Basic Energy Cube | 4,000,000 FE | Bidirectional storage; transfer up to 4,000 FE per call |
| Basic Universal Cable | 8,000 FE | Buffer and transfer through neighbouring ENERGY capabilities |
| Crushing Wheel | 20,000 FE | 50 FE per processing tick; 2,000 FE per 40-tick cycle |

Fuel durations: coal and charcoal 1,600 ticks; coal block 16,000; blaze rod 2,400; dried kelp block 4,000. When the generator buffer is full, it does not consume new fuel and preserves the active burn timer.

## Known limitations

- Only Crushing Wheel consumes FE. Millstone, Industrial Drill, Mechanical Press, and Mechanical Mixer continue to use their existing mechanical processing logic.
- Basic Energy Cube has no dedicated charge-level GUI.
- Cable models do not yet implement dynamic connection states or custom collision shapes.
- In multi-branch networks, neighbour iteration order can affect transfer priority.

## Validation

The supplied archive was opened successfully and its 243 JSON files and two TOML files parsed. Basic archive path and symlink checks passed.

Rust compilation and in-game testing have not been performed. This is an experimental Native Build package, not a verified runtime release. See [`docs/VALIDATION_0.1.8.md`](../../docs/VALIDATION_0.1.8.md) and [`docs/STARLING_NATIVE_BUILD.md`](../../docs/STARLING_NATIVE_BUILD.md).

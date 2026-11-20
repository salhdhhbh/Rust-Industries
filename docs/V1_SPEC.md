# Rust Industries 1.0.0 — product and release specification

## Release principle

1.0.0 must be one coherent, installable, tested release. Do not rename 0.1.3 to 1.0.0 or publish a placeholder package. Keep 0.1.3 available as the last known packaged baseline until the new candidate passes validation.

## Pillar 1 — Creative inventory

- Keep one clearly named `Rust Industries` Creative tab.
- Use a valid registered item as the tab icon.
- Include every registered Rust Industries block and item exactly once, in useful groups/order.
- Check IDs in the tab against actual block/item registry files.
- Confirm the tab appears in-game after a clean restart; do not treat a JSON file existing in the archive as proof of runtime success.

Current finding: 0.1.3 already includes `data/rust_industries/horizon/creative_tab/industry.json`. The task is to validate and correct its contents/runtime loading, not blindly create a second tab.

## Pillar 2 — GitHub source of truth

- Keep source files separate from release binaries.
- Keep the manifest at the HMOD root.
- Keep registry definitions under `data/rust_industries/horizon/`.
- Keep textures, models, blockstates and translations under `assets/rust_industries/`.
- Keep installable artifacts under versioned `releases/vX.Y.Z/` directories.
- Every release must have a changelog, compatibility note, test status and SHA-256 checksum.
- Never overwrite a previous release artifact; publish a new version directory.

## Pillar 3 — Mechanical gameplay

Target experience: compact factory systems inspired by mechanical automation, not a high-cost visual clone of Create.

Suggested progression:
1. Hand crank / water wheel — simple power sources.
2. Shaft / small cogwheel / large cogwheel / gearbox — visible mechanical transmission.
3. Millstone / crusher / press — basic processing.
4. Basin / mixer — recipes and batch processing.
5. Chute / funnel / depot — item routing and staging.
6. Motor / drill / fan — powered utility machines.

Important API boundary: a content-only HMOD using registries API 0.2 can define blocks, items, assets, recipes and loot. It does not, by itself, implement custom per-tick rotation, item transport, machine processing logic, inventories or GUIs. Those features require a separately verified supported code API/runtime (native or WebAssembly) and must be scoped only after checking the exact Horizon Android build. If that implementation is not ready, 1.0.0 must label such blocks as decorative/recipe-driven rather than claim full automation.

## Pillar 4 — performance-first engineering

- No unnecessary block entities or always-running tick handlers.
- No per-tick scans of neighbouring blocks or large areas.
- No particle streams or continuously animated block models as a default.
- Reuse model geometry and textures where possible.
- Prefer simple cuboids and low element counts; avoid complex high-poly meshes.
- Minimize state properties and avoid combinatorial state explosions.
- Use vanilla block templates with suitable collision/occlusion where possible.
- Use simple collision/outline shapes; avoid oversized or multipart collision.
- Avoid random ticking for decorative or non-growing blocks.
- Keep crop growth and loot mechanics data-driven and limited.
- Use compact textures and avoid unnecessary duplicate assets.
- Test a fixed scene with the same render distance, simulation distance and graphics settings, comparing vanilla, 0.1.3 and each 1.0.0 candidate.

## Required release gates

- [ ] Manifest accepted by the target Horizon build.
- [ ] Every JSON file parses.
- [ ] Every referenced model, texture, item and block ID exists.
- [ ] Creative tab appears and contains all intended entries.
- [ ] No unknown block properties or rejected registry definitions in the log.
- [ ] Install, load, save, exit and reload a test world successfully.
- [ ] No missing textures/models or broken blockstates.
- [ ] Performance test completed on the target Android device.
- [ ] Release HMOD and Android ZIP install cleanly.
- [ ] README, changelog, compatibility and SHA-256 are published.

## Version policy

- 0.1.x: prototype and compatibility fixes.
- 0.2–0.9: feature development and testing.
- 1.0.0: first stable feature-complete release meeting all gates above.

Do not mark a gate complete without an actual test result.

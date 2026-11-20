# Android performance rules

The priority is stable frame time and low memory use on mobile hardware. A mod cannot guarantee a particular FPS number: device GPU/CPU, world complexity, render distance, shaders, and other mods all matter. We can reduce Rust Industries' own cost and measure it.

## Cost model

- **RAM:** loaded textures, model data, registry/state tables, cached chunks and any machine inventories.
- **CPU:** random/block ticks, neighbour updates, recipe searches, item routing and machine logic.
- **GPU:** model triangles/faces, draw calls, transparent layers, animated textures, particles and moving renderers.

## Rules for every new block

1. Use a vanilla template with the closest correct collision and occlusion behaviour.
2. Keep the model simple: target 1–6 cuboid elements for ordinary machines; use fewer where possible.
3. Share model geometry and texture assets for repeated components.
4. Avoid transparent textures unless the design truly needs them.
5. Avoid animated textures and particle effects by default.
6. Do not add state properties unless gameplay needs them. Keep facing-only machines to six directions; avoid extra booleans that multiply variants.
7. Do not create a block entity just to animate a decorative cog.
8. Avoid random ticks except for crops or mechanics that genuinely need them.
9. Keep item models and block models separate only where their geometry must differ.
10. Validate all resource references to prevent duplicate/missing assets and runtime fallback work.

## Rules for future machine logic

- Machines should sleep when idle and wake on inventory, neighbour or power changes.
- Process on a measured interval, not every frame and not every game tick unless required.
- Cache recipe lookup results; invalidate cache only when inputs/recipe state changes.
- Move only a bounded number of items per update.
- Limit transfer checks to directly connected faces/slots.
- Do not search an entire chunk/world for connected machines.
- Use compact inventories and bounded queues.
- No per-block background thread, no busy loops, and no unbounded particle/entity spawning.
- Batch network/state updates where the API permits it.

## Existing 0.1.3 audit observations

- The package has 277 archive entries and includes a dedicated Creative tab.
- The current block model set contains several simple models and a number of models with zero local elements (these rely on parent/template/model inheritance or need visual verification). Do not assume zero elements means broken, but inspect them in-game.
- The funnel blockstate has many combinations of facing/extracting/powered. Keep these only if the target engine actually needs each state; otherwise simplify after checking placement/interaction behaviour.
- There are 32 crop-stage model JSON files across four crop types. If they are identical in geometry and differ only by texture, consider shared model definitions or a simpler stage design only if Horizon's model format supports it without breaking rendering.

## Measurement protocol

Use one repeatable test world and record:
- Device model, RAM, Android version and Horizon build.
- Render distance, simulation distance, graphics mode, resolution and shader status.
- FPS average and 1% low if available, plus frame-time spikes.
- Memory use before loading the world and after a fixed 5-minute test.
- Scene: same number of machines, crops, item entities and chunks loaded.

Compare vanilla vs 0.1.3 vs each candidate. Change one performance variable at a time. Never claim "minimum RAM" or "FPS improved" without measurements.

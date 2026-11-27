//! Power infrastructure: a coal generator, a storage cube and a universal cable.
//!
//! The capability layout follows the working Horizon Mekanism port. Fuel values
//! and the generator's output are Rust Industries defaults, not Mekanism values.

use crate::energy::EnergyBuffer;
use crate::screen;
use mod_api::{
    BlockEntityPart, BlockEntityTickEvent, BlockEntityTickReply, BlockEntityTicksEvent,
    BlockPart, Capability, Compound, Direction, ENERGY, EnergyHandler, Host, ItemStack,
    ModContainer, ModEnergyBuffer, Screen, ScreenDrawEvent, ScreenEvent, ServerHost, Tag,
};

pub const COAL_GENERATOR: &str = "rust_industries:coal_generator";
pub const COAL_GENERATOR_PATH: &str = "coal_generator";
pub const COAL_GENERATOR_MENU: &str = "rust_industries:coal_generator";

pub const BASIC_ENERGY_CUBE: &str = "rust_industries:basic_energy_cube";
pub const BASIC_ENERGY_CUBE_PATH: &str = "basic_energy_cube";

pub const BASIC_UNIVERSAL_CABLE: &str = "rust_industries:basic_universal_cable";
pub const BASIC_UNIVERSAL_CABLE_PATH: &str = "basic_universal_cable";

/// The first Rust Industries generator stores one short buffer.
pub const GENERATOR_CAPACITY: i32 = 20_000;
/// Generated FE per active burn tick.
pub const GENERATOR_OUTPUT_PER_TICK: i32 = 50;
/// Maximum energy extracted from the generator in one capability call.
pub const GENERATOR_EXTRACT_RATE: i32 = 1_000;

/// Mekanism Basic Energy Cube reference values.
pub const ENERGY_CUBE_CAPACITY: i32 = 4_000_000;
pub const ENERGY_CUBE_TRANSFER_RATE: i32 = 4_000;

/// Mekanism Basic Universal Cable reference values.
pub const CABLE_CAPACITY: i32 = 8_000;
pub const CABLE_TRANSFER_RATE: i32 = 8_000;

const ENERGY_KEY: &str = "energy";
const BURN_REMAINING_KEY: &str = "rust_industries_generator_burn_remaining";
const BURN_TOTAL_KEY: &str = "rust_industries_generator_burn_total";

pub fn generator_container() -> ModContainer {
    ModContainer {
        slots: 1,
        faces: std::array::from_fn(|_| vec![0]),
        inputs: vec![0],
        outputs: Vec::new(),
        menu: Some(COAL_GENERATOR_MENU.to_owned()),
    }
}

/// The generator accepts no energy; it only exposes its generated buffer.
pub fn generator_battery() -> ModEnergyBuffer {
    ModEnergyBuffer {
        capacity: GENERATOR_CAPACITY,
        max_insert: 0,
        max_extract: GENERATOR_EXTRACT_RATE,
        insert_faces: [false; 6],
        extract_faces: [true; 6],
    }
}

/// The cube is bidirectional storage, limited to 4,000 FE per transfer call.
pub fn cube_battery() -> ModEnergyBuffer {
    ModEnergyBuffer {
        capacity: ENERGY_CUBE_CAPACITY,
        max_insert: ENERGY_CUBE_TRANSFER_RATE,
        max_extract: ENERGY_CUBE_TRANSFER_RATE,
        insert_faces: [true; 6],
        extract_faces: [true; 6],
    }
}

/// The cable is a short-lived 8,000 FE buffer with the same throughput.
pub fn cable_battery() -> ModEnergyBuffer {
    ModEnergyBuffer {
        capacity: CABLE_CAPACITY,
        max_insert: CABLE_TRANSFER_RATE,
        max_extract: CABLE_TRANSFER_RATE,
        insert_faces: [true; 6],
        extract_faces: [true; 6],
    }
}

/// Supported first-pass fuels. Burn duration is measured in game ticks.
pub fn fuel_ticks(item: &str) -> Option<i32> {
    match item {
        "minecraft:coal" | "minecraft:charcoal" => Some(1_600),
        "minecraft:coal_block" => Some(16_000),
        "minecraft:blaze_rod" => Some(2_400),
        "minecraft:dried_kelp_block" => Some(4_000),
        _ => None,
    }
}

fn java<K: for<'a> From<&'a str>>(text: &str) -> K {
    K::from(text)
}

fn get_i(nbt: &Compound, key: &str, default: i32) -> i32 {
    let Some((_, value)) = nbt
        .iter()
        .find(|(name, _)| name.0.iter().copied().eq(key.encode_utf16()))
    else {
        return default;
    };
    match value {
        Tag::Int(v) => *v,
        Tag::Short(v) => i32::from(*v),
        Tag::Byte(v) => i32::from(*v),
        _ => default,
    }
}

fn set_i(nbt: &mut Compound, key: &str, value: i32) {
    nbt.insert(java(key), Tag::Int(value));
}

fn publish_generator(
    event: &mut BlockEntityTickEvent<'_>,
    burn_remaining: i32,
    burn_total: i32,
    energy: i32,
    status: i32,
) -> bool {
    let data = vec![
        burn_remaining,
        burn_total.max(1),
        energy.clamp(0, GENERATOR_CAPACITY),
        GENERATOR_CAPACITY,
        status,
    ];
    if event.entity.data == data {
        false
    } else {
        event.entity.data = data;
        true
    }
}

/// Generator status: 0 idle, 1 burning, 2 buffer full.
pub fn tick_generator(host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
    let resized = event.entity.items.len() != 1;
    if resized {
        event.entity.items.resize(1, None);
    }

    let mut energy = get_i(&event.entity.nbt, ENERGY_KEY, 0).clamp(0, GENERATOR_CAPACITY);
    let mut burn_remaining = get_i(&event.entity.nbt, BURN_REMAINING_KEY, 0).clamp(0, 16_000);
    let mut burn_total = get_i(&event.entity.nbt, BURN_TOTAL_KEY, 1).clamp(1, 16_000);
    let mut changed = resized;

    // Do not consume fuel while the internal buffer is full. A partly used fuel
    // item resumes burning when downstream machines draw energy again.
    if energy < GENERATOR_CAPACITY && burn_remaining == 0 {
        if let Some(fuel) = event.entity.items[0].clone() {
            let fuel_id = fuel.item().id(&*host);
            if let Some(duration) = fuel_ticks(&fuel_id) {
                event.entity.items[0] = fuel.shrink(1);
                burn_remaining = duration;
                burn_total = duration;
                changed = true;
            }
        }
    }

    if energy < GENERATOR_CAPACITY && burn_remaining > 0 {
        let mut buffer = EnergyBuffer::new(
            energy,
            GENERATOR_CAPACITY,
            0,
            GENERATOR_EXTRACT_RATE,
        );
        let generated = buffer.generate(GENERATOR_OUTPUT_PER_TICK);
        if generated > 0 {
            energy = buffer.stored();
            burn_remaining -= 1;
            changed = true;
        }
    }

    if changed {
        set_i(&mut event.entity.nbt, ENERGY_KEY, energy);
        set_i(&mut event.entity.nbt, BURN_REMAINING_KEY, burn_remaining);
        set_i(&mut event.entity.nbt, BURN_TOTAL_KEY, burn_total);
    }

    let status = if energy >= GENERATOR_CAPACITY {
        2
    } else if burn_remaining > 0 {
        1
    } else {
        0
    };
    changed |= publish_generator(event, burn_remaining, burn_total, energy, status);
    if !changed {
        *event.reply = BlockEntityTickReply::Unchanged;
    }
}

pub struct CoalGeneratorPart;

#[mod_api::horizon_part]
impl BlockPart for CoalGeneratorPart {}

#[mod_api::horizon_part]
impl BlockEntityPart for CoalGeneratorPart {
    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        event.ticks = true;
    }

    fn block_entity_tick(&self, host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
        tick_generator(host, event);
    }

    fn screen(&self, _host: &dyn Host, event: &mut ScreenEvent<'_>) -> Option<Screen> {
        (event.block == COAL_GENERATOR).then(screen::generator_screen)
    }

    fn screen_draw(&self, host: &dyn mod_api::ClientHost, event: &mut ScreenDrawEvent<'_>) {
        if event.block == COAL_GENERATOR {
            screen::generator_draw(host, event);
        }
    }
}

pub struct UniversalCablePart;

#[mod_api::horizon_part]
impl BlockPart for UniversalCablePart {}

#[mod_api::horizon_part]
impl BlockEntityPart for UniversalCablePart {
    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        event.ticks = true;
    }

    /// Pulls available FE into the cable, then pushes it to adjacent consumers.
    /// Each handler confirms the actual moved amount, so a full or insert-only
    /// neighbour cannot cause the cable to lose energy.
    fn block_entity_tick(&self, host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
        let pos = event.pos;
        let energy = get_i(&event.entity.nbt, ENERGY_KEY, 0).clamp(0, CABLE_CAPACITY);
        let Some(level) = host.level(host.dimension()) else {
            *event.reply = BlockEntityTickReply::Unchanged;
            return;
        };

        let mut neighbours: Vec<Capability<dyn EnergyHandler>> = Vec::with_capacity(6);
        for face in Direction::ALL {
            let [dx, dy, dz] = face.normal();
            let at = [pos[0] + dx, pos[1] + dy, pos[2] + dz];
            if let Some(handler) = level.capability(host, &ENERGY, at, Some(face.opposite())) {
                neighbours.push(handler);
            }
        }

        if neighbours.is_empty() {
            *event.reply = BlockEntityTickReply::Unchanged;
            return;
        }

        let after = host.transaction(move |host, tx| {
            let mut current = energy;

            // First collect from providers. The cable itself limits the total
            // amount it can hold to one 8,000 FE transfer.
            for handler in &neighbours {
                let room = CABLE_CAPACITY.saturating_sub(current);
                let stored = i32::try_from(handler.amount(host)).unwrap_or(i32::MAX);
                let want = stored.min(room).min(CABLE_TRANSFER_RATE).max(0);
                if want <= 0 {
                    continue;
                }
                let taken = i32::try_from(
                    handler.extract(host, tx, u32::try_from(want).unwrap_or(0)),
                )
                .unwrap_or(i32::MAX);
                current = current.saturating_add(taken);
            }

            // Then distribute the cable's stored FE to neighbours with room.
            for handler in &neighbours {
                if current <= 0 {
                    break;
                }
                let capacity = i32::try_from(handler.capacity(host)).unwrap_or(i32::MAX);
                let stored = i32::try_from(handler.amount(host)).unwrap_or(i32::MAX);
                let space = capacity.saturating_sub(stored);
                let want = current.min(space).min(CABLE_TRANSFER_RATE).max(0);
                if want <= 0 {
                    continue;
                }
                let given = i32::try_from(
                    handler.insert(host, tx, u32::try_from(want).unwrap_or(0)),
                )
                .unwrap_or(i32::MAX);
                current = current.saturating_sub(given);
            }

            Ok::<i32, ()>(current)
        });

        let Ok(after) = after else {
            *event.reply = BlockEntityTickReply::Unchanged;
            return;
        };
        let after = after.clamp(0, CABLE_CAPACITY);
        if after != energy {
            set_i(&mut event.entity.nbt, ENERGY_KEY, after);
        } else {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
    }
}

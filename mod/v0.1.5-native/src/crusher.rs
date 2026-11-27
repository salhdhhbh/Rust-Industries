//! Server-side Crusher inventory and processing.

use crate::energy::{advance, EnergyBuffer, MachineProgress, Operation, TickResult};
use mod_api::{
    BlockEntityTickEvent, BlockEntityTickReply, Compound, Host, Identifier, ItemStack,
    ModEnergyBuffer, ServerHost, Tag,
};

pub const INPUT_SLOT: usize = 0;
pub const OUTPUT_SLOT: usize = 1;
pub const PROCESS_TICKS: i32 = 40;

/// The buffer follows Mekanism's basic electric-machine pattern.
pub const ENERGY_CAPACITY: i32 = 20_000;

/// Energy consumed per successful processing tick.
pub const ENERGY_PER_TICK: i32 = 50;

const PROGRESS_KEY: &str = "rust_industries_crusher_progress";
const ENERGY_KEY: &str = "energy";

/// Register an insert-only energy buffer, chargeable from every side.
pub fn energy_battery() -> ModEnergyBuffer {
    ModEnergyBuffer {
        capacity: ENERGY_CAPACITY,
        max_insert: ENERGY_CAPACITY,
        max_extract: 0,
        insert_faces: [true; 6],
        extract_faces: [false; 6],
    }
}

pub fn recipe_for(input: &str) -> Option<(&'static str, u32)> {
    match input {
        "minecraft:raw_iron" | "minecraft:iron_ingot" | "minecraft:iron_ore"
        | "minecraft:deepslate_iron_ore" => Some(("rust_industries:iron_dust", 2)),
        "minecraft:raw_copper" | "minecraft:copper_ingot" | "minecraft:copper_ore"
        | "minecraft:deepslate_copper_ore" => Some(("rust_industries:copper_dust", 2)),
        "minecraft:raw_gold" | "minecraft:gold_ingot" | "minecraft:gold_ore"
        | "minecraft:deepslate_gold_ore" => Some(("rust_industries:gold_dust", 2)),
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

fn publish(event: &mut BlockEntityTickEvent<'_>, progress: i32, status: i32) -> bool {
    let energy = get_i(&event.entity.nbt, ENERGY_KEY, 0).clamp(0, ENERGY_CAPACITY);
    let data = vec![progress, PROCESS_TICKS, status, energy, ENERGY_CAPACITY];
    if event.entity.data == data {
        false
    } else {
        event.entity.data = data;
        true
    }
}

/// Status: 0 idle, 1 processing, 2 output blocked, 3 unsupported item,
/// 4 insufficient energy.
pub fn tick(host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
    let previous = get_i(&event.entity.nbt, PROGRESS_KEY, 0).clamp(0, PROCESS_TICKS);
    let energy = get_i(&event.entity.nbt, ENERGY_KEY, 0).clamp(0, ENERGY_CAPACITY);

    let input = event.entity.items.get(INPUT_SLOT).cloned().flatten();
    let Some(input) = input else {
        let mut changed = false;
        if previous != 0 {
            set_i(&mut event.entity.nbt, PROGRESS_KEY, 0);
            changed = true;
        }
        changed |= publish(event, 0, 0);
        if !changed {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
        return;
    };

    let host_view: &dyn Host = host;
    let input_id = input.item().id(host_view);
    let Some((output_id, output_count)) = recipe_for(&input_id) else {
        let mut changed = false;
        if previous != 0 {
            set_i(&mut event.entity.nbt, PROGRESS_KEY, 0);
            changed = true;
        }
        changed |= publish(event, 0, 3);
        if !changed {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
        return;
    };

    let Some(output_key) =
        host_view.item_key(&Identifier::parse(output_id).expect("valid output id"))
    else {
        let changed = publish(event, previous, 3);
        if !changed {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
        return;
    };

    let current_output = event.entity.items.get(OUTPUT_SLOT).cloned().flatten();
    let output_total = match current_output.as_ref() {
        None => Some(output_count),
        Some(stack) if stack.item() == output_key => {
            let total = stack.count().saturating_add(output_count);
            (total <= 64).then_some(total)
        }
        Some(_) => None,
    };
    let Some(output_total) = output_total else {
        let changed = publish(event, previous, 2);
        if !changed {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
        return;
    };

    // Construct the output before spending energy, so an invalid item stack
    // can never consume power or advance the machine.
    let Some(result) = ItemStack::new(output_key, output_total) else {
        let changed = publish(event, previous, 3);
        if !changed {
            *event.reply = BlockEntityTickReply::Unchanged;
        }
        return;
    };

    let mut buffer = EnergyBuffer::new(energy, ENERGY_CAPACITY, ENERGY_CAPACITY, 0);
    let mut progress = MachineProgress {
        ticks_done: previous,
    };
    let outcome = advance(
        &mut buffer,
        &mut progress,
        Operation {
            ticks: PROCESS_TICKS,
            energy_per_tick: ENERGY_PER_TICK,
        },
        true,
        true,
    );

    match outcome {
        TickResult::Working => {
            set_i(&mut event.entity.nbt, ENERGY_KEY, buffer.stored());
            set_i(&mut event.entity.nbt, PROGRESS_KEY, progress.ticks_done);
            publish(event, progress.ticks_done, 1);
        }
        TickResult::Completed => {
            event.entity.items[OUTPUT_SLOT] = Some(result);
            event.entity.items[INPUT_SLOT] = input.shrink(1);
            set_i(&mut event.entity.nbt, ENERGY_KEY, buffer.stored());
            set_i(&mut event.entity.nbt, PROGRESS_KEY, 0);
            publish(event, 0, 1);
        }
        TickResult::Idle(crate::energy::IdleReason::NoEnergy) => {
            let changed = publish(event, previous, 4);
            if !changed {
                *event.reply = BlockEntityTickReply::Unchanged;
            }
        }
        TickResult::Idle(_) => {
            let changed = publish(event, progress.ticks_done, 0);
            if !changed {
                *event.reply = BlockEntityTickReply::Unchanged;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ENERGY_CAPACITY, ENERGY_PER_TICK, PROCESS_TICKS, recipe_for,
    };
    use crate::energy::{EnergyBuffer, MachineProgress, Operation, TickResult, advance};

    #[test]
    fn metal_inputs_map_to_existing_dust_items() {
        assert_eq!(recipe_for("minecraft:iron_ingot"), Some(("rust_industries:iron_dust", 2)));
        assert_eq!(recipe_for("minecraft:raw_copper"), Some(("rust_industries:copper_dust", 2)));
        assert_eq!(recipe_for("minecraft:deepslate_gold_ore"), Some(("rust_industries:gold_dust", 2)));
        assert_eq!(recipe_for("minecraft:diamond"), None);
    }

    #[test]
    fn crusher_uses_its_configured_energy_for_exactly_forty_ticks() {
        assert_eq!(PROCESS_TICKS, 40);
        assert_eq!(ENERGY_PER_TICK, 50);
        assert_eq!(ENERGY_CAPACITY, 20_000);

        let mut buffer = EnergyBuffer::new(
            ENERGY_CAPACITY,
            ENERGY_CAPACITY,
            ENERGY_CAPACITY,
            0,
        );
        let mut progress = MachineProgress::default();
        let operation = Operation {
            ticks: PROCESS_TICKS,
            energy_per_tick: ENERGY_PER_TICK,
        };

        for _ in 0..PROCESS_TICKS - 1 {
            assert_eq!(
                advance(&mut buffer, &mut progress, operation, true, true),
                TickResult::Working
            );
        }
        assert_eq!(
            advance(&mut buffer, &mut progress, operation, true, true),
            TickResult::Completed
        );
        assert_eq!(buffer.stored(), 18_000);
        assert_eq!(progress.ticks_done, 0);
    }

    #[test]
    fn crusher_holds_progress_when_its_buffer_runs_out() {
        let mut buffer = EnergyBuffer::new(49, ENERGY_CAPACITY, ENERGY_CAPACITY, 0);
        let mut progress = MachineProgress { ticks_done: 12 };
        let operation = Operation {
            ticks: PROCESS_TICKS,
            energy_per_tick: ENERGY_PER_TICK,
        };

        assert_eq!(
            advance(&mut buffer, &mut progress, operation, true, true),
            TickResult::Idle(crate::energy::IdleReason::NoEnergy)
        );
        assert_eq!(buffer.stored(), 49);
        assert_eq!(progress.ticks_done, 12);
    }
}

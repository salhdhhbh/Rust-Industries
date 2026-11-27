//! Native processing logic for the non-Crusher Rust Industries machines.

use mod_api::{
    BlockEntityTickEvent, BlockEntityTickReply, Compound, Host, Identifier, ItemStack,
    ServerHost, Tag,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Machine {
    Millstone,
    Press,
    Mixer,
    Drill,
}

impl Machine {
    pub fn from_block(block: &str) -> Option<Self> {
        match block {
            "rust_industries:millstone" => Some(Self::Millstone),
            "rust_industries:mechanical_press" => Some(Self::Press),
            "rust_industries:mechanical_mixer" => Some(Self::Mixer),
            "rust_industries:industrial_drill" => Some(Self::Drill),
            _ => None,
        }
    }

    pub fn from_menu(menu: &str) -> Option<Self> {
        match menu {
            "rust_industries:millstone" => Some(Self::Millstone),
            "rust_industries:mechanical_press" => Some(Self::Press),
            "rust_industries:mechanical_mixer" => Some(Self::Mixer),
            "rust_industries:industrial_drill" => Some(Self::Drill),
            _ => None,
        }
    }

    pub const fn ticks(self) -> i32 {
        match self {
            Self::Millstone => 60,
            Self::Press => 60,
            Self::Mixer => 80,
            Self::Drill => 80,
        }
    }

    pub const fn output_slot(self) -> usize {
        if matches!(self, Self::Mixer) { 2 } else { 1 }
    }

    fn progress_key(self) -> &'static str {
        match self {
            Self::Millstone => "rust_industries_millstone_progress",
            Self::Press => "rust_industries_press_progress",
            Self::Mixer => "rust_industries_mixer_progress",
            Self::Drill => "rust_industries_drill_progress",
        }
    }
}

#[derive(Clone, Copy)]
struct Recipe {
    output: &'static str,
    count: u32,
}

fn recipe(machine: Machine, a: &str, b: Option<&str>) -> Option<Recipe> {
    match machine {
        Machine::Millstone => match a {
            "minecraft:cobblestone" => Some(Recipe { output: "minecraft:gravel", count: 1 }),
            "minecraft:gravel" => Some(Recipe { output: "minecraft:flint", count: 1 }),
            "minecraft:bone" => Some(Recipe { output: "minecraft:bone_meal", count: 3 }),
            _ => None,
        },
        Machine::Press => match a {
            "minecraft:iron_ingot" => Some(Recipe { output: "rust_industries:iron_plate", count: 2 }),
            "minecraft:copper_ingot" => Some(Recipe { output: "rust_industries:copper_plate", count: 2 }),
            "minecraft:gold_ingot" => Some(Recipe { output: "rust_industries:gold_plate", count: 2 }),
            "rust_industries:steel_ingot" => Some(Recipe { output: "rust_industries:steel_plate", count: 2 }),
            _ => None,
        },
        Machine::Mixer => match (a, b.unwrap_or("")) {
            ("rust_industries:iron_dust", "minecraft:amethyst_shard") => Some(Recipe { output: "rust_industries:iron_essence", count: 1 }),
            ("rust_industries:copper_dust", "minecraft:amethyst_shard") => Some(Recipe { output: "rust_industries:copper_essence", count: 1 }),
            ("rust_industries:gold_dust", "minecraft:amethyst_shard") => Some(Recipe { output: "rust_industries:gold_essence", count: 1 }),
            ("minecraft:diamond", "minecraft:amethyst_shard") => Some(Recipe { output: "rust_industries:diamond_essence", count: 1 }),
            ("rust_industries:copper_coil", "minecraft:redstone") => Some(Recipe { output: "rust_industries:circuit_board", count: 1 }),
            _ => None,
        },
        Machine::Drill => match a {
            "minecraft:iron_ore" | "minecraft:deepslate_iron_ore" => Some(Recipe { output: "minecraft:raw_iron", count: 2 }),
            "minecraft:copper_ore" | "minecraft:deepslate_copper_ore" => Some(Recipe { output: "minecraft:raw_copper", count: 2 }),
            "minecraft:gold_ore" | "minecraft:deepslate_gold_ore" => Some(Recipe { output: "minecraft:raw_gold", count: 2 }),
            "minecraft:coal_ore" | "minecraft:deepslate_coal_ore" => Some(Recipe { output: "minecraft:coal", count: 3 }),
            "minecraft:diamond_ore" | "minecraft:deepslate_diamond_ore" => Some(Recipe { output: "minecraft:diamond", count: 2 }),
            "minecraft:lapis_ore" | "minecraft:deepslate_lapis_ore" => Some(Recipe { output: "minecraft:lapis_lazuli", count: 4 }),
            "minecraft:redstone_ore" | "minecraft:deepslate_redstone_ore" => Some(Recipe { output: "minecraft:redstone", count: 4 }),
            _ => None,
        },
    }
}

/// Slot filters are deliberately stricter than the generic block containers.
pub fn may_place(menu: &str, slot: usize, item: &str) -> bool {
    if menu == "rust_industries:depot" {
        return slot < 9;
    }
    let Some(machine) = Machine::from_menu(menu) else { return false; };
    match machine {
        Machine::Millstone => slot == 0 && recipe(machine, item, None).is_some(),
        Machine::Press => slot == 0 && recipe(machine, item, None).is_some(),
        Machine::Drill => slot == 0 && recipe(machine, item, None).is_some(),
        Machine::Mixer => match slot {
            0 => matches!(item,
                "rust_industries:iron_dust" | "rust_industries:copper_dust" |
                "rust_industries:gold_dust" | "minecraft:diamond" | "rust_industries:copper_coil"),
            1 => matches!(item, "minecraft:amethyst_shard" | "minecraft:redstone"),
            _ => false,
        },
    }
}

fn java<K: for<'a> From<&'a str>>(text: &str) -> K { K::from(text) }

fn get_i(nbt: &Compound, key: &str, default: i32) -> i32 {
    let Some((_, value)) = nbt.iter().find(|(name, _)| name.0.iter().copied().eq(key.encode_utf16())) else {
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

fn publish(event: &mut BlockEntityTickEvent<'_>, progress: i32, maximum: i32, status: i32) -> bool {
    let data = vec![progress, maximum, status];
    if event.entity.data == data { false } else { event.entity.data = data; true }
}

/// Status: 0 idle, 1 processing, 2 output blocked, 3 unsupported input/combination.
pub fn tick(host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>, machine: Machine) {
    let maximum = machine.ticks();
    let key = machine.progress_key();
    let previous = get_i(&event.entity.nbt, key, 0).clamp(0, maximum);
    let Some(input_a) = event.entity.items.get(0).cloned().flatten() else {
        let mut changed = false;
        if previous != 0 { set_i(&mut event.entity.nbt, key, 0); changed = true; }
        changed |= publish(event, 0, maximum, 0);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };

    let host_view: &dyn Host = host;
    let input_a_id = input_a.item().id(host_view);
    let input_b = if matches!(machine, Machine::Mixer) {
        event.entity.items.get(1).cloned().flatten()
    } else { None };
    if matches!(machine, Machine::Mixer) && input_b.is_none() {
        let mut changed = false;
        if previous != 0 { set_i(&mut event.entity.nbt, key, 0); changed = true; }
        changed |= publish(event, 0, maximum, 0);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    }
    let input_b_id = input_b.as_ref().map(|stack| stack.item().id(host_view));
    let Some(recipe) = recipe(machine, &input_a_id, input_b_id.as_deref()) else {
        let mut changed = false;
        if previous != 0 { set_i(&mut event.entity.nbt, key, 0); changed = true; }
        changed |= publish(event, 0, maximum, 3);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };
    let Some(output_key) = host_view.item_key(&Identifier::parse(recipe.output).expect("valid Rust Industries recipe output")) else {
        let changed = publish(event, previous, maximum, 3);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };
    let output_slot = machine.output_slot();
    let current_output = event.entity.items.get(output_slot).cloned().flatten();
    let output_total = match current_output.as_ref() {
        None => Some(recipe.count),
        Some(stack) if stack.item() == output_key => {
            let total = stack.count().saturating_add(recipe.count);
            (total <= 64).then_some(total)
        }
        Some(_) => None,
    };
    let Some(output_total) = output_total else {
        let changed = publish(event, previous, maximum, 2);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };

    let next = previous + 1;
    let mut changed = false;
    if next >= maximum {
        if let Some(result) = ItemStack::new(output_key, output_total) {
            event.entity.items[output_slot] = Some(result);
            event.entity.items[0] = input_a.shrink(1);
            if let Some(stack) = input_b { event.entity.items[1] = stack.shrink(1); }
            set_i(&mut event.entity.nbt, key, 0);
            changed = true;
            changed |= publish(event, 0, maximum, 1);
        }
    } else {
        set_i(&mut event.entity.nbt, key, next);
        changed = true;
        changed |= publish(event, next, maximum, 1);
    }
    if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
}

#[cfg(test)]
mod tests {
    use super::{recipe, Machine};

    #[test]
    fn each_machine_has_a_distinct_recipe_family() {
        assert_eq!(recipe(Machine::Millstone, "minecraft:cobblestone", None).unwrap().output, "minecraft:gravel");
        assert_eq!(recipe(Machine::Press, "minecraft:iron_ingot", None).unwrap().output, "rust_industries:iron_plate");
        assert_eq!(recipe(Machine::Mixer, "rust_industries:gold_dust", Some("minecraft:amethyst_shard")).unwrap().output, "rust_industries:gold_essence");
        assert_eq!(recipe(Machine::Drill, "minecraft:diamond_ore", None).unwrap().output, "minecraft:diamond");
    }
}

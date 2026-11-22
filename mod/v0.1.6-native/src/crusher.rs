//! Server-side Crusher inventory and processing.

use mod_api::{
    BlockEntityTickEvent, BlockEntityTickReply, Compound, Host, Identifier, ItemStack,
    ServerHost, Tag,
};

pub const INPUT_SLOT: usize = 0;
pub const OUTPUT_SLOT: usize = 1;
pub const PROCESS_TICKS: i32 = 40;
const PROGRESS_KEY: &str = "rust_industries_crusher_progress";

pub fn recipe_for(input: &str) -> Option<(&'static str, u32)> {
    match input {
        "minecraft:raw_iron" | "minecraft:iron_ingot" | "minecraft:iron_ore" | "minecraft:deepslate_iron_ore" => Some(("rust_industries:iron_dust", 2)),
        "minecraft:raw_copper" | "minecraft:copper_ingot" | "minecraft:copper_ore" | "minecraft:deepslate_copper_ore" => Some(("rust_industries:copper_dust", 2)),
        "minecraft:raw_gold" | "minecraft:gold_ingot" | "minecraft:gold_ore" | "minecraft:deepslate_gold_ore" => Some(("rust_industries:gold_dust", 2)),
        _ => None,
    }
}

fn java<K: for<'a> From<&'a str>>(text: &str) -> K { K::from(text) }

fn get_i(nbt: &Compound, key: &str, default: i32) -> i32 {
    let Some((_, value)) = nbt.iter().find(|(name, _)| name.0.iter().copied().eq(key.encode_utf16())) else { return default; };
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
    let data = vec![progress, PROCESS_TICKS, status];
    if event.entity.data == data { false } else { event.entity.data = data; true }
}

/// Status: 0 idle, 1 processing, 2 output blocked, 3 unsupported item.
pub fn tick(host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
    let previous = get_i(&event.entity.nbt, PROGRESS_KEY, 0).clamp(0, PROCESS_TICKS);
    let Some(input) = event.entity.items.get(INPUT_SLOT).cloned().flatten() else {
        let mut changed = false;
        if previous != 0 { set_i(&mut event.entity.nbt, PROGRESS_KEY, 0); changed = true; }
        changed |= publish(event, 0, 0);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };
    let host_view: &dyn Host = host;
    let input_id = input.item().id(host_view);
    let Some((output_id, output_count)) = recipe_for(&input_id) else {
        let mut changed = false;
        if previous != 0 { set_i(&mut event.entity.nbt, PROGRESS_KEY, 0); changed = true; }
        changed |= publish(event, 0, 3);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };
    let Some(output_key) = host_view.item_key(&Identifier::parse(output_id).expect("valid output id")) else {
        let changed = publish(event, previous, 3);
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
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
        if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
        return;
    };
    let next = previous + 1;
    let mut changed = false;
    if next >= PROCESS_TICKS {
        if let Some(result) = ItemStack::new(output_key, output_total) {
            event.entity.items[OUTPUT_SLOT] = Some(result);
            event.entity.items[INPUT_SLOT] = input.shrink(1);
            set_i(&mut event.entity.nbt, PROGRESS_KEY, 0);
            changed = true;
            changed |= publish(event, 0, 1);
        }
    } else {
        set_i(&mut event.entity.nbt, PROGRESS_KEY, next);
        changed = true;
        changed |= publish(event, next, 1);
    }
    if !changed { *event.reply = BlockEntityTickReply::Unchanged; }
}

#[cfg(test)]
mod tests {
    use super::recipe_for;

    #[test]
    fn metal_inputs_map_to_matching_dust() {
        assert_eq!(recipe_for("minecraft:iron_ingot"), Some(("rust_industries:iron_dust", 2)));
        assert_eq!(recipe_for("minecraft:raw_copper"), Some(("rust_industries:copper_dust", 2)));
        assert_eq!(recipe_for("minecraft:deepslate_gold_ore"), Some(("rust_industries:gold_dust", 2)));
        assert_eq!(recipe_for("minecraft:diamond"), None);
    }
}

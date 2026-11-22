//! Rust Industries 0.1.6 - Native industrial machines.

mod crusher;
mod machines;
mod screen;

use mod_api::{
    self, BlockEntityPart, BlockEntityTickEvent, BlockEntityTicksEvent, BlockPart,
    BlockSpeedFactorEvent, ClientHost, Host, LoopContext, MenuContainer, MenuPart,
    MenuSlotRulesEvent, ModContainer, Registrar, RegistrationContext, Screen,
    ScreenDrawEvent, ScreenEvent, ServerHost,
};

const CRUSHING_WHEEL: &str = "rust_industries:crushing_wheel";
const CRUSHING_WHEEL_PATH: &str = "crushing_wheel";
const CRUSHER_MENU: &str = "rust_industries:crusher";
const MILLSTONE: &str = "rust_industries:millstone";
const PRESS: &str = "rust_industries:mechanical_press";
const MIXER: &str = "rust_industries:mechanical_mixer";
const DRILL: &str = "rust_industries:industrial_drill";
const DEPOT: &str = "rust_industries:depot";
const MILLSTONE_MENU: &str = "rust_industries:millstone";
const PRESS_MENU: &str = "rust_industries:mechanical_press";
const MIXER_MENU: &str = "rust_industries:mechanical_mixer";
const DRILL_MENU: &str = "rust_industries:industrial_drill";
const DEPOT_MENU: &str = "rust_industries:depot";

fn container(slots: usize, input_slots: Vec<usize>, output_slots: Vec<usize>, menu: &str) -> ModContainer {
    ModContainer {
        slots,
        faces: std::array::from_fn(|_| input_slots.clone()),
        inputs: input_slots,
        outputs: output_slots,
        menu: Some(menu.to_owned()),
    }
}

fn crusher_container() -> ModContainer {
    container(2, vec![crusher::INPUT_SLOT], vec![crusher::OUTPUT_SLOT], CRUSHER_MENU)
}

fn two_slot_container(menu: &str) -> ModContainer {
    container(2, vec![0], vec![1], menu)
}

fn mixer_container() -> ModContainer {
    container(3, vec![0, 1], vec![2], MIXER_MENU)
}

fn depot_container() -> ModContainer {
    let slots: Vec<usize> = (0..9).collect();
    container(9, slots.clone(), slots, DEPOT_MENU)
}

struct CrusherPart;

#[mod_api::horizon_part]
impl BlockPart for CrusherPart {}

#[mod_api::horizon_part]
impl BlockEntityPart for CrusherPart {
    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        event.ticks = true;
    }
    fn block_entity_tick(&self, host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
        crusher::tick(host, event);
    }
    fn screen(&self, _host: &dyn Host, event: &mut ScreenEvent<'_>) -> Option<Screen> {
        (event.block == CRUSHING_WHEEL).then(screen::crusher_screen)
    }
    fn screen_draw(&self, host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
        if event.block == CRUSHING_WHEEL { screen::crusher_draw(host, event); }
    }
}

struct MachinePart { machine: machines::Machine }

#[mod_api::horizon_part]
impl BlockPart for MachinePart {}

#[mod_api::horizon_part]
impl BlockEntityPart for MachinePart {
    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        event.ticks = true;
    }
    fn block_entity_tick(&self, host: &mut dyn ServerHost, event: &mut BlockEntityTickEvent<'_>) {
        machines::tick(host, event, self.machine);
    }
    fn screen(&self, _host: &dyn Host, event: &mut ScreenEvent<'_>) -> Option<Screen> {
        screen::machine_screen(event.block)
    }
    fn screen_draw(&self, host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
        screen::machine_draw(host, event);
    }
}

struct DepotPart;

#[mod_api::horizon_part]
impl BlockPart for DepotPart {}

#[mod_api::horizon_part]
impl BlockEntityPart for DepotPart {
    fn screen(&self, _host: &dyn Host, event: &mut ScreenEvent<'_>) -> Option<Screen> {
        (event.block == DEPOT).then(screen::depot_screen)
    }
    fn screen_draw(&self, host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
        if event.block == DEPOT { screen::depot_draw(host, event); }
    }
}

struct CrusherSlotRules;

#[mod_api::horizon_part]
impl MenuPart for CrusherSlotRules {
    fn menu_slot_rules(&self, host: &dyn Host, event: &mut MenuSlotRulesEvent<'_>) {
        if event.container != MenuContainer::Main || event.menu != CRUSHER_MENU { return; }
        match event.slot {
            crusher::INPUT_SLOT => {
                let id = event.stack.item().id(host);
                event.may_place = crusher::recipe_for(&id).is_some();
            }
            crusher::OUTPUT_SLOT => event.may_place = false,
            _ => event.may_place = false,
        }
    }
}

struct MachineSlotRules;

#[mod_api::horizon_part]
impl MenuPart for MachineSlotRules {
    fn menu_slot_rules(&self, host: &dyn Host, event: &mut MenuSlotRulesEvent<'_>) {
        if event.container != MenuContainer::Main { return; }
        if event.menu == DEPOT_MENU {
            event.may_place = event.slot < 9;
            return;
        }
        let Some(machine) = machines::Machine::from_menu(event.menu) else { return; };
        if event.slot == machine.output_slot() {
            event.may_place = false;
            return;
        }
        let id = event.stack.item().id(host);
        event.may_place = machines::may_place(event.menu, event.slot, &id);
    }
}

struct MechanicalFanPart;

#[mod_api::horizon_part]
impl BlockPart for MechanicalFanPart {
    fn block_speed_factor(&self, _context: &dyn LoopContext, event: &mut BlockSpeedFactorEvent) {
        // A moving walkway effect when an entity stands on the fan's top face.
        event.factor *= 1.20;
    }
}

#[derive(Default)]
struct RustIndustries;

#[mod_api::horizon_mod]
impl mod_api::Hooks for RustIndustries {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let crusher_kind = registrar.block_entity_type(CRUSHING_WHEEL_PATH).blocks(&["crushing_wheel"]).build();
        registrar.block_entity_part(&crusher_kind, CrusherPart).expect("Rust Industries: Crusher registration");
        let crusher_block = registrar.block_key(CRUSHING_WHEEL);
        registrar.block_container(&crusher_block, crusher_container());
        registrar.menu_part(CRUSHER_MENU, CrusherSlotRules);

        for (path, block_id, menu, machine_container) in [
            ("millstone", MILLSTONE, MILLSTONE_MENU, two_slot_container(MILLSTONE_MENU)),
            ("mechanical_press", PRESS, PRESS_MENU, two_slot_container(PRESS_MENU)),
            ("mechanical_mixer", MIXER, MIXER_MENU, mixer_container()),
            ("industrial_drill", DRILL, DRILL_MENU, two_slot_container(DRILL_MENU)),
        ] {
            let kind = registrar.block_entity_type(path).blocks(&[path]).build();
            registrar.block_entity_part(&kind, MachinePart { machine: machines::Machine::from_block(block_id).expect("known machine block") }).expect("Rust Industries: machine registration");
            let key = registrar.block_key(block_id);
            registrar.block_container(&key, machine_container);
            registrar.menu_part(menu, MachineSlotRules);
        }

        let depot_kind = registrar.block_entity_type("depot").blocks(&["depot"]).build();
        registrar.block_entity_part(&depot_kind, DepotPart).expect("Rust Industries: Depot registration");
        let depot_block = registrar.block_key(DEPOT);
        registrar.block_container(&depot_block, depot_container());
        registrar.menu_part(DEPOT_MENU, MachineSlotRules);

        let fan_block = registrar.block_key("rust_industries:mechanical_fan");
        registrar.block_part(fan_block, MechanicalFanPart);

        for id in [
            "rust_industries:iron_dust", "rust_industries:copper_dust", "rust_industries:gold_dust",
            "rust_industries:iron_essence", "rust_industries:copper_essence", "rust_industries:gold_essence",
            "rust_industries:diamond_essence", "rust_industries:circuit_board",
            "rust_industries:iron_plate", "rust_industries:copper_plate", "rust_industries:gold_plate", "rust_industries:steel_plate",
        ] {
            let _ = registrar.item_key(id);
        }
    }
}

//! Rust Industries 0.1.5 - native machine behaviour.
//! The first native machine is the Crusher.

mod crusher;
mod screen;
mod power;

/// Mekanism-inspired energy storage and machine-cycle primitives.
pub mod energy;

use mod_api::{
    self, BlockEntityPart, BlockEntityTickEvent, BlockEntityTicksEvent, BlockPart, ClientHost,
    Host, MenuContainer, MenuPart, MenuSlotRulesEvent, ModContainer, Registrar,
    RegistrationContext, Screen, ScreenDrawEvent, ScreenEvent, ServerHost,
};

const CRUSHING_WHEEL: &str = "rust_industries:crushing_wheel";
const CRUSHING_WHEEL_PATH: &str = "crushing_wheel";
const CRUSHER_MENU: &str = "rust_industries:crusher";

fn crusher_container() -> ModContainer {
    ModContainer {
        slots: 2,
        faces: std::array::from_fn(|_| vec![crusher::INPUT_SLOT]),
        inputs: vec![crusher::INPUT_SLOT],
        outputs: vec![crusher::OUTPUT_SLOT],
        menu: Some(CRUSHER_MENU.to_owned()),
    }
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
        if event.block == CRUSHING_WHEEL {
            screen::crusher_draw(host, event);
        }
    }
}

struct CrusherSlotRules;

#[mod_api::horizon_part]
impl MenuPart for CrusherSlotRules {
    fn menu_slot_rules(&self, _host: &dyn Host, event: &mut MenuSlotRulesEvent<'_>) {
        if event.container != MenuContainer::Main || event.menu != CRUSHER_MENU {
            return;
        }
        match event.slot {
            crusher::INPUT_SLOT => {
                let id = event.stack.item().id(_host);
                event.may_place = crusher::recipe_for(&id).is_some();
            }
            crusher::OUTPUT_SLOT => event.may_place = false,
            _ => {}
        }
    }
}

struct GeneratorSlotRules;

#[mod_api::horizon_part]
impl MenuPart for GeneratorSlotRules {
    fn menu_slot_rules(&self, host: &dyn Host, event: &mut MenuSlotRulesEvent<'_>) {
        if event.container != MenuContainer::Main
            || event.menu != power::COAL_GENERATOR_MENU
        {
            return;
        }
        let id = event.stack.item().id(host);
        event.may_place = event.slot == 0 && power::fuel_ticks(&id).is_some();
    }
}

#[derive(Default)]
struct RustIndustries;

#[mod_api::horizon_mod]
impl mod_api::Hooks for RustIndustries {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let kind = registrar
            .block_entity_type(CRUSHING_WHEEL_PATH)
            .blocks(&[CRUSHING_WHEEL_PATH])
            .build();
        registrar
            .block_entity_part(&kind, CrusherPart)
            .expect("Rust Industries: Crusher block entity registration");

        let block = registrar.block_key(CRUSHING_WHEEL);
        registrar.block_container(&block, crusher_container());

        // The Crusher accepts FE from all six faces and never exposes energy extraction.
        registrar.block_energy(&block, crusher::energy_battery());

        registrar.menu_part(CRUSHER_MENU, CrusherSlotRules);

        for id in [
            "rust_industries:iron_dust",
            "rust_industries:copper_dust",
            "rust_industries:gold_dust",
        ] {
            let _ = registrar.item_key(id);
        }

        // Coal Generator: a fuel-powered FE source with a persistent buffer.
        let generator = registrar.block_key(power::COAL_GENERATOR);
        registrar.block_container(&generator, power::generator_container());
        registrar.block_energy(&generator, power::generator_battery());
        let generator_kind = registrar
            .block_entity_type(power::COAL_GENERATOR_PATH)
            .blocks(&[power::COAL_GENERATOR_PATH])
            .build();
        registrar
            .block_entity_part(&generator_kind, power::CoalGeneratorPart)
            .expect("Rust Industries: Coal Generator registration");
        registrar.menu_part(power::COAL_GENERATOR_MENU, GeneratorSlotRules);

        // Energy Cube: large bidirectional storage, with no autonomous ticker.
        let cube = registrar.block_key(power::BASIC_ENERGY_CUBE);
        registrar.block_energy(&cube, power::cube_battery());
        registrar
            .block_entity_type(power::BASIC_ENERGY_CUBE_PATH)
            .blocks(&[power::BASIC_ENERGY_CUBE_PATH])
            .build();

        // Universal Cable: a ticked buffer that pulls from and pushes to adjacent
        // Horizon energy capabilities.
        let cable = registrar.block_key(power::BASIC_UNIVERSAL_CABLE);
        registrar.block_energy(&cable, power::cable_battery());
        let cable_kind = registrar
            .block_entity_type(power::BASIC_UNIVERSAL_CABLE_PATH)
            .blocks(&[power::BASIC_UNIVERSAL_CABLE_PATH])
            .build();
        registrar
            .block_entity_part(&cable_kind, power::UniversalCablePart)
            .expect("Rust Industries: Universal Cable registration");
    }
}

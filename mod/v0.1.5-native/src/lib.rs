//! Rust Industries 0.1.5 - native machine behaviour.
//! The first native machine is the Crusher.

mod crusher;
mod screen;

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
        registrar.menu_part(CRUSHER_MENU, CrusherSlotRules);

        for id in [
            "rust_industries:iron_dust",
            "rust_industries:copper_dust",
            "rust_industries:gold_dust",
        ] {
            let _ = registrar.item_key(id);
        }
    }
}

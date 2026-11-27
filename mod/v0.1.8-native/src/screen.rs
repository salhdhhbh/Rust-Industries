//! Client-side machine screens.

use mod_api::{ClientHost, Screen, ScreenDrawEvent};

const CRUSHER_GUI: &str = "rust_industries:gui/crusher";
const GENERATOR_GUI: &str = "rust_industries:gui/coal_generator";
const OPAQUE: u32 = 0xFF00_0000;

fn lang(host: &dyn ClientHost, key: &str, fallback: &str) -> String {
    host.translate(key).unwrap_or_else(|| fallback.to_owned())
}

pub fn crusher_screen() -> Screen {
    Screen {
        width: 176.0,
        height: 194.0,
        slots: vec![Some([24.0, 43.0]), Some([134.0, 43.0])],
        inventory_y: 112.0,
        inventory_x: 8.0,
        own_labels: true,
        own_background: true,
        ..Screen::default()
    }
}

pub fn crusher_draw(host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
    let [ox, oy] = event.origin;
    let draw = &mut *event.draw;
    draw.sprite(CRUSHER_GUI, [0.0, 0.0, 176.0, 194.0], [ox, oy, 176.0, 194.0], None);
    let title = lang(host, "container.rust_industries.crusher", "Crusher");
    draw.text(&title, [ox + 8.0, oy + 6.0], OPAQUE | 0xE0E0E0, false);
    let input = lang(host, "rust_industries.gui.input", "INPUT");
    let output = lang(host, "rust_industries.gui.output", "OUTPUT");
    draw.text(&input, [ox + 22.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    draw.text(&output, [ox + 128.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    draw_progress(host, event, ox, oy, "rust_industries.gui.crushing", "CRUSHING");
    let energy = event.data.get(3).copied().unwrap_or(0).max(0);
    let capacity = event.data.get(4).copied().unwrap_or(20_000).max(1);
    draw.text(&format!("FE {energy}/{capacity}"), [ox + 88.0, oy + 73.0], OPAQUE | 0x79C9F5, true);
}

/// One-slot GUI for the coal-powered generator.
pub fn generator_screen() -> Screen {
    Screen {
        width: 176.0,
        height: 202.0,
        slots: vec![Some([80.0, 43.0])],
        inventory_y: 120.0,
        inventory_x: 8.0,
        own_labels: true,
        own_background: true,
        ..Screen::default()
    }
}

pub fn generator_draw(host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
    let [ox, oy] = event.origin;
    let draw = &mut *event.draw;
    draw.sprite(GENERATOR_GUI, [0.0, 0.0, 176.0, 202.0], [ox, oy, 176.0, 202.0], None);
    let title = lang(host, "container.rust_industries.coal_generator", "Coal Generator");
    draw.text(&title, [ox + 8.0, oy + 6.0], OPAQUE | 0xE0E0E0, false);
    let fuel_label = lang(host, "rust_industries.gui.fuel", "FUEL");
    draw.text(&fuel_label, [ox + 76.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);

    let burn = event.data.first().copied().unwrap_or(0).max(0);
    let burn_max = event.data.get(1).copied().unwrap_or(1).max(1);
    let energy = event.data.get(2).copied().unwrap_or(0).max(0);
    let capacity = event.data.get(3).copied().unwrap_or(20_000).max(1);
    let status = event.data.get(4).copied().unwrap_or(0);
    let fraction = (burn as f32 / burn_max as f32).clamp(0.0, 1.0);
    let fill_width = 120.0 * fraction;
    if fill_width > 0.0 {
        draw.sprite(CRUSHER_GUI, [0.0, 194.0, 120.0, 8.0], [ox + 28.0, oy + 88.0, fill_width, 8.0], None);
    }
    draw.text(&format!("FE {energy}/{capacity}"), [ox + 88.0, oy + 73.0], OPAQUE | 0x79C9F5, true);
    draw.text(&format!("{}%", (fraction * 100.0).round() as i32), [ox + 80.0, oy + 101.0], OPAQUE | 0xE8E8E8, true);
    let (key, fallback, colour) = match status {
        1 => ("rust_industries.gui.generating", "GENERATING", 0x65D98B),
        2 => ("rust_industries.gui.buffer_full", "BUFFER FULL", 0xF0B45B),
        _ => ("rust_industries.gui.idle", "IDLE", 0xB7B7B7),
    };
    let line = lang(host, key, fallback);
    draw.text(&line, [ox + 112.0, oy + 8.0], OPAQUE | colour, true);
}

pub fn machine_screen(block: &str) -> Option<Screen> {
    match block {
        "rust_industries:millstone" | "rust_industries:mechanical_press" | "rust_industries:industrial_drill" => Some(Screen {
            width: 176.0, height: 202.0,
            slots: vec![Some([24.0, 43.0]), Some([134.0, 43.0])],
            inventory_y: 120.0, inventory_x: 8.0,
            own_labels: true, own_background: true,
            ..Screen::default()
        }),
        "rust_industries:mechanical_mixer" => Some(Screen {
            width: 176.0, height: 202.0,
            slots: vec![Some([20.0, 43.0]), Some([68.0, 43.0]), Some([134.0, 43.0])],
            inventory_y: 120.0, inventory_x: 8.0,
            own_labels: true, own_background: true,
            ..Screen::default()
        }),
        _ => None,
    }
}

pub fn machine_draw(host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
    let block = event.block;
    let (gui, title_key, title_fallback, status_key, status_fallback) = if block == "rust_industries:millstone" {
        ("rust_industries:gui/millstone", "container.rust_industries.millstone", "Millstone", "rust_industries.gui.stone", "STONE")
    } else if block == "rust_industries:mechanical_press" {
        ("rust_industries:gui/press", "container.rust_industries.mechanical_press", "Mechanical Press", "rust_industries.gui.pressing", "PRESSING")
    } else if block == "rust_industries:mechanical_mixer" {
        ("rust_industries:gui/mixer", "container.rust_industries.mechanical_mixer", "Mechanical Mixer", "rust_industries.gui.mixing", "MIXING")
    } else if block == "rust_industries:industrial_drill" {
        ("rust_industries:gui/drill", "container.rust_industries.industrial_drill", "Industrial Drill", "rust_industries.gui.drilling", "DRILLING")
    } else { return; };
    let [ox, oy] = event.origin;
    let draw = &mut *event.draw;
    draw.sprite(gui, [0.0, 0.0, 176.0, 202.0], [ox, oy, 176.0, 202.0], None);
    let title = lang(host, title_key, title_fallback);
    draw.text(&title, [ox + 8.0, oy + 6.0], OPAQUE | 0xE0E0E0, false);
    if block == "rust_industries:mechanical_mixer" {
        let input_a = lang(host, "rust_industries.gui.input_a", "INPUT A");
        let input_b = lang(host, "rust_industries.gui.input_b", "INPUT B");
        draw.text(&input_a, [ox + 15.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
        draw.text(&input_b, [ox + 63.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    } else {
        let input = lang(host, "rust_industries.gui.input", "INPUT");
        draw.text(&input, [ox + 22.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    }
    let output = lang(host, "rust_industries.gui.output", "OUTPUT");
    draw.text(&output, [ox + 128.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    draw_progress(host, event, ox, oy, status_key, status_fallback);
}

pub fn depot_screen() -> Screen {
    let mut slots = Vec::with_capacity(9);
    for row in 0..3 {
        for col in 0..3 {
            slots.push(Some([8.0 + col as f32 * 18.0, 30.0 + row as f32 * 18.0]));
        }
    }
    Screen {
        width: 176.0, height: 202.0,
        slots,
        inventory_y: 120.0, inventory_x: 8.0,
        own_labels: true, own_background: true,
        ..Screen::default()
    }
}

pub fn depot_draw(host: &dyn ClientHost, event: &mut ScreenDrawEvent<'_>) {
    let [ox, oy] = event.origin;
    let draw = &mut *event.draw;
    draw.sprite("rust_industries:gui/depot", [0.0, 0.0, 176.0, 202.0], [ox, oy, 176.0, 202.0], None);
    let title = lang(host, "container.rust_industries.depot", "Industrial Depot");
    draw.text(&title, [ox + 8.0, oy + 6.0], OPAQUE | 0xE0E0E0, false);
    let storage = lang(host, "rust_industries.gui.storage", "STORAGE");
    draw.text(&storage, [ox + 64.0, oy + 36.0], OPAQUE | 0xC5C5C5, true);
}

fn draw_progress(
    host: &dyn ClientHost,
    event: &mut ScreenDrawEvent<'_>,
    ox: f32,
    oy: f32,
    active_key: &str,
    active_fallback: &str,
) {
    let draw = &mut *event.draw;
    let progress = event.data.first().copied().unwrap_or(0).max(0);
    let maximum = event.data.get(1).copied().unwrap_or(1).max(1);
    let status = event.data.get(2).copied().unwrap_or(0);
    let fraction = (progress as f32 / maximum as f32).clamp(0.0, 1.0);
    let fill_width = 120.0 * fraction;
    if fill_width > 0.0 {
        draw.sprite("rust_industries:gui/crusher", [0.0, 194.0, 120.0, 8.0], [ox + 28.0, oy + 88.0, fill_width, 8.0], None);
    }
    draw.text(&format!("{}%", (fraction * 100.0).round() as i32), [ox + 80.0, oy + 101.0], OPAQUE | 0xE8E8E8, true);
    let (key, fallback, colour) = match status {
        1 => (active_key, active_fallback, 0x65D98B),
        2 => ("rust_industries.gui.output_full", "OUTPUT FULL", 0xF0B45B),
        3 => ("rust_industries.gui.invalid_input", "INVALID INPUT", 0xF07878),
        4 => ("rust_industries.gui.no_energy", "NO POWER", 0xF07878),
        _ => ("rust_industries.gui.ready", "READY", 0xB7B7B7),
    };
    let line = lang(host, key, fallback);
    draw.text(&line, [ox + 112.0, oy + 8.0], OPAQUE | colour, true);
}

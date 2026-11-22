//! Client-side Crusher screen.

use mod_api::{ClientHost, Screen, ScreenDrawEvent};

const GUI: &str = "rust_industries:gui/crusher";
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
    draw.sprite(GUI, [0.0, 0.0, 176.0, 194.0], [ox, oy, 176.0, 194.0], None);

    let title = lang(host, "container.rust_industries.crusher", "Crusher");
    draw.text(&title, [ox + 8.0, oy + 6.0], OPAQUE | 0xE0E0E0, false);
    let input_label = lang(host, "rust_industries.gui.input", "INPUT");
    let output_label = lang(host, "rust_industries.gui.output", "OUTPUT");
    draw.text(&input_label, [ox + 22.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);
    draw.text(&output_label, [ox + 128.0, oy + 31.0], OPAQUE | 0xC5C5C5, true);

    let progress = event.data.first().copied().unwrap_or(0).clamp(0, 40);
    let maximum = event.data.get(1).copied().unwrap_or(40).max(1);
    let status = event.data.get(2).copied().unwrap_or(0);
    let fraction = (progress as f32 / maximum as f32).clamp(0.0, 1.0);
    let fill_width = 120.0 * fraction;
    if fill_width > 0.0 {
        draw.sprite(GUI, [0.0, 194.0, 120.0, 8.0], [ox + 28.0, oy + 88.0, fill_width, 8.0], None);
    }
    let percent = (fraction * 100.0).round() as i32;
    draw.text(&format!("{percent}%"), [ox + 80.0, oy + 101.0], OPAQUE | 0xE8E8E8, true);

    let (key, fallback, colour) = match status {
        1 => ("rust_industries.gui.crushing", "CRUSHING", 0x65D98B),
        2 => ("rust_industries.gui.output_full", "OUTPUT FULL", 0xF0B45B),
        3 => ("rust_industries.gui.invalid_input", "INVALID INPUT", 0xF07878),
        _ => ("rust_industries.gui.idle", "IDLE", 0xB7B7B7),
    };
    let line = lang(host, key, fallback);
    draw.text(&line, [ox + 112.0, oy + 8.0], OPAQUE | colour, true);
}

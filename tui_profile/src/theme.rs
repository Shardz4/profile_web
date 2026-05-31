// ─── Theme System ───────────────────────────────────────────────────────────
use crate::platform::Color;

pub const THEME_COUNT: usize = 6;
pub const THEME_NAMES: [&str; THEME_COUNT] = [
    "Cyber Neon",
    "Retro Amber",
    "Matrix Green",
    "Vapor Wave",
    "Dracula",
    "Nord Frost",
];

/// Compute the accent color for a given theme and tick count.
pub fn get_accent_color(theme_idx: usize, tick_count: u64) -> Color {
    let t = tick_count as f64 * 0.02;
    match theme_idx {
        0 => { // Cyber Neon — cycling green/teal
            let g = ((t.sin() * 40.0) + 215.0) as u8;
            let b = (((t + 3.14).sin() * 40.0) + 215.0) as u8;
            Color::Rgb(0, g, b)
        }
        1 => { // Retro Amber — warm amber phosphor
            let v = ((t.sin() * 20.0) + 235.0) as u8;
            Color::Rgb(v, ((v as f64) * 0.69) as u8, 0)
        }
        2 => { // Matrix Green — pure neon green
            let v = ((t.sin() * 30.0) + 225.0) as u8;
            Color::Rgb(0, v, ((v as f64) * 0.25) as u8)
        }
        3 => { // Vapor Wave — pink/purple cycling
            let r = ((t.sin() * 30.0) + 225.0) as u8;
            let b = (((t + 2.0).sin() * 40.0) + 215.0) as u8;
            Color::Rgb(r, ((r as f64) * 0.44) as u8, b)
        }
        4 => { // Dracula — purple accent
            let v = ((t.sin() * 25.0) + 230.0) as u8;
            Color::Rgb(((v as f64) * 0.74) as u8, ((v as f64) * 0.57) as u8, v)
        }
        5 => { // Nord Frost — cool blue/cyan
            let b = ((t.sin() * 25.0) + 220.0) as u8;
            let g = (((t + 1.5).sin() * 20.0) + 190.0) as u8;
            Color::Rgb(((b as f64) * 0.53) as u8, g, b)
        }
        _ => Color::White,
    }
}

/// Get the name of a theme by index.
pub fn get_theme_name(theme_idx: usize) -> &'static str {
    THEME_NAMES[theme_idx % THEME_COUNT]
}

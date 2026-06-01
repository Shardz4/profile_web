// ─── UI Rendering Modules ───────────────────────────────────────────────────
pub mod boot;
pub mod chrome;
pub mod home;
pub mod projects;
pub mod skills;
pub mod contact;

use crate::platform::*;
use crate::{App, BG};

// ─── Root UI ────────────────────────────────────────────────────────────────
pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let accent = app.get_accent_color();

    // Render particle background as the very first layer
    render_particles(f, app);

    if app.boot_mode {
        boot::render_boot_screen(f, app, accent);
    } else {
        // Outer chrome
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(accent))
            .style(Style::default().bg(BG));
        f.render_widget(outer, size);

        // Inner layout: header | tabs | body | footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // header
                Constraint::Length(3), // tabs
                Constraint::Min(0),    // body
                Constraint::Length(1), // footer
            ])
            .split(size);

        chrome::render_header(f, chunks[0], accent, app.tick_count);
        chrome::render_tabs(f, chunks[1], app, accent);
        render_body(f, chunks[2], app, accent);
        chrome::render_footer(f, chunks[3], app, accent);
    }

    if app.konami_active {
        render_konami_overlay(f, app);
    }

    // Achievement toast overlay (top-right corner)
    if let Some((ref name, ref desc, _until)) = app.achievement_toast {
        render_achievement_toast(f, app, name, desc);
    }
}

// ─── Body dispatcher ────────────────────────────────────────────────────────
fn render_body(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    match app.tab_index {
        0 => home::render_home(f, area, app, accent),
        1 => projects::render_projects(f, area, accent),
        2 => skills::render_skills(f, area, app, accent),
        3 => contact::render_contact(f, area, accent),
        _ => {}
    }
}

pub fn make_progress_bar(val: f64, width: usize) -> String {
    let filled = ((width * val as usize) / 100).min(width);
    let empty = width.saturating_sub(filled);
    "█".repeat(filled) + &"░".repeat(empty)
}

// ─── Particles ──────────────────────────────────────────────────────────────
fn render_particles(f: &mut Frame, app: &App) {
    let area = f.area();
    let accent = app.get_accent_color();
    // Extract accent RGB for tinting particles
    let (ar, ag, ab) = match accent {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 200, 180),
    };
    let buf = f.buffer_mut();
    for p in &app.particles {
        let px = p.x as u16;
        let py = p.y as u16;
        // Only draw within visible area
        if px >= area.x && px < area.x + area.width && py >= area.y && py < area.y + area.height {
            if let Some(cell) = buf.cell_mut((px, py)) {
                // Only render into empty/space cells so particles don't overwrite content
                let sym = cell.symbol().to_string();
                if sym == " " || sym.is_empty() {
                    let dim = (p.brightness as f32) / 255.0 * 0.35; // keep subtle
                    cell.set_char(p.ch);
                    cell.set_fg(Color::Rgb(
                        (ar as f32 * dim) as u8,
                        (ag as f32 * dim) as u8,
                        (ab as f32 * dim) as u8,
                    ));
                }
            }
        }
    }
}

// ─── Konami Overlay ─────────────────────────────────────────────────────────
fn render_konami_overlay(f: &mut Frame, app: &App) {
    let area = f.area();
    
    // We want a centered box of size 70 wide, 14 tall
    let popup_area = centered_rect_fixed(70, 14, area);

    // Color cycling for flashy NES look
    let pulse = (app.tick_count / 15) % 3;
    let (border_color, text_color) = match pulse {
        0 => (Color::Rgb(255, 0, 255), Color::Rgb(255, 215, 0)), // Magenta border, Gold text
        1 => (Color::Rgb(255, 215, 0), Color::Rgb(0, 255, 255)), // Gold border, Cyan text
        _ => (Color::Rgb(0, 255, 255), Color::Rgb(255, 0, 255)), // Cyan border, Magenta text
    };

    // Draw the overlay block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG))
        .title(Span::styled(" ♥ CHEAT CODE ACTIVATED ♥ ", Style::default().fg(border_color).add_modifier(Modifier::BOLD)));
    
    f.render_widget(block.clone(), popup_area);

    // Get the inner area of the block
    let inner_area = block.inner(popup_area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // spacing
            Constraint::Length(6), // ASCII art
            Constraint::Length(1), // spacing
            Constraint::Length(1), // Sub-text
            Constraint::Length(1), // Blinking prompt
            Constraint::Min(0),
        ])
        .split(inner_area);

    // ASCII art text lines
    let ascii_art = r#" ██████╗  ██████╗     ██╗     ██╗██╗   ██╗███████╗███████╗██╗
██╔═══██╗██╔═══██╗    ██║     ██║██║   ██║██╔════╝██╔════╝██║
 ▄▄▄▄██║██║   ██║    ██║     ██║██║   ██║█████╗  ███████╗██║
 ▀▀▀▀██║██║   ██║    ██║     ██║╚██╗ ██╔╝██╔══╝  ╚════██║╚═╝
██████╔╝╚██████╔╝    ███████╗██║ ╚████╔╝ ███████╗███████║██╗
╚═════╝  ╚═════╝     ╚══════╝╚═╝  ╚═══╝  ╚══════╝╚══════╝╚═╝"#;

    let ascii_lines: Vec<Line> = ascii_art
        .lines()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(text_color).add_modifier(Modifier::BOLD))))
        .collect();

    let ascii_para = Paragraph::new(ascii_lines).alignment(Alignment::Center);
    f.render_widget(ascii_para, chunks[1]);

    // Sub-text
    let sub_text = Paragraph::new(Line::from(vec![
        Span::styled("CONTRA 1986 // RETRO CHEAT MODE ENABLED", Style::default().fg(Color::Gray)),
    ])).alignment(Alignment::Center);
    f.render_widget(sub_text, chunks[3]);

    // Blinking prompt
    let show_prompt = (app.tick_count / 30) % 2 == 0;
    let prompt_span = if show_prompt {
        Span::styled("★★★★★ 30 LIVES GRANTED ★★★★★", Style::default().fg(border_color).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("", Style::default())
    };
    
    let prompt_para = Paragraph::new(Line::from(vec![prompt_span])).alignment(Alignment::Center);
    f.render_widget(prompt_para, chunks[4]);
}

// ─── Achievement Toast ──────────────────────────────────────────────────────
fn render_achievement_toast(f: &mut Frame, app: &App, name: &str, desc: &str) {
    let area = f.area();
    let toast_w: u16 = 34;
    let toast_h: u16 = 7;
    // Position in top-right corner with 2-cell margin
    let x = area.width.saturating_sub(toast_w + 2);
    let y = 2;
    let toast_area = Rect::new(x, y, toast_w.min(area.width), toast_h.min(area.height));

    // Pulsing border color
    let pulse = (app.tick_count / 20) % 3;
    let border_color = match pulse {
        0 => Color::Rgb(255, 215, 0),  // Gold
        1 => Color::Rgb(0, 255, 200),  // Cyan-green
        _ => Color::Rgb(180, 130, 255), // Lavender
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG));
    let inner = block.inner(toast_area);
    f.render_widget(block, toast_area);

    let blink = (app.tick_count / 25) % 2 == 0;
    let star = if blink { "★" } else { "☆" };

    let lines = vec![
        Line::from(Span::styled(
            format!(" {} Achievement Unlocked {}", star, star),
            Style::default().fg(border_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  \"{}\" ", name),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", desc),
            Style::default().fg(Color::Gray),
        )),
    ];

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

// ─── Layout Helper ──────────────────────────────────────────────────────────
pub fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}

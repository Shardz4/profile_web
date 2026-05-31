// ─── Tab: Skills ─────────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{DIM, FG, NARROW};

pub fn render_skills(f: &mut Frame, area: Rect, accent: Color) {
    let width = area.width;

    let systems_skills = vec![
        ("Rust", 92),
        ("C / C++", 88),
        ("Linux Kernel", 85),
        ("Bare-Metal OS", 80),
        ("Memory Mgmt", 84),
        ("x86_64 Assembly", 70),
        ("CUDA / GPU Arch", 72),
    ];

    let ml_skills = vec![
        ("Python", 95),
        ("PyTorch", 90),
        ("OpenCV", 86),
        ("TensorRT", 80),
        ("Deep Learning", 88),
        ("Computer Vision", 85),
        ("Model Optimization", 78),
    ];

    let tools_skills = vec![
        ("Git / GitHub", 95),
        ("Distributed Systems", 82),
        ("Competitive Prog.", 88),
        ("Linux / Shell", 86),
        ("Docker", 80),
        ("CI/CD", 78),
        ("Vim / Neovim", 90),
    ];

    if width < NARROW {
        // Narrow: stack all 3 skill groups vertically
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(area);
        render_skill_col(f, chunks[0], " systems ", &systems_skills, accent);
        render_skill_col(f, chunks[1], " ml & ai ", &ml_skills, accent);
        render_skill_col(f, chunks[2], " tools ", &tools_skills, accent);
    } else {
        // Normal: 3-column horizontal
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);
        render_skill_col(f, cols[0], " systems ", &systems_skills, accent);
        render_skill_col(f, cols[1], " ml & ai ", &ml_skills, accent);
        render_skill_col(f, cols[2], " tools ", &tools_skills, accent);
    }
}

fn render_skill_col(f: &mut Frame, area: Rect, title: &str, skills: &[(&str, u8)], accent: Color) {
    let bar_width = (area.width as usize).saturating_sub(14).max(4);

    let items: Vec<ListItem> = skills
        .iter()
        .flat_map(|(name, pct)| {
            let filled = (bar_width * (*pct as usize)) / 100;
            let empty = bar_width - filled;
            let bar: String = "█".repeat(filled) + &"░".repeat(empty);
            vec![
                ListItem::new(Line::from(Span::styled(
                    format!("  {}", name),
                    Style::default().fg(FG).add_modifier(Modifier::BOLD),
                ))),
                ListItem::new(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(bar, Style::default().fg(accent)),
                    Span::styled(format!(" {}%", pct), Style::default().fg(DIM)),
                ])),
                ListItem::new(Line::from("")),
            ]
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(Span::styled(title, Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    );
    f.render_widget(list, area);
}

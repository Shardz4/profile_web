// ─── Tab: Contact ────────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG, NARROW};

pub fn render_contact(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let width = area.width;

    let contact_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Let's build something interesting.",
            Style::default().fg(FG).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✉  Email   ", Style::default().fg(accent)),
            Span::styled("arnav4324@gmail.com", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ⌘  GitHub  ", Style::default().fg(accent)),
            Span::styled("github.com/Shardz4", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ∟  LinkedIn", Style::default().fg(accent)),
            Span::styled("linkedin.com/in/arnav-sharma-z/", Style::default().fg(FG)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Open to: research collaborations, systems consulting,",
            Style::default().fg(DIM),
        )),
        Line::from(Span::styled(
            "  ML engineering roles, and interesting side projects.",
            Style::default().fg(DIM),
        )),
    ];

    let avail_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ╔─ availability ──────╗",
            Style::default().fg(accent),
        )),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(accent)),
            Span::styled("Status   ", Style::default().fg(DIM)),
            Span::styled("Open             ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(accent)),
        ]),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(accent)),
            Span::styled("Timezone ", Style::default().fg(DIM)),
            Span::styled("UTC+5:30         ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(accent)),
        ]),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(accent)),
            Span::styled("Response ", Style::default().fg(DIM)),
            Span::styled("~24 hours        ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(accent)),
        ]),
        Line::from(Span::styled(
            "  ╚──────────────────────╝",
            Style::default().fg(accent),
        )),
        Line::from(""),
        Line::from(Span::styled("  Location:", Style::default().fg(DIM))),
        Line::from(Span::styled("  Solan, HP, India", Style::default().fg(FG))),
    ];

    // Build achievements lines
    let mut achievement_lines = vec![Line::from("")];
    for ach in &app.achievements {
        if ach.unlocked {
            achievement_lines.push(Line::from(vec![
                Span::styled(format!("  {}  ", ach.icon), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                Span::styled(ach.name, Style::default().fg(FG).add_modifier(Modifier::BOLD)),
                Span::styled(" [UNLOCKED]", Style::default().fg(Color::Yellow)),
            ]));
            achievement_lines.push(Line::from(Span::styled(
                format!("     {}", ach.description),
                Style::default().fg(FG),
            )));
        } else {
            achievement_lines.push(Line::from(vec![
                Span::styled("  🔒  ", Style::default().fg(DIM)),
                Span::styled(ach.name, Style::default().fg(DIM).add_modifier(Modifier::BOLD)),
                Span::styled(" [LOCKED]", Style::default().fg(DIM)),
            ]));
            achievement_lines.push(Line::from(Span::styled(
                format!("     {}", ach.description),
                Style::default().fg(DIM),
            )));
        }
        achievement_lines.push(Line::from("")); // spacer
    }

    if width < NARROW {
        // Narrow layout: Stack contact, avail, and achievements vertically
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12), // Contact
                Constraint::Length(10), // Avail/Meta
                Constraint::Min(0),     // Achievements
            ])
            .split(area);

        let contact = Paragraph::new(contact_lines)
            .block(Block::default()
                .title(Span::styled(" contact ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(contact, chunks[0]);

        let avail = Paragraph::new(avail_lines)
            .block(Block::default()
                .title(Span::styled(" meta ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)));
        f.render_widget(avail, chunks[1]);

        let trophies = Paragraph::new(achievement_lines)
            .block(Block::default()
                .title(Span::styled(" trophies & achievements ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(trophies, chunks[2]);
    } else {
        // Normal layout: 50% left (contact & avail vertically split), 50% right (achievements)
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(55), // Contact
                Constraint::Percentage(45), // Meta
            ])
            .split(cols[0]);

        let contact = Paragraph::new(contact_lines)
            .block(Block::default()
                .title(Span::styled(" contact ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(contact, left_chunks[0]);

        let avail = Paragraph::new(avail_lines)
            .block(Block::default()
                .title(Span::styled(" meta ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)));
        f.render_widget(avail, left_chunks[1]);

        let trophies = Paragraph::new(achievement_lines)
            .block(Block::default()
                .title(Span::styled(" trophies & achievements ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(trophies, cols[1]);
    }
}

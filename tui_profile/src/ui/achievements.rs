// ─── Tab: Achievements ───────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG};

pub fn render_achievements(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Stats panel
            Constraint::Min(0),    // Badges list panel
        ])
        .split(area);

    let unlocked_count = app.achievements.iter().filter(|a| a.unlocked).count();
    let pct = (unlocked_count as f64 / 5.0) * 100.0;

    // Render Stats Panel
    let bar_w = (chunks[0].width as usize).saturating_sub(32).max(10);
    let progress_bar_str = crate::ui::make_progress_bar(pct, bar_w);

    let stats_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Trophy Cabinet: ", Style::default().fg(DIM)),
            Span::styled(progress_bar_str, Style::default().fg(accent)),
            Span::styled(format!(" {}/5 Badges Unlocked ({:.0}%)", unlocked_count, pct), Style::default().fg(FG).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let stats_block = Paragraph::new(stats_lines).block(
        Block::default()
            .title(Span::styled(" progress statistics ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    );
    f.render_widget(stats_block, chunks[0]);

    // Render Badges Panel
    let mut badge_lines = vec![Line::from("")];
    for ach in &app.achievements {
        if ach.unlocked {
            badge_lines.push(Line::from(vec![
                Span::styled(format!("  {}  ", ach.icon), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                Span::styled(ach.name, Style::default().fg(FG).add_modifier(Modifier::BOLD)),
                Span::styled(" [UNLOCKED]", Style::default().fg(Color::Yellow)),
            ]));
            badge_lines.push(Line::from(Span::styled(
                format!("     {}", ach.description),
                Style::default().fg(FG),
            )));
        } else {
            badge_lines.push(Line::from(vec![
                Span::styled("  🔒  ", Style::default().fg(DIM)),
                Span::styled(ach.name, Style::default().fg(DIM).add_modifier(Modifier::BOLD)),
                Span::styled(" [LOCKED]", Style::default().fg(DIM)),
            ]));
            badge_lines.push(Line::from(Span::styled(
                format!("     {}", ach.description),
                Style::default().fg(DIM),
            )));
        }
        badge_lines.push(Line::from("")); // spacer
    }

    let badges_block = Paragraph::new(badge_lines)
        .block(
            Block::default()
                .title(Span::styled(" unlocked trophies & badges ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(badges_block, chunks[1]);
}

// ─── Chrome: Header, Tabs, Footer ───────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG};
use crate::effects::glitch_str;

// ─── Header ─────────────────────────────────────────────────────────────────
pub fn render_header(f: &mut Frame, area: Rect, accent: Color, tick: u64) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("▸ ", Style::default().fg(accent)),
            Span::styled(
                glitch_str("ARNAV SHARMA", tick),
                Style::default()
                    .fg(FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  ·  Systems Architect  |  Machine Learning Engineer  |  Low-Level Enthusiast",
                Style::default().fg(DIM),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(DIM)),
    )
    .alignment(Alignment::Left);
    f.render_widget(header, area);
}

// ─── Tabs ────────────────────────────────────────────────────────────────────
pub fn render_tabs(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let titles: Vec<Line> = app
        .tab_titles
        .iter()
        .map(|t| Line::from(Span::raw(*t)))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(DIM)),
        )
        .select(app.tab_index)
        .style(Style::default().fg(DIM))
        .highlight_style(
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )
        .divider(Span::styled(" │ ", Style::default().fg(DIM)));

    f.render_widget(tabs, area);
}

// ─── Footer ──────────────────────────────────────────────────────────────────
pub fn render_footer(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let mut footer_spans = if app.project_detail_view && app.tab_index == 1 {
        // Project detail view
        vec![
            Span::styled(" Esc ", Style::default().fg(accent)),
            Span::styled("back", Style::default().fg(DIM)),
            Span::styled("  │  ", Style::default().fg(DIM)),
            Span::styled("Enter ", Style::default().fg(accent)),
            Span::styled("github", Style::default().fg(DIM)),
            Span::styled("  │  ", Style::default().fg(DIM)),
            Span::styled("↑↓ ", Style::default().fg(accent)),
            Span::styled("scroll", Style::default().fg(DIM)),
        ]
    } else if app.tab_index == 1 {
        // Project list view
        vec![
            Span::styled(" ↑↓ ", Style::default().fg(accent)),
            Span::styled("select", Style::default().fg(DIM)),
            Span::styled("  │  ", Style::default().fg(DIM)),
            Span::styled("←→ ", Style::default().fg(accent)),
            Span::styled("focus", Style::default().fg(DIM)),
            Span::styled("  │  ", Style::default().fg(DIM)),
            Span::styled("Enter ", Style::default().fg(accent)),
            Span::styled("action", Style::default().fg(DIM)),
        ]
    } else if app.tab_index == 2 {
        vec![
            Span::styled(" Tab ", Style::default().fg(accent)),
            Span::styled("change tab", Style::default().fg(DIM)),
            Span::styled("  │  ", Style::default().fg(DIM)),
            Span::styled("Arrows/WASD ", Style::default().fg(accent)),
            Span::styled("navigate graph", Style::default().fg(DIM)),
        ]
    } else {
        vec![
            Span::styled(" ← → ", Style::default().fg(accent)),
            Span::styled("navigate", Style::default().fg(DIM)),
        ]
    };

    footer_spans.extend(vec![
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("1-4 ", Style::default().fg(accent)),
        Span::styled("jump", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("t ", Style::default().fg(accent)),
        Span::styled("theme", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("f ", Style::default().fg(accent)),
        Span::styled("fullscreen", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("q ", Style::default().fg(accent)),
        #[cfg(target_arch = "wasm32")]
        Span::styled("exit fullscreen", Style::default().fg(DIM)),
        #[cfg(not(target_arch = "wasm32"))]
        Span::styled("quit", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled(format!("◆ {}", app.get_theme_name()), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled(format!("FPS: {:.1}", app.fps_tracker.fps), Style::default().fg(Color::Yellow)),
    ]);

    let footer = Paragraph::new(Line::from(footer_spans))
        .alignment(Alignment::Center);
    f.render_widget(footer, area);
}

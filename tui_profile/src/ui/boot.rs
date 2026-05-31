// ─── Boot UI Screen ──────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, BG, DIM, NARROW};
use crate::effects::glitch_str;
use crate::widgets::MarioWidget;

pub fn render_boot_screen(f: &mut Frame, app: &App, accent: Color) {
    let size = f.area();

    // Draw outer double borders
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(accent))
        .style(Style::default().bg(BG));
    f.render_widget(outer, size);

    // Dynamic grid layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),  // Header / Title
            Constraint::Min(0),     // Middle Widgets (BarChart & Chart)
            Constraint::Length(6),  // Sparkline
            Constraint::Length(14), // Mario + Boot Button
        ])
        .split(size);

    // 1. Render Header (Logo + Title side-by-side)
    let header_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Logo + Title row
            Constraint::Length(1), // Separator row
            Constraint::Min(0),
        ])
        .split(main_chunks[0]);

    let header_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(14), // Width of the smaller logo
            Constraint::Min(0),     // Rest of the space for Title + FPS
        ])
        .split(header_rows[0]);

    // Logo on the left
    let logo_text = r#" ▄▀▀▄ ▄▀▀▀ 
 █▄▄█ ▀▀▀▄ 
 █  █ ▄▄▄▀ "#;
    let logo_para = Paragraph::new(logo_text)
        .style(Style::default().fg(accent).add_modifier(Modifier::BOLD));
    f.render_widget(logo_para, header_cols[0]);

    // Title + FPS on the right, vertically centered (shifted down 1 line)
    let title_lines = vec![
        Line::from(""), // Empty line for vertical alignment
        Line::from(vec![
            Span::styled(glitch_str(" ▸ ARNAV SHARMA PORTFOLIO ", app.tick_count), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" [FPS: {:.1}]", app.fps_tracker.fps), Style::default().fg(Color::Yellow)),
        ]),
    ];
    let title_para = Paragraph::new(title_lines);
    f.render_widget(title_para, header_cols[1]);

    // Horizontal separator
    let separator = Paragraph::new(Line::from(Span::styled(" ──────────────────────────────────────────────────────────────────────────", Style::default().fg(DIM))));
    f.render_widget(separator, header_rows[1]);

    // 2. Middle Row: BarChart & Dual Sine Chart
    if size.width < NARROW {
        // Narrow: stack vertically
        let mid_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

        // BarChart on top
        let bar_values: Vec<u64> = (0..6).map(|i| {
            let t = app.tick_count as f64 * 0.12;
            let base = 50.0 + 40.0 * (t + i as f64 * 0.6).sin();
            let noise = ((app.tick_count + i as u64) % 7) as f64;
            (base + noise).clamp(5.0, 95.0) as u64
        }).collect();

        let bar_data = [
            ("C1", bar_values[0]),
            ("C2", bar_values[1]),
            ("C3", bar_values[2]),
            ("C4", bar_values[3]),
            ("C5", bar_values[4]),
            ("C6", bar_values[5]),
        ];

        let barchart = BarChart::default()
            .block(Block::default()
                .title(Span::styled(" core voltages ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .data(&bar_data)
            .bar_width(2)
            .bar_gap(1)
            .value_style(Style::default().fg(Color::Black).bg(accent))
            .bar_style(Style::default().fg(accent));
        f.render_widget(barchart, mid_chunks[0]);

        // Sine chart on bottom
        let t = app.tick_count as f64 * 0.08;
        let mut wave1 = Vec::new();
        for i in 0..60 {
            let x = (i as f64) * 0.25;
            let y1 = (x - t).sin();
            wave1.push((x, y1));
        }
        let dataset1 = Dataset::default()
            .name("SIG_A")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&wave1);
        let chart = Chart::new(vec![dataset1])
            .block(Block::default()
                .title(Span::styled(" signal ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .x_axis(Axis::default().style(Style::default().fg(DIM)).bounds([0.0, 15.0]))
            .y_axis(Axis::default().style(Style::default().fg(DIM)).bounds([-1.2, 1.2]));
        f.render_widget(chart, mid_chunks[1]);
    } else {
        // Normal side-by-side layout
        let mid_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

    // 2a. BarChart Data (animated via tick_count)
    let bar_values: Vec<u64> = (0..12).map(|i| {
        let t = app.tick_count as f64 * 0.12;
        let base = 50.0 + 40.0 * (t + i as f64 * 0.6).sin();
        let noise = ((app.tick_count + i as u64) % 7) as f64;
        (base + noise).clamp(5.0, 95.0) as u64
    }).collect();

    let bar_data = [
        ("C1", bar_values[0]),
        ("C2", bar_values[1]),
        ("C3", bar_values[2]),
        ("C4", bar_values[3]),
        ("C5", bar_values[4]),
        ("C6", bar_values[5]),
        ("C7", bar_values[6]),
        ("C8", bar_values[7]),
        ("C9", bar_values[8]),
        ("CA", bar_values[9]),
        ("CB", bar_values[10]),
        ("CC", bar_values[11]),
    ];

    let barchart = BarChart::default()
        .block(Block::default()
            .title(Span::styled(" core voltages (120hz) ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)))
        .data(&bar_data)
        .bar_width(3)
        .bar_gap(1)
        .value_style(Style::default().fg(Color::Black).bg(accent))
        .bar_style(Style::default().fg(accent));
    f.render_widget(barchart, mid_chunks[0]);

    // 2b. Dual Sine Chart Data (animated via tick_count)
    let t = app.tick_count as f64 * 0.08;
    let mut wave1 = Vec::new();
    let mut wave2 = Vec::new();
    for i in 0..60 {
        let x = (i as f64) * 0.25;
        let y1 = (x - t).sin();
        let y2 = (1.5 * x + t).cos() * 0.8;
        wave1.push((x, y1));
        wave2.push((x, y2));
    }

    let dataset1 = Dataset::default()
        .name("SIG_A (sin)")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&wave1);

    let dataset2 = Dataset::default()
        .name("SIG_B (cos)")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&wave2);

    let chart = Chart::new(vec![dataset1, dataset2])
        .block(Block::default()
            .title(Span::styled(" sinusoidal phase telemetry ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)))
        .x_axis(Axis::default()
            .title("Time Phase")
            .style(Style::default().fg(DIM))
            .bounds([0.0, 15.0])
            .labels(vec![
                Span::styled("0", Style::default().fg(DIM)),
                Span::styled("7.5", Style::default().fg(DIM)),
                Span::styled("15", Style::default().fg(DIM)),
            ]))
        .y_axis(Axis::default()
            .title("Amp")
            .style(Style::default().fg(DIM))
            .bounds([-1.2, 1.2])
            .labels(vec![
                Span::styled("-1.0", Style::default().fg(DIM)),
                Span::styled("0", Style::default().fg(DIM)),
                Span::styled("1.0", Style::default().fg(DIM)),
            ]));
    f.render_widget(chart, mid_chunks[1]);
    } // end of normal (non-narrow) boot chart layout

    // 3. Sparkline Widget
    let sparkline = Sparkline::default()
        .block(Block::default()
            .title(Span::styled(" frequency spectrum analytics (120hz) ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)))
        .data(&app.sparkline_data)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, main_chunks[2]);

    // 4. Centered Boot Interactive prompt
    let boot_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Space for Mario running/jumping
            Constraint::Length(2),  // Bottom lines for prompt text
        ])
        .split(main_chunks[3]);

    let blink = app.tick_count % 20 < 10;
    let prompt_text = if blink {
        " ▸ ▸ ▸  [ CLICK ANYWHERE TO CONTINUE ]  ◂ ◂ ◂ "
    } else {
        "        [ CLICK ANYWHERE TO CONTINUE ]        "
    };

    let prompt = Paragraph::new(Line::from(vec![
        Span::styled(prompt_text, Style::default().fg(accent).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default()
        .borders(Borders::NONE))
    .alignment(Alignment::Center);
    f.render_widget(prompt, boot_chunks[1]);

    // Render Mario running and jumping across the boot prompt area!
    let mario = MarioWidget { tick_count: app.tick_count };
    f.render_widget(mario, boot_chunks[0]);
}

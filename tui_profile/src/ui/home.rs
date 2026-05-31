// ─── Tab: Home ───────────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG, NARROW};
use crate::events::ShellState;
use super::make_progress_bar;

pub fn render_home(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let width = area.width;

    if width < NARROW {
        // ── NARROW: Stack all panels vertically ──
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // bio (compact)
                Constraint::Min(0),     // shell simulator
            ])
            .split(area);

        // Compact bio
        let bio_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Hello, world. ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                Span::styled("I'm Arnav Sharma.", Style::default().fg(FG)),
            ]),
            Line::from(""),
            Line::from(Span::styled("  CSE Student · NIT Hamirpur", Style::default().fg(FG))),
            Line::from(Span::styled("  Systems · CV · Kernel Research", Style::default().fg(FG))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Based in: ", Style::default().fg(accent)),
                Span::styled("Solan, HP, India", Style::default().fg(FG)),
            ]),
        ];
        let bio = Paragraph::new(bio_lines)
            .block(Block::default()
                .title(Span::styled(" about ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(bio, chunks[0]);
        render_shell_simulator(f, chunks[1], app, accent);
    } else {
        // ── NORMAL / WIDE: 2-column layout ──
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(14), Constraint::Min(0)])
            .split(cols[0]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(15), Constraint::Min(0)])
            .split(cols[1]);

        // Left: bio (top) & Shell Simulator (bottom)
        let bio_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Hello, world. ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                Span::styled("I'm Arnav Sharma.", Style::default().fg(FG)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  A Computer Science & Engineering student focused on building",
                Style::default().fg(FG),
            )),
            Line::from(Span::styled(
                "  high-performance, safety-critical systems.",
                Style::default().fg(FG),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  I build things that run fast, learn well, and break gracefully.",
                Style::default().fg(DIM),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Currently: ", Style::default().fg(accent)),
                Span::styled("NIT Hamirpur (CSE Student)", Style::default().fg(FG)),
            ]),
            Line::from(vec![
                Span::styled("  Based in:  ", Style::default().fg(accent)),
                Span::styled("Solan, Himachal Pradesh, India", Style::default().fg(FG)),
            ]),
            Line::from(vec![
                Span::styled("  Focus:     ", Style::default().fg(accent)),
                Span::styled(
                    "Systems Programming · Computer Vision · Kernel Research",
                    Style::default().fg(FG),
                ),
            ]),
        ];

        let bio = Paragraph::new(bio_lines)
            .block(
                Block::default()
                    .title(Span::styled(" about ", Style::default().fg(DIM)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(DIM)),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(bio, left_chunks[0]);

        render_shell_simulator(f, left_chunks[1], app, accent);

        // Right: dynamic stats / simulated CPU monitor (top) & Matrix Rain (bottom)
        let box_width = cols[1].width as usize;
        let is_wide = box_width >= 40;

    let bar_width = if is_wide {
        ((box_width - 24) / 2).clamp(3, 10)
    } else {
        (box_width - 15).clamp(3, 15)
    };

    let mut stats_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("   OS   ", Style::default().fg(DIM)),
            Span::styled("Linux / Bare-metal OS", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("   Lang ", Style::default().fg(DIM)),
            Span::styled("Rust · Python · C/C++ · Assembly", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("   ML   ", Style::default().fg(DIM)),
            Span::styled("PyTorch · OpenCV · TensorRT", Style::default().fg(FG)),
        ]),
        Line::from(""),
        Line::from(Span::styled("  ┌─ Core Status ───────────────────────┐", Style::default().fg(accent))),
    ];

    if is_wide {
        // Render 2 columns: 4 rows of 2 cores
        for i in (0..8).step_by(2) {
            let cpu1 = app.cpu_cores.get(i).copied().unwrap_or(0.0);
            let cpu2 = app.cpu_cores.get(i + 1).copied().unwrap_or(0.0);
            let bar1 = make_progress_bar(cpu1, bar_width);
            let bar2 = make_progress_bar(cpu2, bar_width);
            
            let left_str = format!("C{} [", i + 1);
            let right_str = format!("C{} [", i + 2);
            
            stats_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(accent)),
                Span::styled(left_str, Style::default().fg(DIM)),
                Span::styled(bar1, Style::default().fg(accent)),
                Span::styled(format!("] {:>2.0}%", cpu1), Style::default().fg(FG)),
                Span::styled(" │ ", Style::default().fg(DIM)),
                Span::styled(right_str, Style::default().fg(DIM)),
                Span::styled(bar2, Style::default().fg(accent)),
                Span::styled(format!("] {:>2.0}%", cpu2), Style::default().fg(FG)),
                Span::styled("  │", Style::default().fg(accent)),
            ]));
        }
    } else {
        // Render 1 column: 4 cores to prevent overflow
        let max_rows = 4;
        for i in 0..max_rows {
            let cpu = app.cpu_cores.get(i).copied().unwrap_or(0.0);
            let bar = make_progress_bar(cpu, bar_width);
            let core_str = format!("Core {} [{}] {:>2.0}%", i + 1, bar, cpu);
            let core_len = core_str.len();
            stats_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(accent)),
                Span::styled(core_str, Style::default().fg(FG)),
                Span::styled(" ".repeat(box_width.saturating_sub(core_len + 6)), Style::default()),
                Span::styled(" │", Style::default().fg(accent)),
            ]));
        }
    }

    // GPU and VRAM info
    let gpu_bar = make_progress_bar(app.gpu_load, bar_width);
    let gpu_str = format!("GPU [{}] {:>2.0}%", gpu_bar, app.gpu_load);
    let gpu_padding = box_width.saturating_sub(gpu_str.len() + 6);
    stats_lines.push(Line::from(vec![
        Span::styled("  │ ", Style::default().fg(accent)),
        Span::styled(gpu_str, Style::default().fg(Color::Yellow)),
        Span::styled(" ".repeat(gpu_padding), Style::default()),
        Span::styled(" │", Style::default().fg(accent)),
    ]));

    let vram_bar = make_progress_bar((app.vram_usage / 8.0) * 100.0, bar_width);
    let vram_str = format!("VRM [{}] {:.2}G/8G", vram_bar, app.vram_usage);
    let vram_padding = box_width.saturating_sub(vram_str.len() + 6);
    stats_lines.push(Line::from(vec![
        Span::styled("  │ ", Style::default().fg(accent)),
        Span::styled(vram_str, Style::default().fg(Color::Magenta)),
        Span::styled(" ".repeat(vram_padding), Style::default()),
        Span::styled(" │", Style::default().fg(accent)),
    ]));

    // Average CPU usage
    let avg_cpu: f64 = app.cpu_cores.iter().sum::<f64>() / 8.0;
    let avg_bar = make_progress_bar(avg_cpu, bar_width);
    let avg_str = format!("AVG [{}] {:>2.0}%", avg_bar, avg_cpu);
    let avg_padding = box_width.saturating_sub(avg_str.len() + 6);
    stats_lines.push(Line::from(vec![
        Span::styled("  │ ", Style::default().fg(accent)),
        Span::styled(avg_str, Style::default().fg(FG)),
        Span::styled(" ".repeat(avg_padding), Style::default()),
        Span::styled(" │", Style::default().fg(accent)),
    ]));

    // Render rolling system load chart
    let history_width = box_width.saturating_sub(16).max(10);
    let mut history_str = String::new();
    let chart_chars = [" ", " ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    let pad_len = history_width.saturating_sub(app.cpu_history.len());
    for _ in 0..pad_len {
        history_str.push(' ');
    }
    let history_slice = if app.cpu_history.len() > history_width {
        &app.cpu_history[app.cpu_history.len() - history_width..]
    } else {
        &app.cpu_history[..]
    };
    for &val in history_slice {
        let idx = ((val / 100.0) * 8.0).round() as usize;
        let idx = idx.clamp(0, 8);
        history_str.push_str(chart_chars[idx]);
    }

    let load_str = format!("Load: {}", history_str);
    let load_padding = box_width.saturating_sub(load_str.chars().count() + 6);
    stats_lines.push(Line::from(vec![
        Span::styled("  │ ", Style::default().fg(accent)),
        Span::styled("Load: ", Style::default().fg(DIM)),
        Span::styled(history_str, Style::default().fg(Color::Yellow)),
        Span::styled(" ".repeat(load_padding), Style::default()),
        Span::styled(" │", Style::default().fg(accent)),
    ]));

    stats_lines.push(Line::from(Span::styled("  └─────────────────────────────────────┘", Style::default().fg(accent))));

    let stats = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .title(Span::styled(" system status ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        );
    f.render_widget(stats, right_chunks[0]);

    render_matrix_rain(f, right_chunks[1], app);
    } // end of normal/wide home layout
}

fn render_shell_simulator(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let mut lines = vec![
        Line::from(vec![
            Span::styled("arnav@host:~$ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("system_diagnostic", Style::default().fg(FG)),
        ]),
        Line::from(Span::styled("Diagnostic module loaded.", Style::default().fg(DIM))),
        Line::from(""),
    ];
    
    let (cmd_full, _) = &app.shell_cmds[app.shell_cmd_idx];
    let typed = if app.shell_char_idx <= cmd_full.len() {
        &cmd_full[0..app.shell_char_idx]
    } else {
        cmd_full
    };
    
    // Draw past output lines
    for line in &app.shell_output {
        lines.push(Line::from(Span::raw(line)));
    }
    
    // Draw the current active prompt
    if app.shell_state == ShellState::Typing {
        let cursor_char = if app.tick_count % 10 < 5 { "█" } else { " " };
        lines.push(Line::from(vec![
            Span::styled("arnav@host:~$ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(typed.to_string(), Style::default().fg(FG)),
            Span::styled(cursor_char, Style::default().fg(accent)),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::styled("arnav@host:~$ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled((*cmd_full).to_string(), Style::default().fg(FG)),
        ]));
    }
    
    // Scroll to show latest lines to prevent overflow
    let max_lines = area.height.saturating_sub(2) as usize;
    let start_idx = lines.len().saturating_sub(max_lines);
    let visible_lines = lines[start_idx..].to_vec();
    
    let shell = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .title(Span::styled(" active shell ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        );
    f.render_widget(shell, area);
}

fn render_matrix_rain(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled(" data stream ", Style::default().fg(DIM)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let cols = inner.width as usize;
    let rows = inner.height as usize;
    if cols == 0 || rows == 0 { return; }

    // Build a 2D grid of (char, brightness) for each cell
    let mut grid: Vec<Vec<(char, u8)>> = vec![vec![(' ', 0); cols]; rows];

    let num_cols = cols.min(app.matrix_cols.len());
    for c in 0..num_cols {
        let col = &app.matrix_cols[c];
        let head = col.head_y;
        let trail = col.trail_len as i32;

        for t in 0..=trail {
            let y = head - t;
            if y < 0 || y >= rows as i32 { continue; }
            let char_idx = (t as usize) % col.chars.len();
            let ch = col.chars[char_idx];

            // Brightness: head is brightest, fades along the trail
            let brightness = if t == 0 {
                255u8
            } else {
                let ratio = 1.0 - (t as f64 / trail as f64);
                (ratio * 180.0).max(30.0) as u8
            };
            grid[y as usize][c] = (ch, brightness);
        }
    }

    // Render grid into Lines
    let mut lines: Vec<Line> = Vec::with_capacity(rows);
    for row in 0..rows {
        let spans: Vec<Span> = grid[row].iter().map(|&(ch, brightness)| {
            if brightness == 0 {
                Span::styled(" ", Style::default())
            } else if brightness == 255 {
                // Head character: bright white
                Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Rgb(220, 255, 220)).add_modifier(Modifier::BOLD),
                )
            } else {
                // Trail: green with fading intensity
                let g = brightness;
                Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Rgb(0, g, 0)),
                )
            }
        }).collect();
        lines.push(Line::from(spans));
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

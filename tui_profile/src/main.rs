#[cfg(not(target_arch = "wasm32"))]
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(not(target_arch = "wasm32"))]
use ratatui::backend::CrosstermBackend;

#[cfg(target_arch = "wasm32")]
use ratzilla::{
    event::KeyCode,
    DomBackend,
};

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use ratatui::{
        backend::Backend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
        Frame, Terminal,
    };
    pub type Instant = std::time::Instant;
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use ratzilla::ratatui::{
        backend::Backend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
        Frame, Terminal,
    };
    pub type Instant = web_time::Instant;
}

use platform::*;
use std::error::Error;

// ─── Palette ────────────────────────────────────────────────────────────────
const ACCENT: Color = Color::Cyan;
const DIM: Color = Color::DarkGray;
const FG: Color = Color::White;
const BG: Color = Color::Black;

// ─── FPS Tracker ────────────────────────────────────────────────────────────
struct FpsTracker {
    last_frame: Instant,
    fps: f64,
    frame_count: u32,
    last_fps_update: Instant,
}

impl FpsTracker {
    fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            fps: 0.0,
            frame_count: 0,
            last_fps_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.frame_count += 1;
        let elapsed = now.duration_since(self.last_fps_update).as_secs_f64();
        if elapsed >= 0.5 {
            self.fps = self.frame_count as f64 / elapsed;
            self.frame_count = 0;
            self.last_fps_update = now;
        }
        self.last_frame = now;
    }
}

// ─── App State ──────────────────────────────────────────────────────────────
// ─── App State ──────────────────────────────────────────────────────────────
struct App {
    pub tab_index: usize,
    pub tab_titles: Vec<&'static str>,
    pub fps_tracker: FpsTracker,
    pub tick_count: u64,
    pub cpu_cores: Vec<f64>,
    pub cpu_history: Vec<f64>,
}

impl App {
    fn new() -> App {
        App {
            tab_index: 0,
            tab_titles: vec!["  Home  ", "  Projects  ", "  Skills  ", "  Contact  "],
            fps_tracker: FpsTracker::new(),
            tick_count: 0,
            cpu_cores: vec![0.0; 8],
            cpu_history: Vec::new(),
        }
    }
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }
    pub fn prev_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }
    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // 1. Update simulated CPU cores loads using sine waves and pseudo-random noise
        let t = self.tick_count as f64 * 0.04;
        self.cpu_cores.clear();
        for i in 0..8 {
            let base = 35.0 + 25.0 * (t + i as f64 * 0.8).sin();
            let noise = ((self.tick_count + i as u64) % 13) as f64;
            let val = (base + noise).clamp(5.0, 99.0);
            self.cpu_cores.push(val);
        }
        
        // 2. Update rolling CPU history (graph)
        if self.tick_count % 3 == 0 {
            let avg_cpu: f64 = self.cpu_cores.iter().sum::<f64>() / 8.0;
            self.cpu_history.push(avg_cpu);
            if self.cpu_history.len() > 30 {
                self.cpu_history.remove(0);
            }
        }
    }
    pub fn get_accent_color(&self) -> Color {
        let t = self.tick_count as f64 * 0.02;
        // Smoothly cycle through neon green/teal/blue range
        let r = 0;
        let g = ((t.sin() * 40.0) + 215.0) as u8; // 175 to 255
        let b = (((t + 3.14).sin() * 40.0) + 215.0) as u8; // 175 to 255
        Color::Rgb(r, g, b)
    }
}

// ─── Entry Point ────────────────────────────────────────────────────────────
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    use std::time::Duration;
    loop {
        app.fps_tracker.update();
        app.tick();
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => app.next_tab(),
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::BackTab => app.prev_tab(),
                    KeyCode::Char('1') => app.tab_index = 0,
                    KeyCode::Char('2') => app.tab_index = 1,
                    KeyCode::Char('3') => app.tab_index = 2,
                    KeyCode::Char('4') => app.tab_index = 3,
                    _ => {}
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use ratzilla::WebRenderer;

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), Box<dyn Error>> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let app = Rc::new(RefCell::new(App::new()));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let app = app.clone();
        move |key_event| {
            let mut app = app.borrow_mut();
            match key_event.code {
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => app.next_tab(),
                KeyCode::Left | KeyCode::Char('h') => app.prev_tab(),
                KeyCode::Char('1') => app.tab_index = 0,
                KeyCode::Char('2') => app.tab_index = 1,
                KeyCode::Char('3') => app.tab_index = 2,
                KeyCode::Char('4') => app.tab_index = 3,
                _ => {}
            }
        }
    });

    terminal.draw_web(move |f| {
        let mut app = app.borrow_mut();
        app.fps_tracker.update();
        app.tick();
        ui(f, &app);
    });

    Ok(())
}

// ─── Root UI ────────────────────────────────────────────────────────────────
fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let accent = app.get_accent_color();

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

    render_header(f, chunks[0], accent);
    render_tabs(f, chunks[1], app, accent);
    render_body(f, chunks[2], app, accent);
    render_footer(f, chunks[3], app, accent);
}

// ─── Header ─────────────────────────────────────────────────────────────────
fn render_header(f: &mut Frame, area: Rect, accent: Color) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("▸ ", Style::default().fg(accent)),
            Span::styled(
                "ARNAV SHARMA",
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
fn render_tabs(f: &mut Frame, area: Rect, app: &App, accent: Color) {
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

// ─── Body dispatcher ────────────────────────────────────────────────────────
fn render_body(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    match app.tab_index {
        0 => render_home(f, area, app, accent),
        1 => render_projects(f, area, accent),
        2 => render_skills(f, area, accent),
        3 => render_contact(f, area, accent),
        _ => {}
    }
}

fn make_progress_bar(val: f64, width: usize) -> String {
    let filled = ((width * val as usize) / 100).min(width);
    let empty = width.saturating_sub(filled);
    "█".repeat(filled) + &"░".repeat(empty)
}

// ─── Tab: Home ───────────────────────────────────────────────────────────────
fn render_home(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    // Left: bio
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
    f.render_widget(bio, cols[0]);

    // Right: dynamic stats / simulated CPU monitor
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
        Line::from(Span::styled("  ┌─ CPU Monitor ───────────────────────┐", Style::default().fg(accent))),
    ];

    let box_width = cols[1].width as usize;
    let is_wide = box_width >= 40;

    // Choose progress bar width dynamically to fit screen size
    let bar_width = if is_wide {
        ((box_width - 24) / 2).clamp(3, 10)
    } else {
        (box_width - 15).clamp(3, 15)
    };

    if is_wide {
        // Render 2 columns: 4 rows of 2 cores
        for i in (0..8).step_by(2) {
            let cpu1 = app.cpu_cores.get(i).copied().unwrap_or(0.0);
            let cpu2 = app.cpu_cores.get(i + 1).copied().unwrap_or(0.0);
            let bar1 = make_progress_bar(cpu1, bar_width);
            let bar2 = make_progress_bar(cpu2, bar_width);
            
            let left_str = format!("C{} [", i + 1);
            let right_str = format!("C{} [", i + 2);
            
            // Render nice color segments
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
        // Render 1 column: 8 rows (or fewer if height is restricted)
        let max_rows = (cols[1].height as usize).saturating_sub(8).min(8).max(4);
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
    stats_lines.push(Line::from(""));
    stats_lines.push(Line::from(Span::styled(
        "   ↹ tab / ← → to navigate  │  q to quit",
        Style::default().fg(DIM),
    )));

    let stats = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .title(Span::styled(" sys-info ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        );
    f.render_widget(stats, cols[1]);
}

// ─── Tab: Projects ───────────────────────────────────────────────────────────
fn render_projects(f: &mut Frame, area: Rect, accent: Color) {
    let projects = vec![
        (
            "[Bare-Metal OS / Kernel]",
            "C / Rust / Assembly",
            "Implementing custom OS components, from demand paging to AI-integrated error detection.",
        ),
        (
            "[High-Concurrency Engine]",
            "Rust / C++",
            "Building memory-safe, high-concurrency engines with lock-free queues and parallel architecture.",
        ),
        (
            "[Real-Time CV Pipeline]",
            "C++ / PyTorch / CUDA",
            "Optimizing perception pipelines with TensorRT and OpenCV for unstructured environments.",
        ),
        (
            "[Distributed Systems Engine]",
            "Rust",
            "High-performance distributed systems architecture designed for resilience and concurrency.",
        ),
        (
            "[Algorithmic Solver]",
            "C++",
            "Custom implementations for complex algorithmic problems optimized for time and space.",
        ),
    ];

    let rows: Vec<ListItem> = projects
        .iter()
        .enumerate()
        .flat_map(|(i, (name, lang, desc))| {
            vec![
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("  {:02}  ", i + 1),
                        Style::default().fg(DIM),
                    ),
                    Span::styled(*name, Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                    Span::styled("  ·  ", Style::default().fg(DIM)),
                    Span::styled(*lang, Style::default().fg(Color::Yellow)),
                ])),
                ListItem::new(Line::from(Span::styled(
                    format!("       {}", desc),
                    Style::default().fg(FG),
                ))),
                ListItem::new(Line::from("")),
            ]
        })
        .collect();

    let list = List::new(rows).block(
        Block::default()
            .title(Span::styled(" projects ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    );
    f.render_widget(list, area);
}

// ─── Tab: Skills ─────────────────────────────────────────────────────────────
fn render_skills(f: &mut Frame, area: Rect, accent: Color) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

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

    render_skill_col(f, cols[0], " systems ", &systems_skills, accent);
    render_skill_col(f, cols[1], " ml & ai ", &ml_skills, accent);
    render_skill_col(f, cols[2], " tools ", &tools_skills, accent);
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

// ─── Tab: Contact ────────────────────────────────────────────────────────────
fn render_contact(f: &mut Frame, area: Rect, accent: Color) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

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

    let contact = Paragraph::new(contact_lines)
        .block(
            Block::default()
                .title(Span::styled(" contact ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(contact, cols[0]);

    // availability block
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
            "  ╚────────────────────╝",
            Style::default().fg(accent),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Location:",
            Style::default().fg(DIM),
        )),
        Line::from(Span::styled(
            "  Solan, HP, India",
            Style::default().fg(FG),
        )),
    ];

    let avail = Paragraph::new(avail_lines)
        .block(
            Block::default()
                .title(Span::styled(" meta ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        );
    f.render_widget(avail, cols[1]);
}

// ─── Footer ──────────────────────────────────────────────────────────────────
fn render_footer(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ← → ", Style::default().fg(accent)),
        Span::styled("navigate", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("1-4 ", Style::default().fg(accent)),
        Span::styled("jump", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("q ", Style::default().fg(accent)),
        Span::styled("quit", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled(format!("FPS: {:.1}", app.fps_tracker.fps), Style::default().fg(Color::Yellow)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, area);
}
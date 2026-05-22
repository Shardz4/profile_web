use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use std::{error::Error, io};

// ─── Palette ────────────────────────────────────────────────────────────────
const ACCENT: Color = Color::Cyan;
const DIM: Color = Color::DarkGray;
const FG: Color = Color::White;
const BG: Color = Color::Black;

// ─── App State ──────────────────────────────────────────────────────────────
struct App {
    pub tab_index: usize,
    pub tab_titles: Vec<&'static str>,
}

impl App {
    fn new() -> App {
        App {
            tab_index: 0,
            tab_titles: vec!["  Home  ", "  Projects  ", "  Skills  ", "  Contact  "],
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
}

// ─── Entry Point ────────────────────────────────────────────────────────────
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
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

// ─── Event Loop ─────────────────────────────────────────────────────────────
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

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

// ─── Root UI ────────────────────────────────────────────────────────────────
fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    // Outer chrome
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(ACCENT))
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

    render_header(f, chunks[0]);
    render_tabs(f, chunks[1], app);
    render_body(f, chunks[2], app);
    render_footer(f, chunks[3]);
}

// ─── Header ─────────────────────────────────────────────────────────────────
fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("▸ ", Style::default().fg(ACCENT)),
            Span::styled(
                "YOUR NAME",
                Style::default()
                    .fg(FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  ·  Systems Engineer  |  ML / Deep Learning  |  AI Researcher",
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
fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
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
                .fg(ACCENT)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )
        .divider(Span::styled(" │ ", Style::default().fg(DIM)));

    f.render_widget(tabs, area);
}

// ─── Body dispatcher ────────────────────────────────────────────────────────
fn render_body(f: &mut Frame, area: Rect, app: &App) {
    match app.tab_index {
        0 => render_home(f, area),
        1 => render_projects(f, area),
        2 => render_skills(f, area),
        3 => render_contact(f, area),
        _ => {}
    }
}

// ─── Tab: Home ───────────────────────────────────────────────────────────────
fn render_home(f: &mut Frame, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Left: bio
    let bio_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Hello, world. ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("I'm [Your Name].", Style::default().fg(FG)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  A systems tinkerer and ML practitioner who lives at the",
            Style::default().fg(FG),
        )),
        Line::from(Span::styled(
            "  intersection of low-level hardware and high-level intelligence.",
            Style::default().fg(FG),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  I build things that run fast, learn well, and break gracefully.",
            Style::default().fg(DIM),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Currently: ", Style::default().fg(ACCENT)),
            Span::styled("[Role / Position / School]", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  Based in:  ", Style::default().fg(ACCENT)),
            Span::styled("[City, Country]", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  Focus:     ", Style::default().fg(ACCENT)),
            Span::styled(
                "Systems programming · Deep Learning · AI inference",
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

    // Right: quick stats / ascii art
    let ascii = vec![
        Line::from(""),
        Line::from(Span::styled("   ╔══════════════╗", Style::default().fg(ACCENT))),
        Line::from(Span::styled("   ║  > whoami    ║", Style::default().fg(FG))),
        Line::from(Span::styled("   ╚══════════════╝", Style::default().fg(ACCENT))),
        Line::from(""),
        Line::from(vec![
            Span::styled("   OS   ", Style::default().fg(DIM)),
            Span::styled("Linux / [Your OS]", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("   Lang ", Style::default().fg(DIM)),
            Span::styled("Rust · Python · C/C++", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("   ML   ", Style::default().fg(DIM)),
            Span::styled("PyTorch · JAX · ONNX", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("   Sys  ", Style::default().fg(DIM)),
            Span::styled("RTOS · CUDA · eBPF", Style::default().fg(FG)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "   ↹ tab / ← → to navigate",
            Style::default().fg(DIM),
        )),
        Line::from(Span::styled(
            "   q to quit",
            Style::default().fg(DIM),
        )),
    ];

    let stats = Paragraph::new(ascii)
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
fn render_projects(f: &mut Frame, area: Rect) {
    let projects = vec![
        (
            "[Project Alpha]",
            "Rust",
            "A high-throughput systems daemon with lock-free queues and eBPF observability hooks.",
        ),
        (
            "[Neural Compiler]",
            "C++ / LLVM",
            "Custom IR lowering pass for DNN operator fusion targeting edge inference hardware.",
        ),
        (
            "[Bare-Metal RTOS]",
            "C / ARMv7-M",
            "Preemptive scheduler with priority inheritance mutexes; boots in < 2 ms on Cortex-M4.",
        ),
        (
            "[LLM Inference Engine]",
            "Python / CUDA",
            "Quantized transformer inference pipeline with custom CUDA kernels; 3× faster than baseline.",
        ),
        (
            "[Distributed RL Agent]",
            "Python / Ray",
            "Multi-agent reinforcement learning framework for robotics sim-to-real transfer experiments.",
        ),
        (
            "[Add Your Project]",
            "Your Stack",
            "Describe what it does, the problem it solves, and what makes it interesting.",
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
                    Span::styled(*name, Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
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
fn render_skills(f: &mut Frame, area: Rect) {
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
        ("Linux Internals", 85),
        ("RTOS / Bare-Metal", 80),
        ("eBPF / Perf", 75),
        ("CUDA / GPU Arch", 70),
        ("Assembly (ARM/x86)", 65),
    ];

    let ml_skills = vec![
        ("Python", 95),
        ("PyTorch", 90),
        ("Deep Learning Theory", 88),
        ("JAX / Flax", 78),
        ("ONNX / TensorRT", 75),
        ("Reinforcement Learning", 72),
        ("Model Quantization", 70),
    ];

    let tools_skills = vec![
        ("Git / GitHub", 95),
        ("Docker / Containers", 85),
        ("LLVM / Compilers", 70),
        ("Distributed Systems", 75),
        ("Data Pipelines", 72),
        ("CI/CD", 80),
        ("Vim / Neovim", 90),
    ];

    render_skill_col(f, cols[0], " systems ", &systems_skills);
    render_skill_col(f, cols[1], " ml & ai ", &ml_skills);
    render_skill_col(f, cols[2], " tools ", &tools_skills);
}

fn render_skill_col(f: &mut Frame, area: Rect, title: &str, skills: &[(&str, u8)]) {
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
                    Span::styled(bar, Style::default().fg(ACCENT)),
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
fn render_contact(f: &mut Frame, area: Rect) {
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
            Span::styled("  ✉  Email   ", Style::default().fg(ACCENT)),
            Span::styled("you@example.com", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ⌘  GitHub  ", Style::default().fg(ACCENT)),
            Span::styled("github.com/yourusername", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ∟  LinkedIn", Style::default().fg(ACCENT)),
            Span::styled("linkedin.com/in/yourprofile", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ✦  Twitter ", Style::default().fg(ACCENT)),
            Span::styled("@yourhandle", Style::default().fg(FG)),
        ]),
        Line::from(vec![
            Span::styled("  ◈  Website ", Style::default().fg(ACCENT)),
            Span::styled("yoursite.dev", Style::default().fg(FG)),
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

    // GPG / availability block
    let avail_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ╔─ availability ──────╗",
            Style::default().fg(ACCENT),
        )),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(ACCENT)),
            Span::styled("Status   ", Style::default().fg(DIM)),
            Span::styled("[Open / Busy]    ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(ACCENT)),
        ]),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(ACCENT)),
            Span::styled("Timezone ", Style::default().fg(DIM)),
            Span::styled("[UTC+X]          ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(ACCENT)),
        ]),
        Line::from(vec![
            Span::styled("  ║ ", Style::default().fg(ACCENT)),
            Span::styled("Response ", Style::default().fg(DIM)),
            Span::styled("~24 hours        ", Style::default().fg(FG)),
            Span::styled("║", Style::default().fg(ACCENT)),
        ]),
        Line::from(Span::styled(
            "  ╚────────────────────╝",
            Style::default().fg(ACCENT),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  PGP / GPG key:",
            Style::default().fg(DIM),
        )),
        Line::from(Span::styled(
            "  [0xYOUR_KEY_FINGERPRINT]",
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
fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ← → ", Style::default().fg(ACCENT)),
        Span::styled("navigate tabs", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("1-4 ", Style::default().fg(ACCENT)),
        Span::styled("jump to tab", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("q ", Style::default().fg(ACCENT)),
        Span::styled("quit", Style::default().fg(DIM)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, area);
}
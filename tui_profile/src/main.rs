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
    CanvasBackend,
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
// ACCENT color is determined dynamically in App::get_accent_color()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellState {
    Typing,
    Running,
    Finished,
}

// ─── App State ──────────────────────────────────────────────────────────────
struct App {
    pub tab_index: usize,
    pub tab_titles: Vec<&'static str>,
    pub fps_tracker: FpsTracker,
    pub tick_count: u64,
    pub cpu_cores: Vec<f64>,
    pub cpu_history: Vec<f64>,
    pub gpu_load: f64,
    pub vram_usage: f64,
    pub page_table: Vec<char>,
    pub shell_cmds: Vec<(&'static str, Vec<&'static str>)>,
    pub shell_cmd_idx: usize,
    pub shell_char_idx: usize,
    pub shell_state: ShellState,
    pub shell_output: Vec<String>,
    pub shell_wait_ticks: u32,
}

impl App {
    fn new() -> App {
        let mut page_table = vec!['.'; 64];
        for i in 0..64 {
            if i % 7 == 0 {
                page_table[i] = 'A';
            } else if i % 13 == 0 {
                page_table[i] = 'D';
            } else if i % 19 == 0 {
                page_table[i] = 'R';
            }
        }

        let shell_cmds = vec![
            (
                "git log -n 3 --oneline",
                vec![
                    "a5f2b8c (HEAD -> main) feat: implement custom lock-free memory allocator",
                    "3d92e10 refactor: optimize demand paging and TLB shootdown logic",
                    "8b201fa fix: resolve GPU TensorRT thread synchronization race condition",
                ],
            ),
            (
                "cargo test --profile release",
                vec![
                    "   Compiling core-allocator v0.1.0",
                    "   Compiling tui-profile v0.1.0",
                    "    Finished release [optimized] target(s) in 1.42s",
                    "     Running unittests src/main.rs",
                    "test platform::tests::test_page_allocation ... ok",
                    "test platform::tests::test_lock_free_queue ... ok",
                    "test result: ok. 2 passed; 0 failed; 0 ignored",
                ],
            ),
            (
                "python3 train_detector.py --epochs 10",
                vec![
                    "[INFO] CUDA device detected: NVIDIA RTX 4090",
                    "[INFO] Initializing PyTorch training pipeline...",
                    "Epoch 1/10 - loss: 0.432 - acc: 89.2%",
                    "Epoch 5/10 - loss: 0.125 - acc: 97.8%",
                    "Epoch 10/10 - loss: 0.048 - acc: 99.4%",
                    "Model weights exported successfully.",
                ],
            ),
            (
                "make build_kernel",
                vec![
                    "nasm -f elf64 src/boot/boot.asm -o build/boot.o",
                    "x86_64-elf-gcc -c src/kernel/main.c -o build/main.o",
                    "x86_64-elf-ld -n -T targets/linker.ld build/boot.o build/main.o -o build/kernel.bin",
                    "Kernel compilation complete: build/kernel.bin",
                ],
            ),
        ];

        App {
            tab_index: 0,
            tab_titles: vec!["  Home  ", "  Projects  ", "  Skills  ", "  Contact  "],
            fps_tracker: FpsTracker::new(),
            tick_count: 0,
            cpu_cores: vec![0.0; 8],
            cpu_history: Vec::new(),
            gpu_load: 0.0,
            vram_usage: 4.12,
            page_table,
            shell_cmds,
            shell_cmd_idx: 0,
            shell_char_idx: 0,
            shell_state: ShellState::Typing,
            shell_output: Vec::new(),
            shell_wait_ticks: 0,
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

        // 3. Update simulated GPU load & VRAM
        self.gpu_load = (45.0 + 35.0 * (t * 0.7).cos() + ((self.tick_count % 11) as f64)).clamp(0.0, 100.0);
        let base_vram = if self.shell_state == ShellState::Running { 6.2 } else { 4.1 };
        self.vram_usage = (base_vram + (t.sin() * 0.15)).clamp(1.0, 8.0);

        // 4. Update simulated page table memory allocations
        if self.tick_count % 15 == 0 {
            let mut rng_seed = self.tick_count;
            for _ in 0..3 {
                let idx = (rng_seed % 64) as usize;
                rng_seed = rng_seed.wrapping_add(17);
                let states = ['.', 'A', 'D', 'R'];
                let state_idx = (rng_seed % 4) as usize;
                self.page_table[idx] = states[state_idx];
            }
        }

        // 5. Shell simulator update logic
        if self.shell_wait_ticks > 0 {
            self.shell_wait_ticks -= 1;
        } else {
            match self.shell_state {
                ShellState::Typing => {
                    let (cmd, _) = &self.shell_cmds[self.shell_cmd_idx];
                    if self.shell_char_idx < cmd.len() {
                        self.shell_char_idx += 1;
                        self.shell_wait_ticks = (self.tick_count % 3) as u32; 
                    } else {
                        self.shell_state = ShellState::Running;
                        self.shell_wait_ticks = 15; // Wait before running
                    }
                }
                ShellState::Running => {
                    let (_, outputs) = &self.shell_cmds[self.shell_cmd_idx];
                    let current_lines = self.shell_output.len();
                    if current_lines < outputs.len() {
                        self.shell_output.push(outputs[current_lines].to_string());
                        self.shell_wait_ticks = 8;
                    } else {
                        self.shell_state = ShellState::Finished;
                        self.shell_wait_ticks = 40;
                    }
                }
                ShellState::Finished => {
                    self.shell_cmd_idx = (self.shell_cmd_idx + 1) % self.shell_cmds.len();
                    self.shell_char_idx = 0;
                    self.shell_output.clear();
                    self.shell_state = ShellState::Typing;
                    self.shell_wait_ticks = 20;
                }
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
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>>
where
    B::Error: 'static,
{
    use std::time::Duration;
    loop {
        app.fps_tracker.update();
        app.tick();
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
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
use wasm_bindgen::{prelude::Closure, JsCast, prelude::wasm_bindgen};
#[cfg(target_arch = "wasm32")]
use ratzilla::WebRenderer;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
    export function request_fullscreen() {
        const el = document.documentElement;
        if (el.requestFullscreen) {
            el.requestFullscreen().catch(err => {
                console.warn("Fullscreen request rejected:", err);
            });
        }
    }
    export function exit_fullscreen() {
        if (document.exitFullscreen) {
            document.exitFullscreen().catch(err => {
                console.warn("Exit fullscreen failed:", err);
            });
        }
    }
    export function setup_fullscreen_click() {
        document.addEventListener('click', () => {
            request_fullscreen();
        }, { once: false });
    }
"#)]
extern "C" {
    fn request_fullscreen();
    fn exit_fullscreen();
    fn setup_fullscreen_click();
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), Box<dyn Error>> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let app = Rc::new(RefCell::new(App::new()));

    let window = web_sys::window().unwrap();
    let init_closure = Closure::wrap(Box::new({
        let app = app.clone();
        move || {
            if let Err(e) = init_app(app.clone()) {
                web_sys::console::error_1(&format!("Failed to initialize app: {:?}", e).into());
            }
        }
    }) as Box<dyn FnMut()>);

    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        init_closure.as_ref().unchecked_ref(),
        100, // 100ms delay to ensure browser layout computes
    ).unwrap();

    init_closure.forget();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn init_app(app: Rc<RefCell<App>>) -> Result<(), Box<dyn Error>> {
    setup_fullscreen_click();
    let backend = CanvasBackend::new_with_options(
        ratzilla::backend::canvas::CanvasBackendOptions::new()
            .grid_id("terminal-container")
    )?;
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
                KeyCode::Char('f') | KeyCode::Char('F') => {
                    request_fullscreen();
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    exit_fullscreen();
                }
                _ => {}
            }
        }
    });

    let terminal = Rc::new(RefCell::new(terminal));
    let last_time = Rc::new(RefCell::new(Instant::now()));
    let accumulator = Rc::new(RefCell::new(0.0));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let app_inner = app.clone();
    let terminal_inner = terminal.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new({
        let last_time = last_time.clone();
        let accumulator = accumulator.clone();
        let f = f.clone();
        move || {
            let now = Instant::now();
            let mut elapsed = now.duration_since(*last_time.borrow()).as_secs_f64();
            *last_time.borrow_mut() = now;

            if elapsed > 0.25 {
                elapsed = 0.25;
            }

            *accumulator.borrow_mut() += elapsed;

            let timestep = 1.0 / 60.0;
            let mut app = app_inner.borrow_mut();
            app.fps_tracker.update();

            while *accumulator.borrow_mut() >= timestep {
                app.tick();
                *accumulator.borrow_mut() -= timestep;
            }

            let mut term = terminal_inner.borrow_mut();
            term.draw(|frame| {
                ui(frame, &app);
            }).unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
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
// ─── Tab: Home ───────────────────────────────────────────────────────────────
fn render_home(f: &mut Frame, area: Rect, app: &App, accent: Color) {
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

    // Right: dynamic stats / simulated CPU monitor (top) & Page Table Allocation Map (bottom)
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

    render_page_table(f, right_chunks[1], app, accent);
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

fn render_page_table(f: &mut Frame, area: Rect, app: &App, _accent: Color) {
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Legend: ", Style::default().fg(DIM)),
            Span::styled(". ", Style::default().fg(Color::DarkGray)), Span::styled("Free ", Style::default().fg(DIM)),
            Span::styled("■ ", Style::default().fg(Color::Green)), Span::styled("Alloc ", Style::default().fg(DIM)),
            Span::styled("▩ ", Style::default().fg(Color::Yellow)), Span::styled("Dirty ", Style::default().fg(DIM)),
            Span::styled("▨ ", Style::default().fg(Color::LightRed)), Span::styled("Locked ", Style::default().fg(DIM)),
        ]),
        Line::from(""),
    ];
    
    // Render the 8x8 memory matrix
    for row in 0..8 {
        let mut row_spans = vec![
            Span::styled("  PAGE_TABLE:  ", Style::default().fg(DIM)),
        ];
        for col in 0..8 {
            let idx = row * 8 + col;
            let ch = app.page_table.get(idx).copied().unwrap_or('.');
            let (symbol, color) = match ch {
                'A' => ("■ ", Color::Green),
                'D' => ("▩ ", Color::Yellow),
                'R' => ("▨ ", Color::LightRed),
                _   => (". ", Color::DarkGray),
            };
            row_spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
        lines.push(Line::from(row_spans));
    }
    
    lines.push(Line::from(""));
    
    let active_pages = app.page_table.iter().filter(|&&c| c != '.').count();
    let usage_pct = (active_pages as f64 / 64.0) * 100.0;
    lines.push(Line::from(vec![
        Span::styled("  Memory Used: ", Style::default().fg(DIM)),
        Span::styled(format!("{:.1}% ", usage_pct), Style::default().fg(Color::LightBlue)),
        Span::styled(format!("({}/64 pages allocated)", active_pages), Style::default().fg(DIM)),
    ]));
    
    let memory_block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(Span::styled(" page allocation map ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)),
        );
    f.render_widget(memory_block, area);
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
        Span::styled("f ", Style::default().fg(accent)),
        Span::styled("fullscreen", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("q ", Style::default().fg(accent)),
        #[cfg(target_arch = "wasm32")]
        Span::styled("exit fullscreen", Style::default().fg(DIM)),
        #[cfg(not(target_arch = "wasm32"))]
        Span::styled("quit", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled(format!("FPS: {:.1}", app.fps_tracker.fps), Style::default().fg(Color::Yellow)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, area);
}
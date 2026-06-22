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
        widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap, BarChart, Sparkline, Chart, Axis, Dataset, GraphType, Widget},
        symbols::Marker,
        buffer::Buffer,
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
        widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap, BarChart, Sparkline, Chart, Axis, Dataset, GraphType, Widget},
        symbols::Marker,
        buffer::Buffer,
        Frame, Terminal,
    };
    pub type Instant = web_time::Instant;
}

use platform::*;
use std::error::Error;

// ─── Modules ────────────────────────────────────────────────────────────────
pub mod theme;
pub mod effects;
pub mod events;
pub mod widgets;
pub mod ui;

use effects::{FpsTracker, Particle, MatrixCol};
use events::{ShellState, Achievement, KONAMI_SEQUENCE};
use theme::THEME_COUNT;
use ui::projects::{ProjectFocus, PROJECT_COUNT, PROJECTS};

// ─── Palette (defaults / fallbacks) ─────────────────────────────────────────
pub const DIM: Color = Color::DarkGray;
pub const FG: Color = Color::White;
pub const BG: Color = Color::Black;

// ─── Responsive Breakpoints ─────────────────────────────────────────────────
pub const NARROW: u16 = 80;
#[allow(dead_code)]
pub const MEDIUM: u16 = 120;

// ─── App State ──────────────────────────────────────────────────────────────
pub struct App {
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
    pub boot_mode: bool,
    pub sparkline_data: Vec<u64>,
    pub konami_buffer: Vec<char>,
    pub konami_active: bool,
    pub konami_ticks: u64,
    pub matrix_cols: Vec<MatrixCol>,
    pub tabs_visited: [bool; 5],
    pub achievements: Vec<Achievement>,
    pub achievement_toast: Option<(String, String, u64)>, // (name, desc, show_until_tick)
    pub particles: Vec<Particle>,
    pub current_theme: usize,
    pub selected_skill_idx: usize,
    // ─── Projects state ─────────────────────────────────────────────────────
    pub project_selected_idx: usize,
    pub project_detail_view: bool,
    pub project_focus: ProjectFocus,
    pub project_detail_scroll: u16,
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
            tab_titles: vec!["  Home  ", "  Projects  ", "  Skills  ", "  Contact  ", "  Trophy Cabinet  "],
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
            boot_mode: true,
            sparkline_data: vec![0; 100],
            konami_buffer: Vec::new(),
            konami_active: false,
            konami_ticks: 0,
            matrix_cols: (0..120).map(|i| MatrixCol::new(i * 37 + 11, 30)).collect(),
            tabs_visited: [true, false, false, false, false], // Home starts visited
            achievements: vec![
                Achievement {
                    name: "Explorer",
                    description: "Visit all 5 tabs",
                    icon: "★",
                    unlocked: false,
                },
                Achievement {
                    name: "Retro Gamer",
                    description: "Activate the Konami Code",
                    icon: "🎮",
                    unlocked: false,
                },
                Achievement {
                    name: "Patient",
                    description: "Stay for 2+ minutes",
                    icon: "⏱",
                    unlocked: false,
                },
                Achievement {
                    name: "Boot Master",
                    description: "Complete the boot sequence",
                    icon: "⚡",
                    unlocked: false,
                },
                Achievement {
                    name: "First Contact",
                    description: "Visit the Contact tab",
                    icon: "✉",
                    unlocked: false,
                },
            ],
            achievement_toast: None,
            particles: (0..40).map(|i| Particle::new(i * 47 + 13, 160, 50)).collect(),
            current_theme: 0,
            selected_skill_idx: 0,
            project_selected_idx: 0,
            project_detail_view: false,
            project_focus: ProjectFocus::KnowMore,
            project_detail_scroll: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
        self.mark_tab_visited();
    }

    pub fn prev_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
        self.mark_tab_visited();
    }

    pub fn move_skill_selection(&mut self, dir: usize) {
        if self.tab_index != 2 {
            return;
        }
        let current = &ui::skills::SKILL_NODES[self.selected_skill_idx];
        let target_id = current.nav[dir];
        if let Some(idx) = ui::skills::SKILL_NODES.iter().position(|n| n.id == target_id) {
            self.selected_skill_idx = idx;
        }
    }

    pub fn project_select_up(&mut self) {
        if self.project_selected_idx > 0 {
            self.project_selected_idx -= 1;
        }
    }

    pub fn project_select_down(&mut self) {
        if self.project_selected_idx < PROJECT_COUNT - 1 {
            self.project_selected_idx += 1;
        }
    }

    pub fn toggle_project_focus(&mut self) {
        self.project_focus = match self.project_focus {
            ProjectFocus::KnowMore => ProjectFocus::GitHub,
            ProjectFocus::GitHub => ProjectFocus::KnowMore,
        };
    }

    pub fn get_selected_github_url(&self) -> &'static str {
        PROJECTS[self.project_selected_idx].github_url
    }

    pub fn mark_tab_visited(&mut self) {
        if self.tab_index < 5 {
            self.tabs_visited[self.tab_index] = true;
        }
        // Check "Explorer" achievement: all 5 tabs visited
        if self.tabs_visited.iter().all(|&v| v) {
            self.unlock_achievement(0); // Explorer
        }
        // Check "First Contact" achievement: Contact tab (index 3)
        if self.tab_index == 3 {
            self.unlock_achievement(4); // First Contact
        }
    }

    pub fn unlock_achievement(&mut self, idx: usize) {
        if let Some(ach) = self.achievements.get_mut(idx) {
            if !ach.unlocked {
                ach.unlocked = true;
                self.achievement_toast = Some((
                    ach.name.to_string(),
                    ach.description.to_string(),
                    self.tick_count + 360, // show for ~3 seconds at 120Hz
                ));
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // Update sparkline data
        let t_spark = self.tick_count as f64 * 0.15;
        let spark_val = (50.0 + 35.0 * t_spark.sin() + 10.0 * (t_spark * 2.3).cos()).clamp(0.0, 99.0) as u64;
        self.sparkline_data.push(spark_val);
        if self.sparkline_data.len() > 100 {
            self.sparkline_data.remove(0);
        }

        // 1. Update simulated CPU cores loads
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
                        self.shell_wait_ticks = 15;
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
        if self.konami_active && self.tick_count.saturating_sub(self.konami_ticks) >= 300 {
            self.konami_active = false;
        }

        // 7. Achievement tick checks
        if self.tick_count == 14400 {
            self.unlock_achievement(2); // Patient — 2+ minutes
        }
        if !self.boot_mode && self.tick_count > 10 {
            self.unlock_achievement(3); // Boot Master
        }
        // Expire achievement toast
        if let Some((_, _, until)) = &self.achievement_toast {
            if self.tick_count > *until {
                self.achievement_toast = None;
            }
        }

        // 6. Update matrix rain columns
        for (i, col) in self.matrix_cols.iter_mut().enumerate() {
            if self.tick_count % (col.speed as u64) == 0 {
                col.head_y += 1;
                if col.head_y > 50 + col.trail_len as i32 {
                    *col = MatrixCol::new(self.tick_count.wrapping_mul(7).wrapping_add(i as u64 * 31), 30);
                    col.head_y = -((self.tick_count.wrapping_add(i as u64 * 3) % 5) as i32);
                }
                let matrix_chars: Vec<char> = "ｦｧｨｩｪｫｬｭｮｯｰ0123456789ABCDEF<>{}|/\\*+=-~^&"
                    .chars().collect();
                let rot_idx = (self.tick_count.wrapping_add(i as u64)) as usize % col.chars.len();
                let new_char_idx = (self.tick_count.wrapping_mul(13).wrapping_add(i as u64 * 7)) as usize % matrix_chars.len();
                col.chars[rot_idx] = matrix_chars[new_char_idx];
            }
        }

        // 8. Update particles
        for (i, p) in self.particles.iter_mut().enumerate() {
            p.x += p.vx;
            p.y += p.vy;
            if p.x < 0.0 { p.x += 160.0; }
            if p.x >= 160.0 { p.x -= 160.0; }
            if p.y < 0.0 { p.y += 50.0; }
            if p.y >= 50.0 { p.y -= 50.0; }
            if (self.tick_count + i as u64 * 7) % 90 == 0 {
                p.brightness = ((p.brightness as u16 + 15) % 70 + 25) as u8;
            }
        }
    }

    pub fn get_accent_color(&self) -> Color {
        theme::get_accent_color(self.current_theme, self.tick_count)
    }

    pub fn get_theme_name(&self) -> &'static str {
        theme::get_theme_name(self.current_theme)
    }

    pub fn cycle_theme(&mut self) {
        self.current_theme = (self.current_theme + 1) % THEME_COUNT;
    }

    pub fn register_key(&mut self, c: char) {
        self.konami_buffer.push(c);
        if self.konami_buffer.len() > 10 {
            self.konami_buffer.remove(0);
        }
        if self.konami_buffer.len() == 10 && self.konami_buffer == KONAMI_SEQUENCE {
            self.konami_active = true;
            self.konami_ticks = self.tick_count;
            self.konami_buffer.clear();
            self.unlock_achievement(1); // Retro Gamer
        }
    }
}

// ─── Entry Point (Native) ───────────────────────────────────────────────────
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
        terminal.draw(|f| ui::ui(f, &app))?;

        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                let key_char = match key.code {
                    KeyCode::Up => Some('u'),
                    KeyCode::Down => Some('d'),
                    KeyCode::Left => Some('l'),
                    KeyCode::Right => Some('r'),
                    KeyCode::Char('b') | KeyCode::Char('B') => Some('b'),
                    KeyCode::Char('a') | KeyCode::Char('A') => Some('a'),
                    _ => Some('_'),
                };
                if let Some(c) = key_char {
                    app.register_key(c);
                }

                if app.boot_mode {
                    match key.code {
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            app.boot_mode = false;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                } else if app.project_detail_view && app.tab_index == 1 {
                    // ── Project detail view keybindings ──
                    match key.code {
                        KeyCode::Esc | KeyCode::Backspace => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                        }
                        KeyCode::Enter => {
                            let url = app.get_selected_github_url();
                            open_url_native(url);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.project_detail_scroll = app.project_detail_scroll.saturating_sub(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.project_detail_scroll += 1;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                        KeyCode::Tab => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.next_tab();
                        }
                        KeyCode::BackTab => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.prev_tab();
                        }
                        KeyCode::Char('1') => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.tab_index = 0;
                            app.mark_tab_visited();
                        }
                        KeyCode::Char('2') => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.tab_index = 1;
                            app.mark_tab_visited();
                        }
                        KeyCode::Char('3') => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.tab_index = 2;
                            app.mark_tab_visited();
                        }
                        KeyCode::Char('4') => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.tab_index = 3;
                            app.mark_tab_visited();
                        }
                        KeyCode::Char('5') => {
                            app.project_detail_view = false;
                            app.project_detail_scroll = 0;
                            app.tab_index = 4;
                            app.mark_tab_visited();
                        }
                        _ => {}
                    }
                } else if app.tab_index == 1 {
                    // ── Project list view keybindings ──
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                            app.project_select_up();
                        }
                        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                            app.project_select_down();
                        }
                        KeyCode::Left | KeyCode::Right => {
                            app.toggle_project_focus();
                        }
                        KeyCode::Enter => {
                            match app.project_focus {
                                ProjectFocus::KnowMore => {
                                    app.project_detail_view = true;
                                    app.project_detail_scroll = 0;
                                }
                                ProjectFocus::GitHub => {
                                    let url = app.get_selected_github_url();
                                    open_url_native(url);
                                }
                            }
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                        KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                        KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                        KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                        KeyCode::Char('5') => { app.tab_index = 4; app.mark_tab_visited(); },
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                        KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                        KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                        KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                        KeyCode::Char('5') => { app.tab_index = 4; app.mark_tab_visited(); },
                        KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        
                        // Skills graph navigation (only on Skills tab)
                        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') if app.tab_index == 2 => {
                            app.move_skill_selection(0);
                        }
                        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') if app.tab_index == 2 => {
                            app.move_skill_selection(1);
                        }
                        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') if app.tab_index == 2 => {
                            app.move_skill_selection(2);
                        }
                        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') if app.tab_index == 2 => {
                            app.move_skill_selection(3);
                        }

                        // Normal tab navigation (on other tabs)
                        KeyCode::Right | KeyCode::Char('l') => app.next_tab(),
                        KeyCode::Left | KeyCode::Char('h') => app.prev_tab(),
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_url_native(url: &str) {
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("cmd").args(["/C", "start", url]).spawn(); }
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg(url).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(url).spawn(); }
}

// ─── Entry Point (WASM) ─────────────────────────────────────────────────────
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
            const event = new KeyboardEvent('keydown', {
                key: 'Enter',
                code: 'Enter',
                keyCode: 13,
                which: 13,
                bubbles: true,
                cancelable: true
            });
            window.dispatchEvent(event);
            document.dispatchEvent(event);
        }, { once: true });
    }
    export function open_url_js(url) {
        window.open(url, '_blank');
    }
    export function apply_terminal_font() {
        const setFont = () => {
            const canvases = document.querySelectorAll('#terminal-container canvas');
            canvases.forEach(canvas => {
                const ctx = canvas.getContext('2d');
                if (ctx) {
                    ctx.font = "16px 'JetBrains Mono', 'Font Awesome 6 Free', 'Font Awesome 6 Brands', monospace";
                }
            });
        };
        setFont();
        setTimeout(setFont, 100);
        setTimeout(setFont, 500);
        setTimeout(setFont, 1000);
        window.addEventListener('resize', setFont);
    }
"#)]
extern "C" {
    fn request_fullscreen();
    fn exit_fullscreen();
    fn setup_fullscreen_click();
    fn open_url_js(url: &str);
    fn apply_terminal_font();
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
        100,
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

    // Apply the custom JetBrains Mono + FontAwesome fonts to the canvas context
    apply_terminal_font();

    terminal.on_key_event({
        let app = app.clone();
        move |key_event| {
            let mut app = app.borrow_mut();
            let key_char = match key_event.code {
                KeyCode::Up => Some('u'),
                KeyCode::Down => Some('d'),
                KeyCode::Left => Some('l'),
                KeyCode::Right => Some('r'),
                KeyCode::Char('b') | KeyCode::Char('B') => Some('b'),
                KeyCode::Char('a') | KeyCode::Char('A') => Some('a'),
                _ => Some('_'),
            };
            if let Some(c) = key_char {
                app.register_key(c);
            }

            if app.boot_mode {
                match key_event.code {
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        app.boot_mode = false;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        exit_fullscreen();
                    }
                    _ => {}
                }
            } else if app.project_detail_view && app.tab_index == 1 {
                // ── Project detail view keybindings (WASM) ──
                match key_event.code {
                    KeyCode::Esc | KeyCode::Backspace => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                    }
                    KeyCode::Enter => {
                        let url = app.get_selected_github_url();
                        open_url_js(url);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.project_detail_scroll = app.project_detail_scroll.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.project_detail_scroll += 1;
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        request_fullscreen();
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        exit_fullscreen();
                    }
                    KeyCode::Tab => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.next_tab();
                    }
                    KeyCode::Char('1') => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.tab_index = 0;
                        app.mark_tab_visited();
                    }
                    KeyCode::Char('2') => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.tab_index = 1;
                        app.mark_tab_visited();
                    }
                    KeyCode::Char('3') => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.tab_index = 2;
                        app.mark_tab_visited();
                    }
                    KeyCode::Char('4') => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.tab_index = 3;
                        app.mark_tab_visited();
                    }
                    KeyCode::Char('5') => {
                        app.project_detail_view = false;
                        app.project_detail_scroll = 0;
                        app.tab_index = 4;
                        app.mark_tab_visited();
                    }
                    _ => {}
                }
            } else if app.tab_index == 1 {
                // ── Project list view keybindings (WASM) ──
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        app.project_select_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        app.project_select_down();
                    }
                    KeyCode::Left | KeyCode::Right => {
                        app.toggle_project_focus();
                    }
                    KeyCode::Enter => {
                        match app.project_focus {
                            ProjectFocus::KnowMore => {
                                app.project_detail_view = true;
                                app.project_detail_scroll = 0;
                            }
                            ProjectFocus::GitHub => {
                                let url = app.get_selected_github_url();
                                open_url_js(url);
                            }
                        }
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        request_fullscreen();
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        exit_fullscreen();
                    }
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                    KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                    KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                    KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                    KeyCode::Char('5') => { app.tab_index = 4; app.mark_tab_visited(); },
                    _ => {}
                }
            } else {
                match key_event.code {
                    KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                    KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                    KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                    KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                    KeyCode::Char('5') => { app.tab_index = 4; app.mark_tab_visited(); },
                    KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        request_fullscreen();
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        exit_fullscreen();
                    }
                    KeyCode::Tab => app.next_tab(),

                    // Skills graph navigation (only on Skills tab)
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') if app.tab_index == 2 => {
                        app.move_skill_selection(0);
                    }
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') if app.tab_index == 2 => {
                        app.move_skill_selection(1);
                    }
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') if app.tab_index == 2 => {
                        app.move_skill_selection(2);
                    }
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') if app.tab_index == 2 => {
                        app.move_skill_selection(3);
                    }

                    // Normal tab navigation (on other tabs)
                    KeyCode::Right | KeyCode::Char('l') => app.next_tab(),
                    KeyCode::Left | KeyCode::Char('h') => app.prev_tab(),
                    _ => {}
                }
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

            let timestep = 1.0 / 120.0;
            let mut app = app_inner.borrow_mut();
            app.fps_tracker.update();

            while *accumulator.borrow_mut() >= timestep {
                app.tick();
                *accumulator.borrow_mut() -= timestep;
            }

            let mut term = terminal_inner.borrow_mut();
            term.draw(|frame| {
                ui::ui(frame, &app);
            }).unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
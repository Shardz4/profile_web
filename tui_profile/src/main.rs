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

// ─── Palette ────────────────────────────────────────────────────────────────
// Colors are now theme-aware via App methods; these are defaults/fallbacks.
const DIM: Color = Color::DarkGray;
const FG: Color = Color::White;
const BG: Color = Color::Black;

// ─── Theme System ───────────────────────────────────────────────────────────
const THEME_COUNT: usize = 6;
const THEME_NAMES: [&str; THEME_COUNT] = [
    "Cyber Neon",
    "Retro Amber",
    "Matrix Green",
    "Vapor Wave",
    "Dracula",
    "Nord Frost",
];

// ─── Responsive Breakpoints ─────────────────────────────────────────────────
const NARROW: u16 = 80;
#[allow(dead_code)]
const MEDIUM: u16 = 120;

// ─── Glitch Text Effect ─────────────────────────────────────────────────────
fn glitch_str(base: &str, tick: u64) -> String {
    let glitch_chars = ['░','▒','▓','█','▄','▀','╗','╔','═','║','▌','▐','╬','┃','┏','┛'];
    let phase = tick % 180;
    if phase < 8 {
        let mut s: Vec<char> = base.chars().collect();
        let len = s.len();
        if len > 4 {
            let pos1 = ((tick.wrapping_mul(7).wrapping_add(3)) as usize) % (len - 2) + 1;
            let pos2 = ((tick.wrapping_mul(13).wrapping_add(5)) as usize) % (len - 2) + 1;
            let gc1 = (tick as usize) % glitch_chars.len();
            let gc2 = ((tick + 3) as usize) % glitch_chars.len();
            s[pos1] = glitch_chars[gc1];
            if pos2 != pos1 {
                s[pos2] = glitch_chars[gc2];
            }
        }
        s.iter().collect()
    } else {
        base.to_string()
    }
}

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

// ─── Achievements ───────────────────────────────────────────────────────────
#[derive(Clone)]
#[allow(dead_code)]
struct Achievement {
    name: &'static str,
    description: &'static str,
    icon: &'static str,
    unlocked: bool,
}

// ─── Particle ─────────────────────────────────────────────────────────────
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ch: char,
    brightness: u8,
}

impl Particle {
    fn new(seed: u64, max_w: u16, max_h: u16) -> Self {
        let chars = ['\u{00b7}', '\u{2218}', '\u{00b0}', '\u{22c5}', '\u{2022}', '\u{2727}', '\u{2729}', '+'];
        let x = (seed.wrapping_mul(31) % max_w as u64) as f32;
        let y = (seed.wrapping_mul(17) % max_h as u64) as f32;
        let vx = ((seed.wrapping_mul(7) % 21) as f32 - 10.0) * 0.008;
        let vy = ((seed.wrapping_mul(13) % 10) as f32 + 2.0) * 0.012;
        let ch = chars[(seed % chars.len() as u64) as usize];
        let brightness = ((seed.wrapping_mul(11) % 60) + 30) as u8;
        Self { x, y, vx, vy, ch, brightness }
    }
}

// ─── Matrix Rain Column ─────────────────────────────────────────────────────
struct MatrixCol {
    head_y: i32,
    speed: u8,
    trail_len: u16,
    chars: Vec<char>,
}

impl MatrixCol {
    fn new(seed: u64, max_height: u16) -> Self {
        let matrix_chars: Vec<char> = "ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺ0123456789ABCDEF<>{}|/\\*+=-~^&@#$%"
            .chars().collect();
        let speed = ((seed % 3) as u8) + 1; // 1–3 ticks per drop (faster)
        let trail_len = ((seed % 13) as u16) + 6; // 6–18 chars (longer trails)
        let start_y = -((seed % (max_height as u64 / 2 + 3)) as i32); // shorter stagger = denser start
        let chars: Vec<char> = (0..trail_len + 6)
            .map(|i| matrix_chars[((seed.wrapping_mul(7).wrapping_add(i as u64 * 13)) % matrix_chars.len() as u64) as usize])
            .collect();
        Self { head_y: start_y, speed, trail_len, chars }
    }
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
    pub boot_mode: bool,
    pub sparkline_data: Vec<u64>,
    pub konami_buffer: Vec<char>,
    pub konami_active: bool,
    pub konami_ticks: u64,
    pub matrix_cols: Vec<MatrixCol>,
    pub tabs_visited: [bool; 4],
    pub achievements: Vec<Achievement>,
    pub achievement_toast: Option<(String, String, u64)>, // (name, desc, show_until_tick)
    pub particles: Vec<Particle>,
    pub current_theme: usize,
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
            boot_mode: true,
            sparkline_data: vec![0; 100],
            konami_buffer: Vec::new(),
            konami_active: false,
            konami_ticks: 0,
            matrix_cols: (0..120).map(|i| MatrixCol::new(i * 37 + 11, 30)).collect(),
            tabs_visited: [true, false, false, false], // Home starts visited
            achievements: vec![
                Achievement {
                    name: "Explorer",
                    description: "Visit all 4 tabs",
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
    pub fn mark_tab_visited(&mut self) {
        if self.tab_index < 4 {
            self.tabs_visited[self.tab_index] = true;
        }
        // Check "Explorer" achievement: all 4 tabs visited
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
        if self.konami_active && self.tick_count.saturating_sub(self.konami_ticks) >= 300 {
            self.konami_active = false;
        }

        // 7. Achievement tick checks
        // "Patient" — 2+ minutes (120 ticks/s * 120s = 14400)
        if self.tick_count == 14400 {
            self.unlock_achievement(2); // Patient
        }
        // "Boot Master" — when boot_mode transitions to false (checked once)
        if !self.boot_mode && self.tick_count > 10 {
            // Only check once around the transition
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
                // Reset when fully off screen
                if col.head_y > 50 + col.trail_len as i32 {
                    *col = MatrixCol::new(self.tick_count.wrapping_mul(7).wrapping_add(i as u64 * 31), 30);
                    col.head_y = -((self.tick_count.wrapping_add(i as u64 * 3) % 5) as i32);
                }
                // Rotate a char in the trail for flicker
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
            // Wrap around screen edges
            if p.x < 0.0 { p.x += 160.0; }
            if p.x >= 160.0 { p.x -= 160.0; }
            if p.y < 0.0 { p.y += 50.0; }
            if p.y >= 50.0 { p.y -= 50.0; }
            // Subtle brightness flicker every ~90 ticks per particle
            if (self.tick_count + i as u64 * 7) % 90 == 0 {
                p.brightness = ((p.brightness as u16 + 15) % 70 + 25) as u8;
            }
        }
    }
    pub fn get_accent_color(&self) -> Color {
        let t = self.tick_count as f64 * 0.02;
        match self.current_theme {
            0 => { // Cyber Neon — cycling green/teal
                let g = ((t.sin() * 40.0) + 215.0) as u8;
                let b = (((t + 3.14).sin() * 40.0) + 215.0) as u8;
                Color::Rgb(0, g, b)
            }
            1 => { // Retro Amber — warm amber phosphor
                let v = ((t.sin() * 20.0) + 235.0) as u8;
                Color::Rgb(v, ((v as f64) * 0.69) as u8, 0)
            }
            2 => { // Matrix Green — pure neon green
                let v = ((t.sin() * 30.0) + 225.0) as u8;
                Color::Rgb(0, v, ((v as f64) * 0.25) as u8)
            }
            3 => { // Vapor Wave — pink/purple cycling
                let r = ((t.sin() * 30.0) + 225.0) as u8;
                let b = (((t + 2.0).sin() * 40.0) + 215.0) as u8;
                Color::Rgb(r, ((r as f64) * 0.44) as u8, b)
            }
            4 => { // Dracula — purple accent
                let v = ((t.sin() * 25.0) + 230.0) as u8;
                Color::Rgb(((v as f64) * 0.74) as u8, ((v as f64) * 0.57) as u8, v)
            }
            5 => { // Nord Frost — cool blue/cyan
                let b = ((t.sin() * 25.0) + 220.0) as u8;
                let g = (((t + 1.5).sin() * 20.0) + 190.0) as u8;
                Color::Rgb(((b as f64) * 0.53) as u8, g, b)
            }
            _ => Color::White,
        }
    }
    pub fn get_theme_name(&self) -> &'static str {
        THEME_NAMES[self.current_theme % THEME_COUNT]
    }
    pub fn cycle_theme(&mut self) {
        self.current_theme = (self.current_theme + 1) % THEME_COUNT;
    }
    pub fn register_key(&mut self, c: char) {
        self.konami_buffer.push(c);
        if self.konami_buffer.len() > 10 {
            self.konami_buffer.remove(0);
        }
        let sequence = ['u', 'u', 'd', 'd', 'l', 'r', 'l', 'r', 'b', 'a'];
        if self.konami_buffer.len() == 10 && self.konami_buffer == sequence {
            self.konami_active = true;
            self.konami_ticks = self.tick_count;
            self.konami_buffer.clear();
            self.unlock_achievement(1); // Retro Gamer
        }
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
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => app.next_tab(),
                        KeyCode::Left | KeyCode::Char('h') | KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                        KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                        KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                        KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                        KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                        _ => {}
                    }
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
            } else {
                match key_event.code {
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => app.next_tab(),
                    KeyCode::Left | KeyCode::Char('h') => app.prev_tab(),
                    KeyCode::Char('1') => { app.tab_index = 0; app.mark_tab_visited(); },
                    KeyCode::Char('2') => { app.tab_index = 1; app.mark_tab_visited(); },
                    KeyCode::Char('3') => { app.tab_index = 2; app.mark_tab_visited(); },
                    KeyCode::Char('4') => { app.tab_index = 3; app.mark_tab_visited(); },
                    KeyCode::Char('t') | KeyCode::Char('T') => app.cycle_theme(),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        request_fullscreen();
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        exit_fullscreen();
                    }
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
                ui(frame, &app);
            }).unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

// ─── Boot UI Screen ──────────────────────────────────────────────────────────
fn render_boot_screen(f: &mut Frame, app: &App, accent: Color) {
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
            Constraint::Length(5),  // Header / Title (reduced from 8 to 5 for compact side-by-side logo layout)
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

fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}

fn render_konami_overlay(f: &mut Frame, app: &App) {
    let area = f.area();
    
    // We want a centered box of size 70 wide, 14 tall
    let popup_area = centered_rect_fixed(70, 14, area);

    // Color cycling for flashy NES look
    let pulse = (app.tick_count / 15) % 3;
    let (border_color, text_color) = match pulse {
        0 => (Color::Rgb(255, 0, 255), Color::Rgb(255, 215, 0)), // Magenta border, Gold text
        1 => (Color::Rgb(255, 215, 0), Color::Rgb(0, 255, 255)), // Gold border, Cyan text
        _ => (Color::Rgb(0, 255, 255), Color::Rgb(255, 0, 255)), // Cyan border, Magenta text
    };

    // Draw the overlay block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG))
        .title(Span::styled(" ♥ CHEAT CODE ACTIVATED ♥ ", Style::default().fg(border_color).add_modifier(Modifier::BOLD)));
    
    f.render_widget(block.clone(), popup_area);

    // Get the inner area of the block
    let inner_area = block.inner(popup_area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // spacing
            Constraint::Length(6), // ASCII art
            Constraint::Length(1), // spacing
            Constraint::Length(1), // Sub-text
            Constraint::Length(1), // Blinking prompt
            Constraint::Min(0),
        ])
        .split(inner_area);

    // ASCII art text lines
    let ascii_art = r#" ██████╗  ██████╗     ██╗     ██╗██╗   ██╗███████╗███████╗██╗
██╔═══██╗██╔═══██╗    ██║     ██║██║   ██║██╔════╝██╔════╝██║
 ▄▄▄▄██║██║   ██║    ██║     ██║██║   ██║█████╗  ███████╗██║
 ▀▀▀▀██║██║   ██║    ██║     ██║╚██╗ ██╔╝██╔══╝  ╚════██║╚═╝
██████╔╝╚██████╔╝    ███████╗██║ ╚████╔╝ ███████╗███████║██╗
╚═════╝  ╚═════╝     ╚══════╝╚═╝  ╚═══╝  ╚══════╝╚══════╝╚═╝"#;

    let ascii_lines: Vec<Line> = ascii_art
        .lines()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(text_color).add_modifier(Modifier::BOLD))))
        .collect();

    let ascii_para = Paragraph::new(ascii_lines).alignment(Alignment::Center);
    f.render_widget(ascii_para, chunks[1]);

    // Sub-text
    let sub_text = Paragraph::new(Line::from(vec![
        Span::styled("CONTRA 1986 // RETRO CHEAT MODE ENABLED", Style::default().fg(Color::Gray)),
    ])).alignment(Alignment::Center);
    f.render_widget(sub_text, chunks[3]);

    // Blinking prompt
    let show_prompt = (app.tick_count / 30) % 2 == 0;
    let prompt_span = if show_prompt {
        Span::styled("★★★★★ 30 LIVES GRANTED ★★★★★", Style::default().fg(border_color).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("", Style::default())
    };
    
    let prompt_para = Paragraph::new(Line::from(vec![prompt_span])).alignment(Alignment::Center);
    f.render_widget(prompt_para, chunks[4]);
}

// ─── Root UI ────────────────────────────────────────────────────────────────
fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let accent = app.get_accent_color();

    // Render particle background as the very first layer
    render_particles(f, app);

    if app.boot_mode {
        render_boot_screen(f, app, accent);
    } else {
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

        render_header(f, chunks[0], accent, app.tick_count);
        render_tabs(f, chunks[1], app, accent);
        render_body(f, chunks[2], app, accent);
        render_footer(f, chunks[3], app, accent);
    }

    if app.konami_active {
        render_konami_overlay(f, app);
    }

    // Achievement toast overlay (top-right corner)
    if let Some((ref name, ref desc, _until)) = app.achievement_toast {
        render_achievement_toast(f, app, name, desc);
    }
}

fn render_particles(f: &mut Frame, app: &App) {
    let area = f.area();
    let accent = app.get_accent_color();
    // Extract accent RGB for tinting particles
    let (ar, ag, ab) = match accent {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 200, 180),
    };
    let buf = f.buffer_mut();
    for p in &app.particles {
        let px = p.x as u16;
        let py = p.y as u16;
        // Only draw within visible area
        if px >= area.x && px < area.x + area.width && py >= area.y && py < area.y + area.height {
            if let Some(cell) = buf.cell_mut((px, py)) {
                // Only render into empty/space cells so particles don't overwrite content
                let sym = cell.symbol().to_string();
                if sym == " " || sym.is_empty() {
                    let dim = (p.brightness as f32) / 255.0 * 0.35; // keep subtle
                    cell.set_char(p.ch);
                    cell.set_fg(Color::Rgb(
                        (ar as f32 * dim) as u8,
                        (ag as f32 * dim) as u8,
                        (ab as f32 * dim) as u8,
                    ));
                }
            }
        }
    }
}

fn render_achievement_toast(f: &mut Frame, app: &App, name: &str, desc: &str) {
    let area = f.area();
    let toast_w: u16 = 34;
    let toast_h: u16 = 7;
    // Position in top-right corner with 2-cell margin
    let x = area.width.saturating_sub(toast_w + 2);
    let y = 2;
    let toast_area = Rect::new(x, y, toast_w.min(area.width), toast_h.min(area.height));

    // Pulsing border color
    let pulse = (app.tick_count / 20) % 3;
    let border_color = match pulse {
        0 => Color::Rgb(255, 215, 0),  // Gold
        1 => Color::Rgb(0, 255, 200),  // Cyan-green
        _ => Color::Rgb(180, 130, 255), // Lavender
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG));
    let inner = block.inner(toast_area);
    f.render_widget(block, toast_area);

    let blink = (app.tick_count / 25) % 2 == 0;
    let star = if blink { "★" } else { "☆" };

    let lines = vec![
        Line::from(Span::styled(
            format!(" {} Achievement Unlocked {}", star, star),
            Style::default().fg(border_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  \"{}\" ", name),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", desc),
            Style::default().fg(Color::Gray),
        )),
    ];

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

// ─── Header ─────────────────────────────────────────────────────────────────
fn render_header(f: &mut Frame, area: Rect, accent: Color, tick: u64) {
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
    // brightness: 0 = empty, 1 = darkest trail, 255 = head
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

// ─── Tab: Contact ────────────────────────────────────────────────────────────
fn render_contact(f: &mut Frame, area: Rect, accent: Color) {
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

    if width < NARROW {
        // Narrow: stack vertically
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
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
    } else {
        // Normal: side-by-side
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        let contact = Paragraph::new(contact_lines)
            .block(Block::default()
                .title(Span::styled(" contact ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)))
            .wrap(Wrap { trim: false });
        f.render_widget(contact, cols[0]);

        let avail = Paragraph::new(avail_lines)
            .block(Block::default()
                .title(Span::styled(" meta ", Style::default().fg(DIM)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM)));
        f.render_widget(avail, cols[1]);
    }
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
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, area);
}

// ─── Custom Mario TUI Widget ─────────────────────────────────────────────────
pub struct MarioWidget {
    pub tick_count: u64,
}

impl Widget for MarioWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 8 {
            return;
        }

        let width = area.width as usize;
        let speed = 4;    // Ticks per column move
        let run_ticks = width * speed;
        let period = run_ticks + 200; // Dynamic period with pause between runs
        let cycle_tick = (self.tick_count % period as u64) as usize;

        if cycle_tick < run_ticks {
            let x = cycle_tick / speed;
            
            // Mario jumps every 45 columns
            let x_mod = x % 45;
            let is_jumping = x_mod >= 15 && x_mod <= 25;

            // Parabolic jump offset (up to 4 rows high, fitting within the 12-row viewport)
            let dy_j = if is_jumping {
                let t = (x_mod - 15) as f64;
                (0.16 * t * (10.0 - t)).round() as u16
            } else {
                0
            };

            // base_y is area.height - 8, so Mario's bottom is exactly at area.height - 1 (top of "Click to continue")
            let base_y = area.height.saturating_sub(8);
            let dy = base_y.saturating_sub(dy_j);

            let start_x = area.left() + x as u16;
            let start_y = area.top() + dy;

            let frame = (self.tick_count / 8) % 2; // running leg frame index

            // NES Small Mario 12x16 pixel art color mappings
            let r_color = Color::Rgb(228, 0, 15);   // Red
            let g_color = Color::Rgb(90, 104, 0);    // Green/Olive
            let p_color = Color::Rgb(255, 166, 0);   // Peach/Skin
            let y_color = Color::Rgb(255, 199, 44);  // Yellow/Gold overalls buttons

            let mut grid = [
                ['.', '.', '.', 'R', 'R', 'R', 'R', 'R', 'R', '.', '.', '.'],
                ['.', '.', 'R', 'R', 'R', 'R', 'R', 'R', 'R', 'R', 'R', '.'],
                ['.', '.', 'G', 'G', 'G', 'P', 'P', 'G', 'P', '.', '.', '.'],
                ['.', 'G', 'P', 'G', 'P', 'P', 'P', 'G', 'P', 'P', 'P', '.'],
                ['.', 'G', 'P', 'G', 'G', 'P', 'P', 'P', 'G', 'P', 'P', 'P'],
                ['.', 'G', 'G', 'P', 'P', 'P', 'P', 'G', 'G', 'G', 'G', '.'],
                ['.', '.', '.', 'P', 'P', 'P', 'P', 'P', 'P', 'P', '.', '.'],
                ['.', '.', '.', 'G', 'R', 'G', 'G', 'G', '.', '.', '.', '.'],
                ['.', 'G', 'G', 'G', 'R', 'G', 'G', 'R', 'G', 'G', 'G', '.'],
                ['G', 'G', 'G', 'G', 'R', 'G', 'G', 'R', 'G', 'G', 'G', 'G'],
                ['P', 'P', 'G', 'R', 'Y', 'R', 'R', 'Y', 'R', 'G', 'P', 'P'],
                ['P', 'P', 'P', 'R', 'R', 'R', 'R', 'R', 'R', 'P', 'P', 'P'],
                ['P', 'P', 'R', 'R', 'R', 'R', 'R', 'R', 'R', 'R', 'P', 'P'],
                ['.', '.', 'R', 'R', 'R', '.', '.', 'R', 'R', 'R', '.', '.'],
                ['.', 'G', 'G', 'G', '.', '.', '.', '.', 'G', 'G', 'G', '.'],
                ['G', 'G', 'G', 'G', '.', '.', '.', '.', 'G', 'G', 'G', 'G'],
            ];

            if frame == 1 && !is_jumping {
                // Animate walking legs by tucking/shifting bottom shoe pixels
                grid[14] = ['.', '.', 'G', 'G', 'G', '.', '.', '.', 'G', 'G', '.', '.'];
                grid[15] = ['.', '.', '.', 'G', 'G', 'G', '.', '.', '.', 'G', 'G', '.'];
            }

            // Render the grid double-pixel style (8 cells high, 12 columns wide)
            for cell_y in 0..8 {
                for col in 0..12 {
                    let p_top = grid[cell_y * 2][col];
                    let p_bottom = grid[cell_y * 2 + 1][col];

                    let c_top = match p_top {
                        'R' => r_color,
                        'G' => g_color,
                        'P' => p_color,
                        'Y' => y_color,
                        _ => BG,
                    };

                    let c_bottom = match p_bottom {
                        'R' => r_color,
                        'G' => g_color,
                        'P' => p_color,
                        'Y' => y_color,
                        _ => BG,
                    };

                    let draw_px = start_x + col as u16;
                    let draw_py = start_y + cell_y as u16;

                    if draw_px < area.right() && draw_py < area.bottom() {
                        let (symbol, style) = match (p_top, p_bottom) {
                            ('.', '.') => {
                                ("", Style::default())
                            }
                            (_, '.') => {
                                ("▀", Style::default().fg(c_top).bg(BG))
                            }
                            ('.', _) => {
                                ("▄", Style::default().fg(c_bottom).bg(BG))
                            }
                            (_, _) => {
                                if p_top == p_bottom {
                                    ("█", Style::default().fg(c_top))
                                } else {
                                    ("▀", Style::default().fg(c_top).bg(c_bottom))
                                }
                            }
                        };

                        if !symbol.is_empty() {
                            buf.set_string(draw_px, draw_py, symbol, style);
                        }
                    }
                }
            }
        }
    }
}
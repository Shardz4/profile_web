// ─── Effects: FPS Tracker, Glitch Text, Particles, Matrix Rain ──────────────
use crate::platform::Instant;

// ─── Glitch Text Effect ─────────────────────────────────────────────────────
pub fn glitch_str(base: &str, tick: u64) -> String {
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
pub struct FpsTracker {
    last_frame: Instant,
    pub fps: f64,
    frame_count: u32,
    last_fps_update: Instant,
}

impl FpsTracker {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            fps: 0.0,
            frame_count: 0,
            last_fps_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
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

// ─── Particle ─────────────────────────────────────────────────────────────
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ch: char,
    pub brightness: u8,
}

impl Particle {
    pub fn new(seed: u64, max_w: u16, max_h: u16) -> Self {
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
pub struct MatrixCol {
    pub head_y: i32,
    pub speed: u8,
    pub trail_len: u16,
    pub chars: Vec<char>,
}

impl MatrixCol {
    pub fn new(seed: u64, max_height: u16) -> Self {
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

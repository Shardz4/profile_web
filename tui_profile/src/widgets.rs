// ─── Custom Mario TUI Widget ─────────────────────────────────────────────────
use crate::platform::*;
use crate::BG;

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

            // base_y is area.height - 8, so Mario's bottom is exactly at area.height - 1
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

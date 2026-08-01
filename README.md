#  Arnav Sharma | WASM Terminal Portfolio

A high-performance, retro-futuristic developer profile and systems monitor emulator built using **Rust**, **Ratatui**, and **Ratzilla**. The application compiles to WebAssembly (WASM) using Trunk, rendering a fully interactive, 120Hz canvas-driven terminal interface directly in the web browser.

---

##  Key Features

*   **Retro CRT Terminal UI**: Styled with responsive terminal grids, custom borders, scanline overlay filters, and dynamic neon glow effects.
*   **Animated Boot Splash Screen**: A vintage telemetry screen featuring real-time diagnostic animations (120Hz core voltage bars, dual-sinusoidal phase charts, real-time frequency analytics spectrums) before loading the main developer profile.
*   **Double-Pixel Mario Widget**: A custom 12x16 NES Small Mario sprite that runs and jumps across the footer of the splash screen, complete with custom frame animations and parabolic gravity physics.
*   **Dynamic Theme Switcher**: 6 harmoniously designed retro-neon themes (`Cyber Neon`, `Retro Amber`, `Matrix Green`, `Vapor Wave`, `Dracula`, and `Nord Frost`) with real-time dynamic RGB accent color cycling.
*   **Interactive Achievements & Badge System**: Unlock badges for exploring tabs, staying on the site, completing the boot sequence, or triggering secret codes, displayed with a dedicated top-right toast overlay and badge UI.
*   **Ambient Particle Background**: A custom physics-simulated particle layer floating behind terminal panels to create visual depth and a premium aesthetic.
*   **Matrix Rain Data Stream**: Dense streams of digital code falling down the home screen with customizable scaling and speed.
*   **WASM & Native Cross-Compilation**: Dual-backend support allows it to run natively in a standard desktop terminal (via `crossterm`) or in the browser (via `ratzilla`'s canvas-driven DOM backend).
*   **Responsive Layout Breakpoints**: Custom grid geometry adapting seamlessly to varying terminal dimensions, scaling from wide multi-column views down to space-efficient vertical lists.
*   **Keyboard Navigation & Easter Eggs**: Switch tabs via arrows, WASD, number keys `1`-`4`, or mouse clicks. Discover secret triggers like the classic Konami Code (`↑↑↓↓←→←→ B A`).

---

##  Key Controls & Navigation

| **Key** | **Action** |
| :--- | :--- |
| **`←` / `→`** or **`h` / `l`** or **`Tab`** | Cycle through tabs |
| **`1` / `2` / `3` / `4`** | Jump to Home / Projects / Skills / Contact tab |
| **`t` / `T`** | Cycle through the 6 visual color themes |
| **`f` / `F`** | Toggle Fullscreen (WASM / Browser only) |
| **`q` / `Q` / `Esc`** | Exit application (Native) or exit Fullscreen (WASM) |
| **`↑` `↑` `↓` `↓` `←` `→` `←` `→` `b` `a`** | Trigger the Konami Code secret easter egg |

---

##  The Tech Arsenal

| **Category** | **Tools & Technologies** |
| :--- | :--- |
| **Low-Level** | `Rust` · `C` · `C++` · `x86_64 Assembly` · `Linux Kernel` |
| **AI & Vision** | `PyTorch` · `OpenCV` · `TensorRT` · `Deep Learning` |
| **Systems** | `Bare-metal OS` · `Memory Management` · `Distributed Systems` |

---

##  Custom TUI Double-Pixel Rendering

A typical terminal character cell has a roughly 1:2 width-to-height aspect ratio. To render block art without stretching, this application utilizes a custom double-pixel rasterizer:
*   **Vertical Half-Blocks**: By utilizing unicode half-block characters (`▀` and `▄`), we fit two vertical pixels inside a single character cell.
*   **1:1 Square Pixels**: Drawing 1 vertical pixel (0.5 character cells high) over 1 character cell wide yields a perfect 1:1 square pixel aspect ratio.
*   **Sprite Mapping**: The `MarioWidget` maps a 12x16 grid of state values (`R` = Red, `G` = Green, `P` = Peach, `Y` = Yellow, `.` = Transparency) into 8 vertical character cells and 12 horizontal character cells, creating a clean NES Small Mario sprite directly in terminal text.

---

##  Setup & Running Locally

### 1. Prerequisites

Make sure you have Rust installed along with the WebAssembly target and `trunk`:

```bash
# Add the WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM builder and packager)
cargo install --locked trunk
```

### 2. Running the Web App (WASM)

Navigate to the `tui_profile` directory and start the Trunk development server:

```bash
cd tui_profile
trunk serve
```

Open your browser and visit: **`http://localhost:8080/`**

### 3. Running Natively (Desktop Terminal)

To run the portfolio directly as a native TUI application in your command line:

```bash
cd tui_profile
cargo run --release
```

---

##  Repository Structure

```
├── tui_profile/
│   ├── src/
│   │   ├── main.rs          # Slim orchestrator and tick loop
│   │   ├── theme.rs         # Theme definitions (6 themes) and accent cycling
│   │   ├── effects.rs       # Animations, FPS tracker, particle systems, matrix rain
│   │   ├── events.rs        # Konami sequence, shell states, achievement badges
│   │   ├── widgets.rs       # Double-pixel custom widgets (e.g. NES Mario Sprite)
│   │   └── ui/              # Render tree layout and tab interfaces
│   │       ├── mod.rs       # UI entry point and layout definition
│   │       ├── boot.rs      # Interactive telemetric boot splash screen
│   │       ├── chrome.rs    # Shell UI chrome (header, tabs, footer controls)
│   │       ├── home.rs      # Profile card, shell simulator, CPU stats, matrix stream
│   │       ├── projects.rs  # Horizontal projects showcase
│   │       ├── skills.rs    # 3-column responsive skills grid
│   │       └── contact.rs   # Interactive contact panel and achievement summary
│   ├── Cargo.toml        # Conditional compilation and crate dependency configs
│   ├── index.html        # Trunk web index wrapper template
│   ├── style.css         # CRT scanlines and DOM canvas styling
│   └── dist/             # (Ignored) Production-ready compiled WASM artifacts
└── README.md             # This document
```

---

## 📬 Contact & Links

*   **Email**: arnav4324@gmail.com
*   **GitHub**: [github.com/Shardz4](https://github.com/Shardz4)
*   **LinkedIn**: [linkedin.com/in/arnav-sharma-z/](https://linkedin.com/in/arnav-sharma-z/)

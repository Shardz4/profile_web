# 👨‍💻 Arnav Sharma | WASM Terminal Portfolio

A high-performance developer profile and systems monitor emulator built using **Rust**, **Ratatui**, and **Ratzilla**. The application compiles to WebAssembly (WASM) using Trunk, rendering a fully interactive, 60+ FPS terminal interface directly in the web browser.

---

## 🚀 Key Features

*   **Retro CRT Terminal UI**: Styled with responsive terminal grids, double borders, scanline overlays, and neon glow effects.
*   **Animated Boot Splash Screen**: A vintage interactive landing page showing telemetric animations (120Hz core voltage bars, dual-sinusoidal phase charts, real-time frequency analytics spectrums) before loading the main developer profile.
*   **Double-Pixel Mario Widget**: A custom 12x16 NES Small Mario sprite that runs and jumps across the footer of the splash screen, complete with animated leg cycles and parabolic gravity physics.
*   **60/120 FPS Animations**: Fixed-timestep update loop syncing terminal rendering to high refresh rate displays.
*   **Dynamic RGB Accent Cycling**: Main borders cycle smoothly through neon cyans, teals, and blues to create a premium, futuristic layout.
*   **WASM & Native Cross-Compilation**: Dual-backend support allows it to run natively in a standard desktop terminal (via `crossterm`) or in the browser (via `ratzilla`'s canvas-driven DOM backend).
*   **Keyboard & Mouse Navigation**: Navigate tabs using `←` / `→` / `Tab`, number keys `1`-`4`, or mouse clicks.

---

## 🛠️ The Tech Arsenal

| **Category** | **Tools & Technologies** |
| :--- | :--- |
| **Low-Level** | `Rust` · `C` · `C++` · `x86_64 Assembly` · `Linux Kernel` |
| **AI & Vision** | `PyTorch` · `OpenCV` · `TensorRT` · `Deep Learning` |
| **Systems** | `Bare-metal OS` · `Memory Management` · `Distributed Systems` |

---

## 👾 Custom TUI Double-Pixel Rendering

A typical terminal character cell has a roughly 1:2 width-to-height aspect ratio. To render block art without stretching, this application utilizes a custom double-pixel rasterizer:
*   **Vertical Half-Blocks**: By utilizing the unicode half-block characters (`▀` and `▄`), we fit two vertical pixels inside a single character cell.
*   **1:1 Square Pixels**: Drawing 1 vertical pixel (0.5 character cells high) over 1 character cell wide yields a perfect 1:1 square pixel aspect ratio.
*   **Sprite Mapping**: The `MarioWidget` maps a 12x16 grid of state values (`R` = Red, `G` = Green, `P` = Peach, `Y` = Yellow, `.` = Transparency) into 8 vertical character cells and 12 horizontal character cells, creating a clean NES Small Mario sprite directly in terminal text.

---


## 🛠️ Setup & Running Locally

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

## 📁 Repository Structure

```
├── tui_profile/
│   ├── src/
│   │   └── main.rs       # Single-source entrypoint matching native & WASM targets
│   ├── Cargo.toml        # Conditional compilation target configs
│   ├── index.html        # Trunk web index wrapper template
│   ├── style.css         # CRT overlay & screen styling
│   └── dist/             # (Ignored) Compiled WASM assets
└── README.md             # This document
```

---

## 📬 Contact & Links

*   **Email**: arnav4324@gmail.com
*   **GitHub**: [github.com/Shardz4](https://github.com/Shardz4)
*   **LinkedIn**: [linkedin.com/in/arnav-sharma-z/](https://linkedin.com/in/arnav-sharma-z/)

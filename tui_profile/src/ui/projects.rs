// ─── Tab: Projects ───────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG};

// ─── Focus state for the two action buttons ─────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectFocus {
    KnowMore,
    GitHub,
}

// ─── Static project data ────────────────────────────────────────────────────
pub struct ProjectData {
    pub name: &'static str,
    pub short_desc: &'static str,
    pub tech_stack: &'static str,
    pub github_url: &'static str,
    pub detail_desc: &'static str,
    pub problem_statement: &'static str,
    pub why_better: &'static str,
}

pub const PROJECT_COUNT: usize = 4;

pub static PROJECTS: [ProjectData; PROJECT_COUNT] = [
    ProjectData {
        name: "ADAS-HybridEngine",
        short_desc: "Real-time ADAS with hybrid Rust+Python perception pipeline for guidance-only driver assistance.",
        tech_stack: "Rust · Python · PyO3 · YOLOv8 · ONNX · OpenCV",
        github_url: "https://github.com/Shardz4/ADAS-HybridEngine",
        detail_desc: "A real-time Advanced Driver Assistance System built with a hybrid Rust + Python \
architecture. Performance-critical perception (lane detection, object tracking, traffic-light \
classification, ONNX inference) runs in compiled Rust via PyO3 bindings, while the application \
layer orchestrates everything in Python with OpenCV and Ultralytics YOLO. The system features \
Canny + Hough lane detection, centroid-based object tracking with TTC estimation, HSV-based \
traffic light classification, and a custom ONNX runtime for traffic sign recognition. The \
Rust source compiles into a Python extension module (adas_pilot) exposing detect_lanes_rust(), \
check_traffic_lights(), RustTracker, RustLaneManager, and AdasBrain with full ONNX inference. \
The Python application layer (main.py) orchestrates the camera loop, AI pipeline, and HUD \
rendering. Training and export utilities allow fine-tuning YOLOv8 on custom traffic sign \
datasets and exporting to ONNX format.",
        problem_statement: "Traditional ADAS solutions are tightly coupled to expensive hardware sensor suites \
(LiDAR, radar arrays, ultrasonic sensors) costing thousands of dollars per vehicle, making \
them completely inaccessible for independent research, aftermarket applications, and developing \
nations. Most commercial systems (Mobileye, Tesla Autopilot) are black-box proprietary stacks \
that cannot be inspected, modified, or extended by researchers. On the open-source side, \
virtually all alternatives run entirely in Python — a language that suffers from GIL \
contention, high per-frame latency, and unpredictable garbage collection pauses that make \
real-time perception unreliable. A typical Python-only YOLO + OpenCV pipeline can barely \
sustain 10-15 FPS on commodity hardware, far below the 25-30 FPS minimum needed for \
responsive driver guidance. Furthermore, most open-source ADAS projects attempt full \
autonomous control (steering, braking), which is both legally complex and dangerous without \
redundant sensor fusion — there is a critical gap for a software-only system that focuses \
purely on guidance (warnings and visual overlays) using a single camera feed, making it \
safe, legal, and deployable on any vehicle with a dashcam.",
        why_better: "By offloading all latency-critical perception kernels to compiled Rust while keeping \
the orchestration layer in Python for flexibility, the system achieves near-native C++ \
performance without sacrificing developer ergonomics or rapid prototyping capability. The \
PyO3 bridge enables zero-copy NumPy array passing between languages — raw camera frames \
flow from Python to Rust without serialization, and detection results flow back as native \
NumPy arrays. The architecture is split into specialized Rust modules: lane_detect.rs \
(Canny + Hough transform), lane_manager.rs (temporal smoothing and ego-lane filtering), \
object_proc.rs (centroid-based multi-object tracking with Time-To-Collision estimation), \
traffic_light.rs (HSV color-space classification), and lib.rs (AdasBrain with custom ONNX \
runtime via the ort crate for traffic sign recognition and Non-Maximum Suppression). Unlike \
full ADAS stacks that aim for autonomous vehicle control, this system focuses purely on \
guidance — providing real-time HUD overlays and audio warnings without ever touching \
steering or braking. This dramatically reduces the safety certification burden while \
remaining genuinely useful as a driver aid. The system runs on commodity hardware with a \
single camera, requires no LiDAR or radar, and the maturin build system produces a single \
pip-installable extension module that works on Linux, macOS, and Windows.",
    },
    ProjectData {
        name: "Lore",
        short_desc: "Decentralized multi-agent network with ZK-proof verified on-chain behavioral data insights.",
        tech_stack: "Go · Rust · TypeScript · Solidity · RISC Zero zkVM · Redis · Next.js",
        github_url: "https://github.com/Shardz4/lore",
        detail_desc: "Lore is a distributed edge architecture for autonomous behavioral data processing. It \
introduces a paradigm shift in AI observability by forcing AI agents to mathematically prove \
their workflows using Zero-Knowledge Proofs (RISC Zero zkVM) and enforcing honesty via an \
Off-Chain Algorithmic Slashing mechanic. The system consists of tiered agentic layers: a \
Scout Agent (Go) for edge ingestion acting as an MCP Client pushing data to Redis Streams, \
an Analyst Agent (Rust) for async stream processing and ZK proof generation acting as the \
Punisher to slash hallucinating agents, a Narrative Agent (Go) interfacing with Google \
Gemini (gemini-2.5-flash) for PM-friendly summaries and exposing the REST API, a Next.js \
Dashboard as the command center displaying a Global Trust Leaderboard, ZK Circuits (Rust) \
for RISC Zero guest/host implementation, and Solidity Smart Contracts \
(LoreZKVerifierLedger.sol) for on-chain Groth16 proof verification. The entire system is \
orchestrated via Docker Compose with Redis Streams as the distributed message broker, and \
OpenTelemetry + Jaeger provide end-to-end tracing from edge ingestion to blockchain commit.",
        problem_statement: "Current AI observability platforms (Datadog AI Monitoring, LangSmith, Weights & \
Biases) trust agent outputs at face value — there is no cryptographic guarantee that an \
AI agent's reported insights actually match the raw data it processed. Enterprise product \
teams making critical business decisions based on AI-generated behavioral analytics have \
zero mechanism to verify the integrity of the analysis pipeline. An agent could hallucinate \
engagement metrics, fabricate user journey insights, or silently drop edge cases, and no \
one would know. Meanwhile, traditional blockchain-based accountability solutions require \
expensive ERC20 token staking to enforce honesty, creating unnecessary financial overhead \
and introducing complex tokenomics that distract from the core problem. The staking model \
also fails to address the fundamental issue: proving that the computational work itself was \
honest, not just that the agent has collateral at risk. Additionally, most observability \
systems expose raw JSON payloads and proprietary enterprise data on-chain or to third-party \
platforms, creating unacceptable data privacy risks for regulated industries like healthcare \
and finance. There is no existing system that simultaneously guarantees computational \
integrity, preserves data privacy, enforces agent accountability without tokens, and \
provides end-to-end traceability from raw telemetry to final insight.",
        why_better: "Lore solves the trust problem without tokens by introducing Algorithmic Slashing — a \
purely mathematical accountability mechanism implemented in reputation.go. Each agent is \
assigned a Trust Score computed as (Success / (Success + Fail)) * 100, with exponential \
penalty factors applied for every hallucination. If an agent's score drops below 60%, the \
system explicitly bans it from committing further data — no token staking, no financial \
overhead, just pure math. The Zero-Knowledge Privacy Layer (RISC Zero zkVM) ensures raw \
enterprise data never leaves the edge node. The Guest Circuit executes inside the VM to \
securely verify that the agent's analysis didn't hallucinate, while only cryptographic \
ZK-SNARK proofs and redacted public journals are submitted on-chain via the \
LoreZKVerifierLedger.sol Groth16 verifier contract. This means enterprise behavioral data \
stays completely private while the integrity of every insight is publicly and permanently \
verifiable on the blockchain. Built-in Dead-Letter Queues (DLQ) ensure zero data loss even \
during LLM provider outages or agent crashes. The system is fully observable via \
OpenTelemetry and Jaeger — every single behavioral event is traceable from the edge \
ingestion point all the way to the final LLM-generated summary. The architecture scales \
horizontally: Scout Agents can be deployed at thousands of edge locations, all streaming \
into Redis, with multiple Analyst Agents consuming and proving in parallel. Deployed at \
lore-sand-kappa.vercel.app with a live Next.js dashboard showing the Global Trust \
Leaderboard and real-time ZK proof submission interface.",
    },
    ProjectData {
        name: "Raven",
        short_desc: "Autonomous AI agent resolving GitHub issues via multi-LLM consensus and Docker-sandboxed verification.",
        tech_stack: "Go · Python · Streamlit · Docker · NATS JetStream · Multi-LLM",
        github_url: "https://github.com/Shardz4/Raven",
        detail_desc: "Raven is an autonomous AI-powered software development agent that resolves GitHub issues \
entirely on autopilot. Given a GitHub issue URL, Raven fetches issue context via the GitHub \
REST API, detects the repository's primary programming language, fans out the prompt to \
multiple LLMs (GPT-4o, Claude Sonnet, DeepSeek, Grok, Ollama) in parallel using goroutine-\
based fan-out with sync.Mutex synchronization, collects generated code patches, verifies \
each patch inside a secure Docker sandbox, scores them through the novel RavenMind multi-\
phase consensus engine, and optionally opens a Pull Request with the winning solution by \
forking the repo, creating a raven/fix-issue-N branch, and committing a language-aware \
solution file. It supports monolithic mode for local dev and fully distributed multi-agent \
mode via NATS JetStream with 9 containerized services (Store, API Server, Orchestrator, \
per-provider Solver workers, Safety Agent, Sandbox Agent, Consensus Agent, PR Agent), with \
Telegram and Discord bot integration for live progress updates via SSE streams.",
        problem_statement: "Traditional AI code generation relies on a single model's output, which is \
fundamentally brittle — a single hallucination, subtle off-by-one error, security \
vulnerability, or misunderstood API contract can slip through completely undetected. \
Existing tools like GitHub Copilot and Cursor generate inline code suggestions but perform \
zero verification — they don't compile the code, don't run tests, don't check for security \
vulnerabilities, and don't validate that the generated solution actually addresses the \
original issue. There is no automated system that treats code generation as an ensemble \
problem, combining multiple AI perspectives with rigorous automated testing to select the \
best solution. Furthermore, current tools require constant human supervision: a developer \
must review every suggestion, manually test it, and decide whether to accept or reject. \
For well-defined bug reports and feature requests in open-source projects, this human-in-\
the-loop overhead is unnecessary and expensive. The industry lacks an end-to-end autonomous \
agent that can take a GitHub issue URL, generate multiple candidate solutions from diverse \
AI models, rigorously test each one in isolation, mathematically select the best candidate, \
and open a production-ready Pull Request — all without any human intervention. Additionally, \
when all candidate solutions fail, existing tools simply give up; there is no self-healing \
mechanism that learns from failure logs and iteratively improves the solutions.",
        why_better: "Raven's key innovation is the RavenMind Consensus engine — a weighted 4-phase \
evaluation pipeline that is fundamentally more reliable than any single model. Phase 1: \
Static Analysis Safety checks scan every candidate patch for dangerous patterns (eval(), \
exec(), os.system(), subprocess calls, file system mutations) before any code is executed. \
Phase 2: Dynamic Sandbox Execution runs each patch inside an isolated Docker container \
(configurable via SANDBOX_IMAGE and DOCKER_TIMEOUT), capturing stdout, stderr, and exit \
codes to verify functional correctness. Phase 3: Structural Similarity Clustering groups \
patches by code structure, rewarding solutions that multiple independent models converged \
on — the intuition being that if GPT-4o, Claude, and DeepSeek all produce structurally \
similar fixes, that fix is likely correct. Phase 4: Independent LLM-as-Judge evaluation \
uses a dedicated judge model (configurable via JUDGE_PROVIDER/JUDGE_MODEL, or a custom \
endpoint) to score each candidate on correctness, code quality, and adherence to the \
original issue requirements. The final weighted composite score selects the winner. When \
all patches fail testing, Raven autonomously self-heals: it feeds the Docker error logs \
and stack traces back to the LLMs and re-prompts them, repeating up to MAX_HEAL_RETRIES \
times — achieving resolution without any human intervention. The Provider abstraction \
(provider.go) enables trivial addition of new LLM backends, and the fan-out architecture \
ensures the system scales linearly with the number of providers. The Streamlit frontend \
provides a dark-themed glassmorphism UI with real-time SSE streaming of the consensus \
process, a dashboard with job history and success rate analytics, and live system health \
monitoring of connected LLM providers.",
    },
    ProjectData {
        name: "CustomCV",
        short_desc: "High-performance Computer Vision library in Rust with zero-copy Python bindings via PyO3.",
        tech_stack: "Rust · Python · PyO3 · rust-numpy · Maturin",
        github_url: "https://github.com/Shardz4/CustomCV",
        detail_desc: "A high-performance Computer Vision library written entirely in Rust (100% Rust codebase) \
with seamless Python bindings. Operations execute natively in Rust through PyO3 + rust-numpy, \
so every function accepts and returns regular NumPy arrays with no data-copy overhead. The \
library implements core CV operations from scratch: color space conversions (rgb_to_gray, \
RGB to HSV), edge detection (apply_canny with configurable low/high thresholds, Sobel), \
morphological operations (apply_dilation, erosion, opening, closing with custom kernels), \
filtering (Gaussian blur, median filter), thresholding, histogram equalization, and geometric \
transforms. Ships with GitHub Actions CI workflow (.github/workflows/CI.yml) building wheels \
for Linux, macOS, and Windows on every push. The library compiles to a single rust_cv_lib \
Python extension module installable via maturin develop --release.",
        problem_statement: "OpenCV is the de facto standard for computer vision in Python, but it is a massive \
C++ monolith with over 2,500 algorithms and complex build dependencies (cmake, libgtk, \
libavcodec, and dozens of optional modules) that make installation a nightmare — especially \
on embedded systems, CI pipelines, and minimal Docker images. The Python API is an auto-\
generated wrapper around the C++ core, leading to inconsistent function signatures, opaque \
error messages that reference C++ internals, and significant data marshalling overhead when \
arrays cross the C++/Python boundary through SWIG bindings. For the vast majority of \
computer vision projects that need only core image processing primitives (grayscale \
conversion, edge detection, morphological operations, filtering), pulling in the entire \
OpenCV ecosystem is massive overkill — a typical pip install opencv-python downloads 60+ MB \
of compiled binaries. Additionally, the lack of memory safety in the C++ core leads to \
subtle bugs in edge cases: buffer overflows on malformed images, use-after-free in multi-\
threaded pipelines, and undefined behavior from uninitialized memory — all of which surface \
as cryptic segfaults that are nearly impossible to debug from the Python side. There is no \
lightweight, memory-safe alternative that provides just the essential CV primitives with a \
Pythonic API and zero-copy performance.",
        why_better: "By implementing every CV primitive from scratch in Rust, the library achieves C++-\
level performance with Rust's compile-time memory safety guarantees — no segfaults, no \
buffer overflows, no use-after-free, no undefined behavior, and no data races in multi-\
threaded contexts. The borrow checker catches entire categories of bugs at compile time \
that would be runtime crashes in C++. The PyO3 + rust-numpy bridge enables true zero-copy \
interop: NumPy arrays are passed directly to Rust functions as ndarray views without any \
serialization, copying, or data marshalling, keeping memory overhead near zero and \
eliminating the SWIG bridge overhead that plagues OpenCV's Python bindings. The library is \
radically lightweight — no OpenCV dependency, no cmake, no system libraries, just a single \
.pyd (Windows) or .so (Linux/macOS) extension module produced by Maturin. The --release \
build flag enables full LLVM optimizations (auto-vectorization, loop unrolling, constant \
folding) that match hand-tuned C++ performance. The API is deliberately Pythonic: functions \
like rgb_to_gray(image), apply_canny(gray, 50.0, 150.0), and apply_dilation(edges, kernel) \
accept and return standard NumPy arrays, making the library a drop-in replacement for \
OpenCV's most commonly used functions. Cross-platform CI via GitHub Actions \
(.github/workflows/CI.yml) ensures pre-built wheels work on all major operating systems, \
and cargo test + cargo clippy maintain code quality on every commit. The library is 100% \
Rust with no unsafe blocks, making it auditable for safety-critical applications.",
    },
];

// ─── Project List View ──────────────────────────────────────────────────────
pub fn render_projects(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let outer_block = Block::default()
        .title(Span::styled(" projects ", Style::default().fg(DIM)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(DIM));

    let inner = outer_block.inner(area);
    f.render_widget(outer_block, area);

    // Split inner area into project card slots
    let card_height = 6u16; // 1 name + 1 desc + 1 blank + 2 buttons + 1 separator
    let constraints: Vec<Constraint> = PROJECTS
        .iter()
        .map(|_| Constraint::Length(card_height))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let slots = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, proj) in PROJECTS.iter().enumerate() {
        if i >= slots.len().saturating_sub(1) {
            break;
        }
        let slot = slots[i];
        let is_selected = app.project_selected_idx == i;

        render_project_card(f, slot, proj, i, is_selected, app.project_focus, accent);
    }
}

fn render_project_card(
    f: &mut Frame,
    area: Rect,
    proj: &ProjectData,
    idx: usize,
    is_selected: bool,
    focus: ProjectFocus,
    accent: Color,
) {
    if area.height < 4 || area.width < 20 {
        return;
    }

    // Row 1: project name + tech stack
    let name_area = Rect::new(area.x, area.y, area.width, 1);
    let idx_style = if is_selected {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(DIM)
    };
    let name_style = if is_selected {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(FG).add_modifier(Modifier::BOLD)
    };
    let selector = if is_selected { "▸" } else { " " };

    let name_line = Line::from(vec![
        Span::styled(format!(" {} ", selector), idx_style),
        Span::styled(format!("{:02}  ", idx + 1), Style::default().fg(DIM)),
        Span::styled(format!("[{}]", proj.name), name_style),
        Span::styled("  ·  ", Style::default().fg(DIM)),
        Span::styled(proj.tech_stack, Style::default().fg(Color::Yellow)),
    ]);
    f.render_widget(Paragraph::new(name_line), name_area);

    // Row 2: short description
    if area.height > 1 {
        let desc_area = Rect::new(area.x, area.y + 1, area.width, 1);
        let desc_line = Line::from(Span::styled(
            format!("       {}", proj.short_desc),
            Style::default().fg(if is_selected { FG } else { Color::Gray }),
        ));
        f.render_widget(Paragraph::new(desc_line), desc_area);
    }

    // Row 3: action buttons (know_more + github)
    if area.height > 2 && is_selected {
        let btn_area = Rect::new(area.x + 7, area.y + 3, area.width.saturating_sub(7), 1);

        let km_focused = focus == ProjectFocus::KnowMore;
        let gh_focused = focus == ProjectFocus::GitHub;

        let km_style = if km_focused {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(DIM)
        };
        let gh_style = if gh_focused {
            Style::default().fg(Color::Rgb(110, 84, 148)).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(DIM)
        };

        let km_label = if km_focused {
            "▶ [ KNOW MORE ]"
        } else {
            "  [ KNOW MORE ]"
        };
        let gh_label = if gh_focused {
            "▶ [ \u{f09b} GITHUB ]"
        } else {
            "  [ \u{f09b} GITHUB ]"
        };

        let btn_line = Line::from(vec![
            Span::styled(km_label, km_style),
            Span::styled("    ", Style::default()),
            Span::styled(gh_label, gh_style),
        ]);
        f.render_widget(Paragraph::new(btn_line), btn_area);
    }
}

// ─── Project Detail View ────────────────────────────────────────────────────
pub fn render_project_detail(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let proj = &PROJECTS[app.project_selected_idx];

    // Outer bordered block with project name as title
    let title = format!(" ▸ {} ", proj.name);
    let block = Block::default()
        .title(Span::styled(&title, Style::default().fg(accent).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(accent));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Layout: github button row | content | footer hints
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // github widget row
            Constraint::Min(0),   // scrollable content
            Constraint::Length(1), // footer hints
        ])
        .split(inner);

    // ── GitHub widget (top-right style) ──────────────────────────────────────
    let gh_style = Style::default().fg(Color::Rgb(110, 84, 148)).add_modifier(Modifier::BOLD);

    let gh_line = Line::from(vec![
        Span::styled("  [ \u{f09b} GITHUB ]", gh_style),
        Span::styled("  ", Style::default()),
        Span::styled(proj.github_url, Style::default().fg(DIM)),
    ]);
    f.render_widget(Paragraph::new(gh_line), chunks[0]);

    // ── Content area ─────────────────────────────────────────────────────────
    let content_area = chunks[1];
    let mut lines: Vec<Line> = Vec::new();

    // Section: DESCRIPTION
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  DESCRIPTION",
        Style::default().fg(accent).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────",
        Style::default().fg(DIM),
    )));
    for text_line in wrap_text(proj.detail_desc, content_area.width.saturating_sub(6) as usize) {
        lines.push(Line::from(Span::styled(
            format!("   {}", text_line),
            Style::default().fg(FG),
        )));
    }

    // Section: TECH STACK
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  TECH STACK",
        Style::default().fg(accent).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ────────────",
        Style::default().fg(DIM),
    )));
    lines.push(Line::from(Span::styled(
        format!("   {}", proj.tech_stack),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));

    // Section: PROBLEM STATEMENT
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  PROBLEM STATEMENT",
        Style::default().fg(Color::Rgb(255, 100, 100)).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ───────────────────",
        Style::default().fg(DIM),
    )));
    for text_line in wrap_text(proj.problem_statement, content_area.width.saturating_sub(6) as usize) {
        lines.push(Line::from(Span::styled(
            format!("   {}", text_line),
            Style::default().fg(Color::Rgb(220, 180, 180)),
        )));
    }

    // Section: WHY THIS SOLUTION IS BETTER
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  WHY THIS SOLUTION IS BETTER",
        Style::default().fg(Color::Rgb(100, 255, 150)).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────",
        Style::default().fg(DIM),
    )));
    for text_line in wrap_text(proj.why_better, content_area.width.saturating_sub(6) as usize) {
        lines.push(Line::from(Span::styled(
            format!("   {}", text_line),
            Style::default().fg(Color::Rgb(200, 255, 210)),
        )));
    }

    lines.push(Line::from(""));

    let paragraph = Paragraph::new(lines)
        .scroll((app.project_detail_scroll, 0));
    f.render_widget(paragraph, content_area);

    // ── Footer hints ─────────────────────────────────────────────────────────
    let hint_line = Line::from(vec![
        Span::styled(" Esc ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled("back", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("Enter ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled("open github", Style::default().fg(DIM)),
        Span::styled("  │  ", Style::default().fg(DIM)),
        Span::styled("↑↓ ", Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled("scroll", Style::default().fg(DIM)),
    ]);
    f.render_widget(Paragraph::new(hint_line).alignment(Alignment::Center), chunks[2]);
}

// ─── Text wrapping helper ───────────────────────────────────────────────────
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() > max_width {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

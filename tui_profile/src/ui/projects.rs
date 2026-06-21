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
traffic light classification, and a custom ONNX runtime for traffic sign recognition.",
        problem_statement: "Traditional ADAS solutions are tightly coupled to expensive hardware sensor suites \
(LiDAR, radar arrays) costing thousands of dollars, making them inaccessible for research and \
aftermarket applications. Most open-source alternatives run entirely in Python, suffering from \
real-time latency issues that make them unreliable for actual driving guidance. There is a gap \
for a software-only, guidance-focused ADAS that can run on commodity hardware with a single \
camera feed while maintaining real-time performance.",
        why_better: "By offloading all latency-critical perception kernels (lane detection, object tracking, \
traffic light classification) to compiled Rust while keeping the orchestration layer in Python \
for flexibility, the system achieves near-native performance without sacrificing developer \
ergonomics. The PyO3 bridge enables zero-copy NumPy array passing between languages. Unlike \
full ADAS stacks that aim for autonomous control, this system focuses purely on guidance \
(warnings and HUD overlays), dramatically reducing the safety certification burden while \
remaining genuinely useful as a driver aid.",
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
Scout Agent (Go) for edge ingestion, an Analyst Agent (Rust) for async stream processing \
and ZK proof generation, a Narrative Agent (Go) interfacing with Gemini for PM-friendly \
summaries, a Next.js Dashboard as the command center, ZK Circuits (Rust) for RISC Zero \
guest/host implementation, and Solidity Smart Contracts for on-chain Groth16 proof \
verification.",
        problem_statement: "Current AI observability platforms trust agent outputs at face value — there is no \
cryptographic guarantee that an AI agent's reported insights actually match the data it \
processed. Enterprise product teams making decisions based on AI-generated behavioral \
analytics have no way to verify the integrity of the analysis pipeline. Meanwhile, \
traditional blockchain-based solutions require expensive ERC20 token staking to enforce \
accountability, creating unnecessary financial overhead.",
        why_better: "Lore solves the trust problem without tokens by introducing Algorithmic Slashing: agents \
are assigned a mathematical trust score based on their success rate, with exponential \
penalties for hallucination. If an agent's score drops below 60%, it is automatically banned. \
The Zero-Knowledge Privacy Layer (RISC Zero zkVM) ensures raw data never leaves the edge — \
only cryptographic proofs are submitted on-chain via a Groth16 verifier contract. This means \
enterprise data stays private while the integrity of every insight is publicly verifiable. \
Built-in Dead-Letter Queues ensure zero data loss even during LLM provider outages.",
    },
    ProjectData {
        name: "Raven",
        short_desc: "Autonomous AI agent resolving GitHub issues via multi-LLM consensus and Docker-sandboxed verification.",
        tech_stack: "Go · Python · Streamlit · Docker · NATS JetStream · Multi-LLM",
        github_url: "https://github.com/Shardz4/Raven",
        detail_desc: "Raven is an autonomous AI-powered software development agent that resolves GitHub issues \
entirely on autopilot. Given a GitHub issue URL, Raven fetches issue context, fans out the \
prompt to multiple LLMs (GPT-4o, Claude Sonnet, DeepSeek, Grok, Ollama) in parallel, \
collects generated code patches, verifies each patch inside a secure Docker sandbox, scores \
them through the novel RavenMind multi-phase consensus engine, and optionally opens a Pull \
Request with the winning solution. It supports monolithic mode for local dev and fully \
distributed multi-agent mode via NATS JetStream, with Telegram and Discord bot integration \
for live progress updates.",
        problem_statement: "Traditional AI code generation relies on a single model's output, which is brittle — \
a single hallucination, subtle bug, or security vulnerability can slip through undetected. \
Existing tools like Copilot and Cursor generate code suggestions but don't verify them. \
There is no automated system that treats code generation as an ensemble problem, combining \
multiple AI perspectives with rigorous automated testing to select the best solution.",
        why_better: "Raven's key innovation is the RavenMind Consensus engine — a weighted 4-phase evaluation \
pipeline combining: (1) static analysis safety checks, (2) dynamic sandbox test execution \
in Docker, (3) structural code similarity clustering, and (4) independent LLM-as-judge \
evaluation. By querying N different LLMs simultaneously and subjecting all candidates to \
this rigorous pipeline, the system is demonstrably more reliable than any single model. \
When all patches fail testing, Raven autonomously self-heals by feeding error logs back \
to the LLMs and re-prompting — achieving resolution without human intervention.",
    },
    ProjectData {
        name: "CustomCV",
        short_desc: "High-performance Computer Vision library in Rust with zero-copy Python bindings via PyO3.",
        tech_stack: "Rust · Python · PyO3 · rust-numpy · Maturin",
        github_url: "https://github.com/Shardz4/CustomCV",
        detail_desc: "A high-performance Computer Vision library written entirely in Rust with seamless Python \
bindings. Operations execute natively in Rust through PyO3 + rust-numpy, so every function \
accepts and returns regular NumPy arrays with no data-copy overhead. The library implements \
core CV operations from scratch: color space conversions (RGB to grayscale, HSV), edge \
detection (Canny, Sobel), morphological operations (dilation, erosion, opening, closing), \
filtering (Gaussian blur, median filter), thresholding, histogram equalization, and \
geometric transforms. Ships with CI/CD building wheels for Linux, macOS, and Windows.",
        problem_statement: "OpenCV is the de facto standard for computer vision in Python, but it is a massive C++ \
monolith with complex build dependencies, inconsistent Python API wrappers, and significant \
data marshalling overhead when crossing the C++/Python boundary. For projects that need only \
core image processing primitives, pulling in the entire OpenCV ecosystem is overkill. \
Additionally, the lack of memory safety in the C++ core leads to subtle bugs in edge cases \
that are difficult to debug from the Python side.",
        why_better: "By implementing CV primitives from scratch in Rust, the library achieves C++-level \
performance with Rust's memory safety guarantees — no segfaults, no buffer overflows, no \
undefined behavior. The PyO3 + rust-numpy bridge enables true zero-copy interop: NumPy \
arrays are passed directly to Rust functions without serialization or copying, keeping \
memory overhead near zero. The library is lightweight (no OpenCV dependency), compiles to \
a single .pyd/.so extension module via Maturin, and the --release builds enable full LLVM \
optimizations. Cross-platform CI ensures wheels work on all major operating systems.",
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

        let km_bg = if km_focused {
            accent
        } else {
            Color::Rgb(22, 27, 34)
        };
        let km_fg = if km_focused {
            Color::Black
        } else {
            accent
        };
        let km_border = if km_focused {
            accent
        } else {
            Color::Rgb(48, 54, 61)
        };

        let gh_bg = if gh_focused {
            Color::Rgb(48, 54, 61)
        } else {
            Color::Rgb(33, 38, 45)
        };
        let gh_fg = if gh_focused {
            Color::Rgb(240, 246, 252)
        } else {
            Color::Rgb(201, 209, 217)
        };
        let gh_border = if gh_focused {
            Color::Rgb(139, 148, 158)
        } else {
            Color::Rgb(48, 54, 61)
        };

        let km_style = Style::default().fg(km_fg).bg(km_bg);
        let gh_style = Style::default().fg(gh_fg).bg(gh_bg).add_modifier(Modifier::BOLD);

        let km_label = if km_focused {
            " ⏎ KNOW MORE "
        } else {
            "   KNOW MORE "
        };
        let gh_label = if gh_focused {
            " ⏎ \u{f09b} GITHUB "
        } else {
            "   \u{f09b} GITHUB "
        };

        let btn_line = Line::from(vec![
            Span::styled("┌", Style::default().fg(km_border).bg(km_bg)),
            Span::styled(km_label, km_style),
            Span::styled("┐", Style::default().fg(km_border).bg(km_bg)),
            Span::styled("  ", Style::default()),
            Span::styled("┌", Style::default().fg(gh_border).bg(gh_bg)),
            Span::styled(gh_label, gh_style),
            Span::styled("┐", Style::default().fg(gh_border).bg(gh_bg)),
        ]);
        f.render_widget(Paragraph::new(btn_line), btn_area);

        // Bottom border of buttons
        if area.height > 3 {
            let border_area = Rect::new(area.x + 7, area.y + 4, area.width.saturating_sub(7), 1);
            let km_w = km_label.chars().count();
            let gh_w = gh_label.chars().count();
            let km_border_line = "─".repeat(km_w);
            let gh_border_line = "─".repeat(gh_w);

            let border_line = Line::from(vec![
                Span::styled("└", Style::default().fg(km_border).bg(km_bg)),
                Span::styled(km_border_line, Style::default().fg(km_border).bg(km_bg)),
                Span::styled("┘", Style::default().fg(km_border).bg(km_bg)),
                Span::styled("  ", Style::default()),
                Span::styled("└", Style::default().fg(gh_border).bg(gh_bg)),
                Span::styled(gh_border_line, Style::default().fg(gh_border).bg(gh_bg)),
                Span::styled("┘", Style::default().fg(gh_border).bg(gh_bg)),
            ]);
            f.render_widget(Paragraph::new(border_line), border_area);
        }
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

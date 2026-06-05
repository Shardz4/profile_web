// ─── Tab: Skills ─────────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{App, DIM, FG, NARROW};

#[derive(Clone, Copy)]
pub struct SkillNode {
    pub id: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub pct: u8,
    pub x: u16,
    pub y: u16,
    pub parents: &'static [&'static str],
    pub desc: &'static str,
    pub level: &'static str,
    // Navigation targets: [Up, Down, Left, Right]
    pub nav: [&'static str; 4],
}

pub const SKILL_NODES: &[SkillNode] = &[
    SkillNode {
        id: "core",
        name: "Arnav (Core)",
        short_name: " Core ",
        pct: 100,
        x: 22,
        y: 11,
        parents: &[],
        desc: "Foundational core of systems programming, OOP, machine learning, and system design.",
        level: "Root",
        nav: ["shell", "docker", "cpp", "py"],
    },
    // Systems Branch
    SkillNode {
        id: "cpp",
        name: "C / C++ Programming",
        short_name: " C++  ",
        pct: 88,
        x: 12,
        y: 9,
        parents: &["core"],
        desc: "Low-level systems code, compiler design, assembly bindings, and performance profiling.",
        level: "Expert",
        nav: ["mem", "rust", "os", "core"],
    },
    SkillNode {
        id: "asm",
        name: "x86_64 Assembly",
        short_name: " Asm  ",
        pct: 70,
        x: 2,
        y: 7,
        parents: &["cpp"],
        desc: "Direct register allocation, BIOS bootloader routines, and inline hardware optimization.",
        level: "Intermediate",
        nav: ["linux", "os", "linux", "cpp"],
    },
    SkillNode {
        id: "os",
        name: "Bare-Metal OS Dev",
        short_name: "  OS  ",
        pct: 80,
        x: 2,
        y: 11,
        parents: &["cpp"],
        desc: "Writing operating system kernels, configuring virtual memory paging, and custom schedulers.",
        level: "Expert",
        nav: ["asm", "dist", "asm", "cpp"],
    },
    SkillNode {
        id: "rust",
        name: "Rust Programming",
        short_name: " Rust ",
        pct: 92,
        x: 12,
        y: 13,
        parents: &["core"],
        desc: "Memory-safe systems programming, concurrency, PyO3 bindings, and WASM compilation.",
        level: "Expert",
        nav: ["cpp", "llvm", "dist", "core"],
    },
    SkillNode {
        id: "dist",
        name: "Distributed Systems",
        short_name: " Dist ",
        pct: 82,
        x: 2,
        y: 15,
        parents: &["rust"],
        desc: "High-performance system design, Raft consensus replication, and concurrent RPC protocols.",
        level: "Advanced",
        nav: ["os", "llvm", "os", "rust"],
    },
    SkillNode {
        id: "llvm",
        name: "LLVM Compiler Internals",
        short_name: " LLVM ",
        pct: 72,
        x: 12,
        y: 17,
        parents: &["cpp"],
        desc: "AST translation, custom LLVM IR optimization passes, and code generation strategies.",
        level: "Intermediate",
        nav: ["rust", "sol", "dist", "docker"],
    },
    SkillNode {
        id: "linux",
        name: "Linux Kernel Development",
        short_name: "Linux ",
        pct: 85,
        x: 2,
        y: 3,
        parents: &["os"],
        desc: "Writing kernel modules, device driver configurations, and Unix file systems.",
        level: "Advanced",
        nav: ["git", "asm", "asm", "mem"],
    },
    SkillNode {
        id: "mem",
        name: "Memory Management",
        short_name: " Mem  ",
        pct: 84,
        x: 12,
        y: 5,
        parents: &["os"],
        desc: "Custom page/slab allocators, virtual address mapping, and cache/TLB optimization.",
        level: "Expert",
        nav: ["linux", "cpp", "linux", "shell"],
    },
    // ML & AI Branch
    SkillNode {
        id: "py",
        name: "Python Programming",
        short_name: "Python",
        pct: 95,
        x: 34,
        y: 9,
        parents: &["core"],
        desc: "Scientific computing, PyO3 Rust extensions, and YOLOv8 deep learning inference pipeline orchestration.",
        level: "Expert",
        nav: ["vim", "opencv", "core", "pytorch"],
    },
    SkillNode {
        id: "pytorch",
        name: "PyTorch & Deep Learning",
        short_name: "PyTorch",
        pct: 90,
        x: 46,
        y: 7,
        parents: &["py"],
        desc: "Neural network training, custom autograd operators, and LibTorch C++ integration.",
        level: "Expert",
        nav: ["vim", "dl", "py", "dl"],
    },
    SkillNode {
        id: "dl",
        name: "Deep Learning Theory",
        short_name: "  DL  ",
        pct: 88,
        x: 46,
        y: 11,
        parents: &["pytorch"],
        desc: "Transformers, CNNs, generative modeling, weight optimization, and backpropagation mechanics.",
        level: "Advanced",
        nav: ["pytorch", "cv", "opencv", "cv"],
    },
    SkillNode {
        id: "cv",
        name: "Computer Vision & OpenCV",
        short_name: "  CV  ",
        pct: 85,
        x: 46,
        y: 15,
        parents: &["dl"],
        desc: "Object detection pipelines, image segmentation, optical flow tracking, feature matching.",
        level: "Advanced",
        nav: ["dl", "db", "opencv", "db"],
    },
    SkillNode {
        id: "opencv",
        name: "OpenCV Library",
        short_name: "OpenCV",
        pct: 86,
        x: 34,
        y: 13,
        parents: &["py"],
        desc: "Digital image processing, real-time matrix filters, convolution, and OpenCV C++ bindings.",
        level: "Advanced",
        nav: ["py", "ai", "core", "dl"],
    },
    SkillNode {
        id: "ai",
        name: "Agentic AI & LLMs",
        short_name: "Agentic",
        pct: 78,
        x: 34,
        y: 17,
        parents: &["dl"],
        desc: "Building autonomous AI agents, tool use, prompt chaining, and large language model integration.",
        level: "Advanced",
        nav: ["opencv", "db", "llvm", "db"],
    },
    SkillNode {
        id: "db",
        name: "PostgreSQL & Databases",
        short_name: " SQL  ",
        pct: 80,
        x: 46,
        y: 19,
        parents: &["py"],
        desc: "Relational database design, query optimization, indexing, and transaction management.",
        level: "Advanced",
        nav: ["cv", "cicd", "ai", "cicd"],
    },
    // Tools Branch
    SkillNode {
        id: "shell",
        name: "Linux & Shell Scripting",
        short_name: "Shell ",
        pct: 86,
        x: 22,
        y: 4,
        parents: &["core"],
        desc: "Bash/zsh scripting, terminal workflows, process signals, and automation pipelines.",
        level: "Expert",
        nav: ["git", "core", "mem", "vim"],
    },
    SkillNode {
        id: "git",
        name: "Git / GitHub Version Control",
        short_name: " Git  ",
        pct: 95,
        x: 22,
        y: 0,
        parents: &["shell"],
        desc: "Rebasing, submodule coordination, hooks automation, and complex git workflows.",
        level: "Expert",
        nav: ["git", "shell", "linux", "vim"],
    },
    SkillNode {
        id: "vim",
        name: "Vim / Neovim Modal Editor",
        short_name: " Vim  ",
        pct: 90,
        x: 34,
        y: 3,
        parents: &["shell"],
        desc: "Custom Lua configurations, AST syntax trees, custom shortcuts, and fast modal editing.",
        level: "Expert",
        nav: ["git", "py", "shell", "pytorch"],
    },
    SkillNode {
        id: "docker",
        name: "Docker Containerization",
        short_name: "Docker",
        pct: 80,
        x: 22,
        y: 18,
        parents: &["core"],
        desc: "Multi-stage builds, rootless container security, and reproducible runtime setups.",
        level: "Advanced",
        nav: ["core", "cicd", "llvm", "ai"],
    },
    SkillNode {
        id: "cicd",
        name: "CI/CD & DevOps",
        short_name: "CI/CD ",
        pct: 78,
        x: 22,
        y: 21,
        parents: &["docker"],
        desc: "GitHub Actions orchestration, regression testing, and container deployment pipelines.",
        level: "Advanced",
        nav: ["docker", "docker", "sol", "db"],
    },
    SkillNode {
        id: "sol",
        name: "Solidity Smart Contracts",
        short_name: "Solidity",
        pct: 88,
        x: 12,
        y: 21,
        parents: &["core"],
        desc: "Smart contract development on Ethereum, CPMM algorithms, security auditing, and gas optimization.",
        level: "Expert",
        nav: ["llvm", "cicd", "llvm", "cicd"],
    },
];

pub fn render_skills(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let width = area.width;

    if width < NARROW {
        render_skills_narrow(f, area, app, accent);
    } else {
        render_skills_wide(f, area, app, accent);
    }
}

// ─── Wide Screen Layout (Interactive Graph + Details Card) ──────────────────
fn render_skills_wide(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(60), // Fixed width for graph canvas
            Constraint::Min(0),     // Detail inspector card
        ])
        .split(area);

    let graph_area = chunks[0];
    let detail_area = chunks[1];

    // Render Graph Box
    let graph_block = Block::default()
        .title(Span::styled(" skill tree & network graph ", Style::default().fg(DIM)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(DIM));
    let graph_inner = graph_block.inner(graph_area);
    f.render_widget(graph_block, graph_area);

    // Render connection lines and nodes directly to the buffer
    let buf = f.buffer_mut();
    let dim_accent = dim_color(accent);

    // 1. Draw connection lines
    for node in SKILL_NODES {
        for parent_id in node.parents {
            if let Some(parent) = SKILL_NODES.iter().find(|n| n.id == *parent_id) {
                draw_connection(buf, parent.x, parent.y, node.x, node.y, graph_inner, dim_accent);
            }
        }
    }

    // 2. Draw nodes on top
    for (idx, node) in SKILL_NODES.iter().enumerate() {
        let is_selected = idx == app.selected_skill_idx;
        draw_node(buf, node, is_selected, graph_inner, accent);
    }

    // Render detail card
    let selected_node = &SKILL_NODES[app.selected_skill_idx];
    render_detail_card(f, detail_area, selected_node, accent);
}

// ─── Narrow Screen Layout (List view + Details Card) ───────────────────────
fn render_skills_narrow(f: &mut Frame, area: Rect, app: &App, accent: Color) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // List of skills
            Constraint::Percentage(45), // Details below
        ])
        .split(area);

    let list_area = chunks[0];
    let detail_area = chunks[1];

    // Render list of skills
    let list_items: Vec<ListItem> = SKILL_NODES
        .iter()
        .enumerate()
        .map(|(idx, node)| {
            let is_selected = idx == app.selected_skill_idx;
            let (bullet, text_style) = if is_selected {
                ("▶ ", Style::default().fg(accent).add_modifier(Modifier::BOLD))
            } else {
                ("  ", Style::default().fg(FG))
            };
            ListItem::new(Line::from(vec![
                Span::styled(bullet, text_style),
                Span::styled(node.name, text_style),
                Span::styled(format!(" ({}%)", node.pct), Style::default().fg(DIM)),
            ]))
        })
        .collect();



    // Let the list show the selected item centered (approximate list state)
    // Ratatui list doesn't auto-scroll unless we pass a state. Since this is a simple list,
    // and we want it to be stateless, we can just slice the items to fit the height!
    // Or we can let it scroll. Let's do a simple slice:
    let height = list_area.height.saturating_sub(2) as usize;
    let start_idx = app.selected_skill_idx.saturating_sub(height / 2).min(SKILL_NODES.len().saturating_sub(height));
    let sliced_items = list_items[start_idx..=(start_idx + height).min(SKILL_NODES.len() - 1)].to_vec();

    let sliced_list = List::new(sliced_items).block(
        Block::default()
            .title(Span::styled(" skill nodes ", Style::default().fg(DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    );
    f.render_widget(sliced_list, list_area);

    // Render details card below
    let selected_node = &SKILL_NODES[app.selected_skill_idx];
    render_detail_card(f, detail_area, selected_node, accent);
}

// ─── Detail Inspector Card ──────────────────────────────────────────────────
fn render_detail_card(f: &mut Frame, area: Rect, node: &SkillNode, accent: Color) {
    let block = Block::default()
        .title(Span::styled(" node details ", Style::default().fg(DIM)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(DIM));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Name
            Constraint::Length(1), // Level
            Constraint::Length(2), // Progress bar
            Constraint::Min(0),    // Description
            Constraint::Length(1), // Prerequisites
        ])
        .split(inner);

    // 1. Name
    let name_para = Paragraph::new(Line::from(Span::styled(
        node.name,
        Style::default().fg(accent).add_modifier(Modifier::BOLD),
    )));
    f.render_widget(name_para, chunks[0]);

    // 2. Level
    let level_para = Paragraph::new(Line::from(vec![
        Span::styled("Level: ", Style::default().fg(DIM)),
        Span::styled(node.level, Style::default().fg(FG).add_modifier(Modifier::BOLD)),
    ]));
    f.render_widget(level_para, chunks[1]);

    // 3. Progress bar
    let bar_w = (chunks[2].width as usize).saturating_sub(10).max(4);
    let progress_bar_str = crate::ui::make_progress_bar(node.pct as f64, bar_w);
    let progress_para = Paragraph::new(Line::from(vec![
        Span::styled("Progress: ", Style::default().fg(DIM)),
        Span::styled(progress_bar_str, Style::default().fg(accent)),
        Span::styled(format!(" {}%", node.pct), Style::default().fg(FG)),
    ]));
    f.render_widget(progress_para, chunks[2]);

    // 4. Description
    let desc_para = Paragraph::new(node.desc)
        .style(Style::default().fg(FG))
        .wrap(Wrap { trim: true });
    f.render_widget(desc_para, chunks[3]);

    // 5. Prerequisites
    let prereq_str = if node.parents.is_empty() {
        "None (Core Root)".to_string()
    } else {
        node.parents
            .iter()
            .map(|id| {
                SKILL_NODES
                    .iter()
                    .find(|n| n.id == *id)
                    .map(|n| n.name)
                    .unwrap_or(id)
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let prereq_para = Paragraph::new(Line::from(vec![
        Span::styled("Prereqs: ", Style::default().fg(DIM)),
        Span::styled(prereq_str, Style::default().fg(Color::Gray)),
    ]));
    f.render_widget(prereq_para, chunks[4]);
}

// ─── Graph Drawing Helpers ──────────────────────────────────────────────────
fn draw_node(buf: &mut Buffer, node: &SkillNode, is_selected: bool, area: Rect, accent: Color) {
    let nx = area.left() + node.x;
    let ny = area.top() + node.y;

    let label = node.short_name;
    let len = label.len() as u16;

    // Draw brackets/pointers
    let (left_bracket, right_bracket, style_bracket, style_text) = if is_selected {
        ("▶", "◀", Style::default().fg(accent).add_modifier(Modifier::BOLD), Style::default().fg(accent).add_modifier(Modifier::BOLD))
    } else {
        ("[", "]", Style::default().fg(Color::DarkGray), Style::default().fg(Color::Gray))
    };

    // Ensure we don't draw outside the area
    if nx < area.right() && ny < area.bottom() {
        buf.set_string(nx, ny, left_bracket, style_bracket);
    }
    for (i, ch) in label.chars().enumerate() {
        let cx = nx + 1 + i as u16;
        if cx < area.right() && ny < area.bottom() {
            buf.set_string(cx, ny, ch.to_string(), style_text);
        }
    }
    let rx = nx + 1 + len;
    if rx < area.right() && ny < area.bottom() {
        buf.set_string(rx, ny, right_bracket, style_bracket);
    }
}

fn draw_connection(buf: &mut Buffer, x1: u16, y1: u16, x2: u16, y2: u16, area: Rect, color: Color) {
    let ax = area.left() + x1 + 3; // Offset by ~3 to connect to center of boxes
    let ay = area.top() + y1;
    let bx = area.left() + x2 + 3;
    let by = area.top() + y2;

    if ay == by {
        // Straight horizontal
        let start_x = ax.min(bx);
        let end_x = ax.max(bx);
        for x in start_x..=end_x {
            if x < area.right() && ay < area.bottom() {
                buf.set_string(x, ay, "─", Style::default().fg(color));
            }
        }
    } else if ax == bx {
        // Straight vertical
        let start_y = ay.min(by);
        let end_y = ay.max(by);
        for y in start_y..=end_y {
            if ax < area.right() && y < area.bottom() {
                buf.set_string(ax, y, "│", Style::default().fg(color));
            }
        }
    } else {
        // Manhattan routing (horizontal then vertical then horizontal)
        let mx = (ax + bx) / 2;

        // First horizontal segment
        let start_x = ax.min(mx);
        let end_x = ax.max(mx);
        for x in start_x..=end_x {
            if x < area.right() && ay < area.bottom() {
                buf.set_string(x, ay, "─", Style::default().fg(color));
            }
        }

        // Vertical segment
        let start_y = ay.min(by);
        let end_y = ay.max(by);
        for y in start_y..=end_y {
            if mx < area.right() && y < area.bottom() {
                buf.set_string(mx, y, "│", Style::default().fg(color));
            }
        }

        // Second horizontal segment
        let start_x = mx.min(bx);
        let end_x = mx.max(bx);
        for x in start_x..=end_x {
            if x < area.right() && by < area.bottom() {
                buf.set_string(x, by, "─", Style::default().fg(color));
            }
        }

        // Corners at bend 1 (mx, ay) and bend 2 (mx, by)
        if mx < area.right() && ay < area.bottom() {
            let c1 = if ax < mx {
                if ay < by { "┐" } else { "┘" }
            } else {
                if ay < by { "┌" } else { "└" }
            };
            buf.set_string(mx, ay, c1, Style::default().fg(color));
        }

        if mx < area.right() && by < area.bottom() {
            let c2 = if ay < by {
                if mx < bx { "┌" } else { "┐" }
            } else {
                if mx < bx { "└" } else { "┘" }
            };
            buf.set_string(mx, by, c2, Style::default().fg(color));
        }
    }
}

fn dim_color(c: Color) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(r / 3, g / 3, b / 3),
        _ => Color::DarkGray,
    }
}

// ─── Tab: Projects ───────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{DIM, FG};

pub fn render_projects(f: &mut Frame, area: Rect, accent: Color) {
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

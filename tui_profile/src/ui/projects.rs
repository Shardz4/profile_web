// ─── Tab: Projects ───────────────────────────────────────────────────────────
use crate::platform::*;
use crate::{DIM, FG};

pub fn render_projects(f: &mut Frame, area: Rect, accent: Color) {
    let projects = vec![
        (
            "[Real-Time ADAS Pilot (Hybrid Engine)]",
            "Rust · Python · PyO3 · YOLOv8",
            "Hybrid perception pipeline with Python YOLOv8 inference and low-latency Rust physics/tracking.",
        ),
        (
            "[Alter-Ego: Universal Prediction Market (DeFi)]",
            "Solidity · Next.js · Wagmi · TypeScript",
            "Decentralized prediction market on Ethereum with CPMM liquidity pool and DAO governance.",
        ),
        (
            "[Bare-Metal OS & Kernel Dev]",
            "C · Rust · x86_64 Assembly",
            "Hobby kernel components, demand paging, custom slab allocators, and interrupt routing.",
        ),
        (
            "[Distributed Consensus Engine]",
            "Rust · Tokio · gRPC",
            "High-performance implementation of Raft consensus protocol with distributed replication.",
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

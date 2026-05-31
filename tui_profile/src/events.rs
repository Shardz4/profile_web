// ─── Events: ShellState, Achievements, Konami Code ──────────────────────────

/// Shell simulator state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellState {
    Typing,
    Running,
    Finished,
}

/// An unlockable achievement badge.
#[derive(Clone)]
#[allow(dead_code)]
pub struct Achievement {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub unlocked: bool,
}

/// The Konami code sequence: ↑↑↓↓←→←→ B A
pub const KONAMI_SEQUENCE: [char; 10] = ['u', 'u', 'd', 'd', 'l', 'r', 'l', 'r', 'b', 'a'];

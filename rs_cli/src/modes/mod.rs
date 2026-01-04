// Application modes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    MainMenu,
    Chat,
    Investigation,
    Debugging,
    SystemStatus,
}

impl Mode {
    /// Get all available modes
    pub fn all() -> Vec<Self> {
        vec![
            Mode::MainMenu,
            Mode::Chat,
            Mode::Investigation,
            Mode::Debugging,
            Mode::SystemStatus,
        ]
    }

    /// Get mode name
    pub fn name(&self) -> &'static str {
        match self {
            Mode::MainMenu => "Main Menu",
            Mode::Chat => "Chat",
            Mode::Investigation => "Investigation",
            Mode::Debugging => "Debugging",
            Mode::SystemStatus => "System Status",
        }
    }
}


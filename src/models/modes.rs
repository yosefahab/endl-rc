pub enum InputMode {
    Normal,
    Typing,
    Command,
    Help,
    Info(String),
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Help
    }
}

impl std::fmt::Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal | Self::Info(_) => write!(f, " Normal Mode "),
            Self::Typing => write!(f, " Typing Mode "),
            Self::Command => write!(f, " Command Mode "),
            Self::Help => write!(f, " Help "),
        }
    }
}

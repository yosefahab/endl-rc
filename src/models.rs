#[allow(unused)]
pub struct User {
    id: u8,
    name: String,
}

pub enum Command {
    Quit,
    Invite,
}
pub enum CommandResult {
    Ok(),
    Error(String),
    QuitSig,
}
pub enum AppState {
    Prompt,
    Normal,
    Typing(String),
    Command(String),
}

impl AppState {
    pub fn set(&mut self, state: AppState) {
        *self = state;
    }
}
impl Default for AppState {
    fn default() -> Self {
        return Self::Prompt;
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prompt => write!(f, "Propmt Mode"),
            Self::Normal => write!(f, "Normal Mode"),
            Self::Command(_) => write!(f, "Command Mode"),
            Self::Typing(_) => write!(f, "Typing Mode"),
        }
    }
}

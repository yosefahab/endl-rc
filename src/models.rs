use ratatui::style::Color;
mod user {
    use super::Color;
    pub struct User {
        pub id: usize,
        pub name: String,
        pub color: Color,
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                id: 0,
                name: String::from("You"), // todo: read user from file
                color: Color::LightBlue,
            }
        }
    }
}

pub mod commands {
    #[allow(dead_code)]
    pub enum Command {
        Unknown,
        Quit,
        Invite,
    }
    pub enum CommandResult {
        Ok(),
        QuitSig,
    }
}

pub mod message {
    use super::Color;
    pub struct Message {
        pub user_id: usize,
        pub content: String,
        pub color: Color,
    }
}
pub mod app {
    use super::{message::Message, user::User};
    use tui_input::Input;

    pub struct Session {
        pub input_mode: InputMode,
        pub users: Vec<User>,
        pub messages: Vec<Message>,
        pub text_buffer: Input,
        pub command_buffer: Input,
    }

    impl Default for Session {
        fn default() -> Self {
            Self {
                input_mode: InputMode::default(),
                text_buffer: Input::default(),
                command_buffer: Input::default(),
                users: vec![User::default()],
                messages: vec![],
            }
        }
    }

    pub enum InputMode {
        Prompt,
        Normal,
        Typing,
        Command,
    }

    impl InputMode {
        pub fn set(&mut self, state: InputMode) {
            *self = state;
        }
    }

    impl Default for InputMode {
        fn default() -> Self {
            Self::Prompt
        }
    }

    impl std::fmt::Display for InputMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Prompt => write!(f, "Propmt Mode"),
                Self::Normal => write!(f, "Normal Mode"),
                Self::Typing => write!(f, "Typing Mode"),
                Self::Command => write!(f, "Command Mode"),
            }
        }
    }
}

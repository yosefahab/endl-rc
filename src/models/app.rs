pub mod mode {
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
                Self::Help => write!(f, " Propmt Mode "),
            }
        }
    }
}
use crate::services::server::Server;

use super::commands::Command;
use super::message::Message;
use super::user::User;
use mode::InputMode;
use tui_input::Input;

pub struct Session {
    pub input_mode: InputMode,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
    pub text_buffer: Input,
    server: Server,
}

impl Session {
    pub fn root_user(&self) -> &User {
        self.nth_user(0)
    }
    pub fn nth_user(&self, id: usize) -> &User {
        self.users.get(id).unwrap()
    }
    pub fn switch_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
    pub fn send_user_msg(&mut self) {
        self.messages.push(Message {
            user_id: self.root_user().id,
            content: self.text_buffer.value().into(),
            color: self.root_user().color,
        });
        self.text_buffer.reset();
    }


    pub fn execute_cmd(&mut self) -> Result<InputMode, ()> {
        // todo
        let mut info = String::new();
        match self.parse_cmd(&mut self.text_buffer.value().to_owned()) {
            Command::Invite => {
                cli_clipboard::set_contents(self.server.get_invite_link()).unwrap();
                info = String::from("Invite Link copied to clipboard");
            }
            Command::Join(link) => {
                self.server
                    .join(link)
                    .unwrap_or_else(|_| info = String::from("Failed to join session"));
            }
            Command::Unknown => {
                info = String::from("Unknown Command!");
            }
        }
        self.text_buffer.reset();
        Ok(InputMode::Info(info))
    }

    fn parse_cmd(&self, cmd: &mut str) -> Command {
        // todo: parse and execute command
        let words: Vec<&str> = cmd.split_whitespace().collect();
        return match words.first() {
            Some(&"inv") => Command::Invite,
            Some(&"join") => if words.len() == 2 { Command::Join(String::from(words[1])) }
            else { Command::Unknown },
                _ => Command::Unknown,
        };
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            input_mode: InputMode::default(),
            text_buffer: Input::default(),
            users: vec![User::default()],
            messages: vec![],
            server: Server::new(),
        }
    }
}

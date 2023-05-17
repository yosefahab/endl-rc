pub mod mode {
    pub enum InputMode {
        Prompt,
        Normal,
        Typing,
        Command,
    }

    impl Default for InputMode {
        fn default() -> Self {
            Self::Prompt
        }
    }

    impl std::fmt::Display for InputMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Prompt => write!(f, " Propmt Mode "),
                Self::Normal => write!(f, " Normal Mode "),
                Self::Typing => write!(f, " Typing Mode "),
                Self::Command => write!(f, " Command Mode "),
            }
        }
    }
}
use mode::InputMode;
use tui_input::Input;
use super::user::User;
use super::message::Message;


pub struct Session {
    pub input_mode: InputMode,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
    pub text_buffer: Input,
    pub command_buffer: Input,
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
    pub fn send_user_msg(&mut self, user_id: usize, msg: String) {
        self.messages.push(Message {
            user_id,
            content: msg,
            color: self.nth_user(user_id).color
        });
        self.text_buffer.reset();
    }
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

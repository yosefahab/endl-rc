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
                Self::Help => write!(f, " Help "),
            }
        }
    }
}
use super::commands::Command;
use super::message::Message;
use super::user::User;
use mode::InputMode;
use tokio::sync::broadcast;
use tui_input::Input;

pub struct Session {
    pub input_mode: InputMode,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
    pub text_buffer: Input,
    messages_tx: broadcast::Sender<Message>,
    messages_rx: broadcast::Receiver<Message>,
}

impl Session {
    pub fn new(messages_tx: broadcast::Sender<Message>) -> Session {
        Session {
            input_mode: InputMode::default(),
            text_buffer: Input::default(),
            users: vec![User::default()],
            messages: vec![],
            messages_rx: messages_tx.subscribe(),
            messages_tx,
        }
    }
    pub fn root_user(&self) -> &User {
        self.nth_user(0)
    }
    pub fn nth_user(&self, id: usize) -> &User {
        self.users.get(id).unwrap()
    }
    pub fn switch_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
    pub async fn send_user_msg(&mut self) {
        self.messages_tx
            .send(Message::new(
                self.root_user().id,
                format!("{}\n", self.text_buffer.value()),
            ))
            .unwrap();
        // empty the text input field
        self.text_buffer.reset();
    }
    pub async fn listen_for_msgs(&mut self) {
        if let Ok(msg) = self.messages_rx.recv().await {
            self.messages.push(Message {
                user_id: msg.user_id,
                content: msg.content,
                color: self.root_user().color,
            });
        }
    }
    pub fn execute_cmd(&mut self) -> Result<InputMode, ()> {
        // TODO
        let mut info = String::new();
        match self.parse_cmd(&mut self.text_buffer.value().to_owned()) {
            Command::Invite => {
                // cli_clipboard::set_contents(self.server.get_invite_link()).unwrap();
                // info = String::from("Invite Link copied to clipboard");
                info = String::from("Not yet implemented!");
            }
            Command::Join(_link) => {
                // self.server
                //     .join(link)
                //     .unwrap_or_else(|_| info = String::from("Failed to join session"));
            }
            Command::Unknown => {
                info = String::from("Unknown Command!");
            }
            Command::Quit => {
                return Err(());
            }
        }
        self.text_buffer.reset();
        Ok(InputMode::Info(info))
    }

    fn parse_cmd(&self, cmd: &mut str) -> Command {
        // todo: parse and execute command
        let words: Vec<&str> = cmd.split_whitespace().collect();
        return match words.first() {
            Some(&"quit") => Command::Quit,
            Some(&"inv") => Command::Invite,
            Some(&"join") => {
                if words.len() == 2 {
                    Command::Join(String::from(words[1]))
                } else {
                    Command::Unknown
                }
            }
            _ => Command::Unknown,
        };
    }
}

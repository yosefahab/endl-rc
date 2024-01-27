use crate::services::server_commands::ServerCommand;

use super::commands::Command;
use super::message::Message;
use super::modes::InputMode;
use super::user::User;
use tokio::sync::{broadcast, watch};
use tui_input::Input;

pub struct Session {
    pub input_mode: InputMode,
    users: Vec<User>,
    pub messages: Vec<Message>,
    pub text_buffer: Input,
    outgoing_messages_tx: broadcast::Sender<Message>,
    incoming_messages_rx: broadcast::Receiver<Message>,
    // used to send commands to server
    server_commands_tx: broadcast::Sender<ServerCommand>,
    // used to signal to server when renderer_task finishes
    exit_signal_tx: watch::Sender<bool>,
}

impl Session {
    pub fn new(server_commands_tx: broadcast::Sender<ServerCommand>) -> Session {
        let (messages_tx, messages_rx) = broadcast::channel::<Message>(10);
        Session {
            input_mode: InputMode::default(),
            text_buffer: Input::default(),
            users: vec![User::default()],
            messages: vec![],
            server_commands_tx,
            incoming_messages_rx: messages_rx,
            outgoing_messages_tx: messages_tx,
            exit_signal_tx: watch::channel(false).0,
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
        let msg = Message::new(
            self.root_user().id,
            format!("{}\n", self.text_buffer.value()),
        );
        if self.outgoing_messages_tx.send(msg.clone()).is_ok() {
            self.messages.push(msg);
            // empty the text input field
            self.text_buffer.reset();
        }
    }
    pub async fn listen_for_msgs(&mut self) {
        if let Ok(msg) = self.incoming_messages_rx.recv().await {
            self.messages.push(msg);
        }
    }
    pub fn execute_cmd(&mut self) -> Result<InputMode, ()> {
        let info: String;
        match self.parse_cmd(&mut self.text_buffer.value().to_owned()) {
            Command::Invite => {
                // TODO: figure out how to pass link from server
                // cli_clipboard::set_contents(self.server.get_invite_link()).unwrap();
                // info = String::from("Invite Link copied to clipboard");
                info = String::from("Not yet implemented!");
            }
            Command::Join(link) => {
                let (exit_signal_tx, exit_signal_rx) = watch::channel::<bool>(false);
                self.exit_signal_tx = exit_signal_tx;
                let (incoming_messages_tx, incoming_messages_rx) =
                    broadcast::channel::<Message>(10);
                let (outgoing_messages_tx, _) = broadcast::channel::<Message>(10);

                self.incoming_messages_rx = incoming_messages_rx;
                self.outgoing_messages_tx = outgoing_messages_tx.clone();
                let _ = self.server_commands_tx.send(ServerCommand::JoinRoom((
                    link.clone(),
                    exit_signal_rx,
                    incoming_messages_tx,
                    outgoing_messages_tx,
                )));
                info = format!("joined room {}", link);
            }
            Command::Unknown => {
                info = String::from("Unknown Command!");
            }
            Command::Quit => {
                return Err(());
            }
            Command::Run => {
                let (exit_signal_tx, exit_signal_rx) = watch::channel::<bool>(false);
                self.exit_signal_tx = exit_signal_tx;
                let (incoming_messages_tx, incoming_messages_rx) =
                    broadcast::channel::<Message>(10);
                let (outgoing_messages_tx, _) = broadcast::channel::<Message>(10);

                self.incoming_messages_rx = incoming_messages_rx;
                self.outgoing_messages_tx = outgoing_messages_tx.clone();
                let _ = self.server_commands_tx.send(ServerCommand::HostRoom((
                    exit_signal_rx,
                    incoming_messages_tx,
                    outgoing_messages_tx,
                )));
                info = String::from("Server running on localhost:8080");
            }
        }
        self.text_buffer.reset();
        Ok(InputMode::Info(info))
    }

    fn verify_join_link(&self, link: String) -> Option<String> {
        // TODO: implement
        Some(link)
    }

    fn parse_cmd(&self, cmd: &mut str) -> Command {
        let words: Vec<&str> = cmd.split_whitespace().collect();
        return match words.first() {
            Some(&"quit") => Command::Quit,
            Some(&"inv") => Command::Invite,
            Some(&"run") => Command::Run,
            Some(&"join") if words.len() == 2 => self
                .verify_join_link(words[1].to_string())
                .map(Command::Join)
                .unwrap_or(Command::Unknown),
            _ => Command::Unknown,
        };
    }
}

use crate::models::message::Message;
use tokio::sync::{broadcast, watch};

#[derive(Debug, Clone)]
pub enum ServerCommand {
    HostRoom(
        (
            watch::Receiver<bool>,
            broadcast::Sender<Message>,
            broadcast::Sender<Message>,
        ),
    ),
    JoinRoom(
        (
            String,
            watch::Receiver<bool>,
            broadcast::Sender<Message>,
            broadcast::Sender<Message>,
        ),
    ),
}

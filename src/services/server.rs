use ratatui::style::Color;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::models::message::Message;

use super::server_commands::ServerCommand;

pub struct Server {
    session_link: String,
}

impl Server {
    pub fn new() -> Server {
        Server {
            session_link: String::from("localhost:8080"),
        }
    }
    async fn join(
        &mut self,
        link: String,
        server_app_messages_tx: broadcast::Sender<Message>,
        app_server_messages_tx: broadcast::Sender<Message>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO
        self.session_link = link;
        let socket: TcpStream = TcpStream::connect(self.session_link.clone()).await?;
        Self::handle_client(socket, server_app_messages_tx, app_server_messages_tx).await;
        Ok(())
    }
    async fn handle_client(
        mut socket: tokio::net::TcpStream,
        server_app_messages_tx: broadcast::Sender<Message>,
        app_server_messages_tx: broadcast::Sender<Message>,
    ) {
        let (socket_reader, mut socket_writer) = socket.split();
        let mut buff_reader = BufReader::new(socket_reader);
        let mut line = String::new();
        let mut app_server_messages_rx = app_server_messages_tx.subscribe();
        loop {
            tokio::select! {
                // socket incoming messages
                bytes_read = buff_reader.read_line(&mut line) => {
                    if let Ok(0) = bytes_read { break; }
                    let new_msg = line.clone();
                    // TODO: actually de-serialize the incoming message to figure out which user sent it
                    if server_app_messages_tx.send(Message::new(new_msg, Color::LightYellow, String::from("Program"))).is_err() { return }
                    line.clear();
                }
                // user messages
                result = app_server_messages_rx.recv() => {
                    let msg = result.unwrap();
                    if (socket_writer.write_all(&msg.as_bytes()).await).is_err() { return }
                }
            }
        }
    }
    async fn run(
        &mut self,
        server_app_messages_tx: broadcast::Sender<Message>,
        app_server_messages_tx: broadcast::Sender<Message>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // wait for incoming connections
        let listener: TcpListener = TcpListener::bind(self.session_link.clone()).await?;

        // TODO: set a limit on the number of clients able to connect
        while let Ok((socket, _)) = listener.accept().await {
            // dispatch a task for each new client
            tokio::spawn(Self::handle_client(
                socket,
                server_app_messages_tx.clone(),
                app_server_messages_tx.clone(),
            ));
        }
        Ok(())
    }
    pub async fn start(
        &mut self,
        mut commands_channel: broadcast::Receiver<ServerCommand>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // listen for commands
            match commands_channel.recv().await? {
                ServerCommand::JoinRoom((
                    room_link,
                    mut exit_signal,
                    server_app_messages_tx,
                    app_server_messages_tx,
                )) => {
                    tokio::select! {
                        _ = self.join(room_link, server_app_messages_tx, app_server_messages_tx) => {}
                        _ = exit_signal.changed() => {}
                    }
                }
                ServerCommand::HostRoom((
                    mut exit_signal,
                    server_app_messages_tx,
                    app_server_messages_tx,
                )) => {
                    tokio::select! {
                        _ = self.run(server_app_messages_tx, app_server_messages_tx) => {}
                        _ = exit_signal.changed() => {}
                    }
                }
            }
        }
    }
}

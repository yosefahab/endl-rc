use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
    sync::oneshot,
};

use crate::models::message::Message;

pub struct Server {
    session_link: String,
    messages_tx: broadcast::Sender<Message>,
}

impl Server {
    pub fn new(messages_tx: broadcast::Sender<Message>) -> Server {
        Server {
            session_link: String::from("localhost:8080"),
            messages_tx,
        }
    }
    // pub fn get_invite_link(&self) -> String {
    //     self.session_link.clone()
    // }
    // pub fn join(&mut self, link: String) -> Result<(), ()> {
    //     self.session_link = link;
    //     Ok(())
    // }
    async fn handle_client(
        mut socket: tokio::net::TcpStream,
        messages_tx: broadcast::Sender<Message>,
    ) {
        let (socket_reader, mut socket_writer) = socket.split();
        let mut buff_reader = BufReader::new(socket_reader);
        let mut line = String::new();
        let mut messages_rx = messages_tx.subscribe();
        loop {
            tokio::select! {
                bytes_read = buff_reader.read_line(&mut line) => {
                    if bytes_read.unwrap() == 0 { break; }
                    let new_msg = line.clone();
                    // TODO: actually de-serialize the incoming message to figure out which user sent it
                    messages_tx.send(Message::new(0, new_msg)).unwrap();
                    line.clear();
                }
                result = messages_rx.recv() => {
                    let msg = result.unwrap();
                    socket_writer.write_all(&msg.as_bytes()).await.unwrap();
                }
            }
        }
    }
    async fn run(
        &mut self,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        while let Ok((socket, _)) = listener.accept().await {
            // dispatch a task for each new client
            // TODO: set a limit on the number of clients able to connect
            tokio::spawn(Self::handle_client(socket, self.messages_tx.clone()));
        }

        Ok(())
    }
    pub async fn start(
        &mut self,
        exit_signal: oneshot::Receiver<bool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // wait for incoming connections
        let listener: TcpListener = TcpListener::bind(self.session_link.clone()).await?;
        let server_status = tokio::select! {
            // break if exit_signal received
            _ = exit_signal => Ok(()) ,
            status = self.run(listener) =>  status
        };
        server_status
    }
}

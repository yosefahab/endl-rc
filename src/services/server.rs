use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
    sync::oneshot,
};

pub struct Server {
    session_link: String,
    app_channel: broadcast::Sender<String>,
}

impl Server {
    pub fn new(app_tx: broadcast::Sender<String>) -> Server {
        Server {
            session_link: String::from("localhost:8080"),
            app_channel: app_tx,
        }
    }
    // pub fn get_invite_link(&self) -> String {
    //     self.session_link.clone()
    // }
    // pub fn join(&mut self, link: String) -> Result<(), ()> {
    //     self.session_link = link;
    //     Ok(())
    // }
    async fn run(&mut self, listener: TcpListener) -> Result<(), ()> {
        let (mut socket, _) = listener.accept().await.unwrap();
        // dispatch a task and give it a tx channel for communication
        let mut rx = self.app_channel.subscribe();
        let tx = self.app_channel.clone();
        tokio::spawn(async move {
            let (socket_reader, mut socket_writer) = socket.split();
            // manages reading from socket
            let mut buff_reader = BufReader::new(socket_reader);
            let mut line = String::new();
            loop {
                // whichever one finishes first
                tokio::select! {
                    // receive message from socket
                    bytes_read = buff_reader.read_line(&mut line) => {
                        // indicates that connection is closed
                        if bytes_read.unwrap() == 0 {
                            break;
                        }
                        // echo back msg
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    },
                    // app tries to send a message
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        // write message to socket
                        socket_writer.write_all(msg.as_bytes()).await.unwrap();
                        // tx.send(msg).unwrap();
                    }
                }
            }
        });
        Ok(())
    }
    pub async fn start(&mut self, exit_signal: oneshot::Receiver<bool>) -> Result<(), ()> {
        // wait for incoming connections
        let listener: TcpListener = TcpListener::bind(self.session_link.clone()).await.unwrap();
        tokio::select! {
            // break if exit_signal received
            _ = exit_signal => { Ok(()) },
            status = self.run(listener) => { status }
        }
    }
}

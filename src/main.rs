use std::{error::Error, io::stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod models;
mod services;
use models::{app::Session, message::Message};
mod views;
use tokio::sync::{broadcast, oneshot};
use views::renderer::start_renderer;

use crate::services::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal, switching to an alternate screen and disabling mouse input
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // used to signal to server when renderer_task finishes
    let (exit_signal_tx, exit_signal_rx) = oneshot::channel();
    let (messages_tx, _messages_rx) = broadcast::channel::<Message>(10);

    let messages_tx_clone = messages_tx.clone();
    let server_task = tokio::spawn(async move {
        let mut server = Server::new(messages_tx_clone);
        server.start(exit_signal_rx).await
    });
    let renderer_task = tokio::spawn(async move {
        let _ = start_renderer(&mut terminal, &mut Session::new(messages_tx)).await;
        let _ = exit_signal_tx.send(true); // signal to server to shutdown
        let _ = disable_raw_mode();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        let _ = terminal.show_cursor();
    });
    let (renderer_res, server_res) = tokio::join!(renderer_task, server_task);

    if renderer_res.is_err() || server_res.is_err() {
        eprintln!("Error during shutdown");
    }
    Ok(())
}

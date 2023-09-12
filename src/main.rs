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
use models::app::Session;
mod view;
use tokio::sync::{broadcast, oneshot};
use view::run_app;

use crate::services::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal, switching to an alternate screen and disabling mouse input
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // used to signal to server when app_task finishes
    let (exit_signal_tx, exit_signal_rx) = oneshot::channel();
    let (tx, _rx) = broadcast::channel::<String>(10);

    let tx_clone = tx.clone();
    let server_task = tokio::spawn(async move {
        let mut server = Server::new(tx_clone);
        let _ = server.start(exit_signal_rx).await;
    });
    let app_task = tokio::spawn(async move {
        let _ = run_app(&mut terminal, &mut Session::new(tx)).await;
        let _ = exit_signal_tx.send(true);
        // signal to server to shutdown
        let _ = disable_raw_mode();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        terminal.show_cursor().unwrap();
    });
    let (app_res, server_res) = tokio::join!(app_task, server_task);

    if app_res.is_err() || server_res.is_err() {
        eprintln!("Error during shutdown");
    }
    Ok(())
}

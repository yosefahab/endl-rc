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
use models::session::Session;
mod views;
use services::server_commands::ServerCommand;
use tokio::sync::broadcast;
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

    let (server_commands_tx, server_commands_rx) = broadcast::channel::<ServerCommand>(1);
    let server_task = tokio::spawn(async move {
        let mut server = Server::new();
        server.start(server_commands_rx).await
    });
    let renderer_task = tokio::spawn(async move {
        let renderer_result =
            start_renderer(&mut terminal, &mut Session::new(server_commands_tx)).await;
        let _ = disable_raw_mode();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        let _ = terminal.show_cursor();
        renderer_result
    });
    let (renderer_res, server_res) = tokio::join!(renderer_task, server_task);

    if renderer_res.is_err() || server_res.is_err() {
        eprintln!("Error during shutdown");
    }
    Ok(())
}

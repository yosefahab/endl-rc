use crossterm::event::{read, Event::Key, KeyCode};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use tui::{Frame, Terminal};

use crate::models::{AppState, Command, CommandResult};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut state: AppState,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|frame| update_ui(frame, &state))?;
        // todo: add non blocking for recieving messages
        if let Key(key) = read()? {
            match state {
                AppState::Prompt => match key.code {
                    KeyCode::Char(':') => {
                        state.set(AppState::Command(String::new()));
                    }
                    KeyCode::Char('t') => {
                        state.set(AppState::Typing(String::new()));
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => state.set(AppState::Normal),
                },
                AppState::Normal => match key.code {
                    KeyCode::Char(':') => {
                        state.set(AppState::Command(String::new()));
                    }
                    KeyCode::Char('t') => {
                        state.set(AppState::Typing(String::new()));
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    // todo: other functionalities
                    _ => (),
                },
                AppState::Command(ref mut cmd) => match key.code {
                    KeyCode::Char(c) => cmd.push(c),
                    KeyCode::Esc => {
                        cmd.clear();
                        state.set(AppState::Normal);
                    }
                    KeyCode::Enter => match execute_cmd(cmd) {
                        CommandResult::QuitSig => return Ok(()),
                        _ => cmd.clear(),
                    },
                    _ => (),
                },
                AppState::Typing(ref mut buffer) => match key.code {
                    KeyCode::Esc => {
                        buffer.clear();
                        state.set(AppState::Normal);
                    }
                    KeyCode::Enter => {
                        // todo: send message
                        buffer.clear();
                    }
                    KeyCode::Char(c) => {
                        buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        buffer.pop();
                    }
                    _ => (),
                },
            };
        }
    }
}
fn get_invite_link() -> String {
    return String::from("");
}
fn execute_cmd(cmd: &mut String) -> CommandResult {
    match parse_cmd(cmd) {
        Command::Quit => CommandResult::QuitSig,
        Command::Invite => {
            get_invite_link();
            CommandResult::Ok()
        }
    }
}
fn parse_cmd(cmd: &mut String) -> Command {
    // todo: parse and execute command
    // if cmd.eq("q".to_string()) {
    // }
    return Command::Quit;
}
fn update_ui<B: Backend>(frame: &mut Frame<B>, state: &AppState) {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.size());

    let chat_box = Block::default()
        .title("Chat")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::LightBlue));

    frame.render_widget(Clear, parent[0]);
    frame.render_widget(chat_box, parent[0]);

    let text_box = textbox(state);
    frame.render_widget(text_box, parent[1]);

    // todo: remove branch
    if let AppState::Prompt = state {
        help_popup(frame);
    }
}
fn textbox<'a>(state: &AppState) -> Paragraph<'a> {
    Paragraph::new(if let AppState::Typing(buffer) = state {
        buffer.to_owned()
    } else {
        String::new()
    })
    .block(
        Block::default()
            .title(state.to_string().to_owned())
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::LightBlue))
}

fn help_popup<B: Backend>(frame: &mut Frame<B>) {
    let prompt_block = Block::default()
        .title("Welcome to the End of Line Club")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Rgb(235, 124, 57)));

    let area = centered_rect(frame.size());
    frame.render_widget(Clear, area);
    frame.render_widget(prompt_block, area);

    let chunk = Layout::default()
        .margin(2)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    const COMMANDS: &str = r#"
Normal Mode:
Press * to ****
Press : to enter Command mode:
    ` Enter t to enter Typing mode
    ` Enter q to Quit
    ` Press Esc to Switch back to Normal mode"#;
    let commands = Paragraph::new(COMMANDS).alignment(Alignment::Left);
    frame.render_widget(commands, chunk[0]);
}
fn centered_rect(area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1])[1]
}

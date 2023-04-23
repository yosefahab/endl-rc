use crate::{
    client,
    models::{
        app::{InputMode, Session},
        commands::{Command, CommandResult},
        message::Message,
    },
};
use crossterm::event::{read, Event, Event::Key, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    text::Spans,
    widgets::ListItem,
    widgets::{Block, BorderType, Borders, Clear, List, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use tui_input::{backend::crossterm::EventHandler, Input};

const COLOR_CLU: Color = Color::Rgb(235, 124, 57);

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Session) -> io::Result<()> {
    loop {
        terminal.draw(|frame| update_ui(frame, app))?;
        // todo: add non blocking for recieving messages
        if let Key(key) = read()? {
            match app.input_mode {
                InputMode::Prompt => match key.code {
                    KeyCode::Char(':') => app.input_mode.set(InputMode::Command),
                    KeyCode::Char('t') => app.input_mode.set(InputMode::Typing),
                    KeyCode::Char('q') => return Ok(()),
                    _ => app.input_mode.set(InputMode::Normal),
                },
                InputMode::Normal => match key.code {
                    KeyCode::Char(':') => app.input_mode.set(InputMode::Command),
                    KeyCode::Char('t') => app.input_mode.set(InputMode::Typing),
                    KeyCode::Char('q') => return Ok(()),
                    // todo: other functionalities (scroll)
                    _ => (),
                },
                InputMode::Command => match key.code {
                    KeyCode::Esc => app.input_mode.set(InputMode::Normal),
                    KeyCode::Enter => {
                        match execute_cmd(&mut app.command_buffer.value().to_string()) {
                            CommandResult::QuitSig => return Ok(()),
                            CommandResult::Ok() => return Ok(()),
                        }
                    }
                    _ => {
                        app.command_buffer.handle_event(&Event::Key(key));
                    }
                },
                InputMode::Typing => match key.code {
                    KeyCode::Esc => {
                        app.input_mode.set(InputMode::Normal);
                    }
                    KeyCode::Enter => {
                        // todo: send message
                        app.messages.push(Message {
                            user_id: 0,
                            content: app.text_buffer.value().into(),
                            color: app.users.get(0).unwrap().color,
                        });
                        app.text_buffer.reset();
                    }
                    _ => {
                        app.text_buffer.handle_event(&Event::Key(key));
                    }
                },
            };
        }
    }
}
fn execute_cmd(cmd: &mut str) -> CommandResult {
    match parse_cmd(cmd) {
        Command::Quit => CommandResult::QuitSig,
        Command::Invite => {
            client::get_invite_link();
            CommandResult::Ok()
        }
        // todo
        Command::Unknown => CommandResult::Ok(),
    }
}
fn parse_cmd(cmd: &mut str) -> Command {
    // todo: parse and execute command
    let words: Vec<&str> = cmd.split_whitespace().collect();
    if let Some(&"q") = words.first() {
        return Command::Quit;
    }
    Command::Unknown
}

fn update_ui<B: Backend>(frame: &mut Frame<B>, app: &mut Session) {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.size());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!(
                " <{}> {}",
                app.users.get(m.user_id).unwrap().name,
                m.content
            )))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Chat")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightBlue)),
    );
    frame.render_widget(messages, parent[0]);

    let width = parent[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.text_buffer.visual_scroll(width as usize);
    let text_box = textbox(&app.input_mode, &app.text_buffer, scroll).wrap(Wrap { trim: false });
    frame.render_widget(text_box, parent[1]);

    if let InputMode::Typing = app.input_mode {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor(
            // Put cursor past the end of the input text
            parent[1].x + ((app.text_buffer.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            parent[1].y + 1,
        )
    }

    // todo: remove branch
    if let InputMode::Prompt = app.input_mode {
        display_help_popup(frame);
    }
}

fn textbox<'a>(state: &InputMode, input: &'a Input, scroll: usize) -> Paragraph<'a> {
    let text = input.value();
    let style = match state {
        InputMode::Typing => Style::default().fg(COLOR_CLU),
        _ => Style::default().fg(Color::LightBlue),
    };
    Paragraph::new(text)
        .style(style)
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(state.to_string()),
        )
}
fn construct_paragraph<'a>(message: &'a str) -> Paragraph<'a> {
    Paragraph::new(message).alignment(Alignment::Center)
}
fn display_help_popup<B: Backend>(frame: &mut Frame<B>) {
    const COMMANDS: &str = r#"
Normal Mode
Press * to ****
Press q to Quit
Press : to enter Command mode:
Press t to enter Typing mode
Press Esc to Switch back to Normal mode"#;

    let commands = construct_paragraph(COMMANDS);
    display_popup(frame, commands);
}
fn display_popup<B: Backend>(frame: &mut Frame<B>, paragraph: Paragraph) {
    let prompt_block = Block::default()
        .title("Welcome to the End of Line Club")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(COLOR_CLU));

    let area = centered_rect(frame.size());
    frame.render_widget(Clear, area);
    frame.render_widget(prompt_block, area);

    let chunk = Layout::default()
        .margin(2)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    frame.render_widget(paragraph, chunk[0]);
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

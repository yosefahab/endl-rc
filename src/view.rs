use crate::{
    models::{
        app::{mode::InputMode, Session},
        message::Message,
    },
    services::{commands::execute_cmd, server::Server},
};
use crossterm::event::{poll, read, Event, Event::Key, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::ListItem,
    widgets::{Block, BorderType, Borders, Clear, List, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, time::Duration};
use tui_input::{backend::crossterm::EventHandler, Input};

const COLOR_CLU: Color = Color::Rgb(235, 124, 57);
const COLOR_TRON: Color = Color::LightBlue;
const BORDER_TYPE: BorderType = BorderType::Rounded;
const BORDERS_DIR: Borders = Borders::ALL;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Session) -> io::Result<()> {
    let server = Server::new();
    loop {
        terminal.draw(|frame| update_ui(frame, app))?;
        if !poll(Duration::from_millis(100))? {
            continue;
        }

        if let Key(key) = read()? {
            match app.input_mode {
                InputMode::Help | InputMode::Info(_) => match key.code {
                    KeyCode::Char(':') => app.switch_mode(InputMode::Command),
                    KeyCode::Char('t') => app.switch_mode(InputMode::Typing),
                    KeyCode::Char('q') => return Ok(()),
                    _ => app.switch_mode(InputMode::Normal),
                },
                InputMode::Normal => match key.code {
                    KeyCode::Char(':') => app.switch_mode(InputMode::Command),
                    KeyCode::Char('t') => app.switch_mode(InputMode::Typing),
                    KeyCode::Char('h') => app.switch_mode(InputMode::Help),
                    KeyCode::Char('q') => return Ok(()),
                    // todo: other functionalities (scroll)
                    _ => (),
                },
                InputMode::Command => match key.code {
                    KeyCode::Esc => app.switch_mode(InputMode::Normal),
                    KeyCode::Enter => {
                        match execute_cmd(&mut app.command_buffer.value().to_string(), &server) {
                            Ok(mode) => app.switch_mode(mode),
                            Err(()) => return Ok(()),
                        }
                    }
                    _ => {
                        app.command_buffer.handle_event(&Event::Key(key));
                    }
                },
                InputMode::Typing => match key.code {
                    KeyCode::Esc => app.switch_mode(InputMode::Normal),
                    KeyCode::Enter => {
                        app.send_user_msg(app.root_user().id, app.text_buffer.value().into())
                    }
                    _ => {
                        app.text_buffer.handle_event(&Event::Key(key));
                    }
                },
            };
        }
    }
}

fn update_ui<B: Backend>(frame: &mut Frame<B>, app: &mut Session) {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.size());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(compose_msg(m, app)))
        .collect();
    let messages = List::new(messages).block(
        Block::default()
            .title(Spans::from(" The Grid "))
            .title_alignment(Alignment::Center)
            .borders(BORDERS_DIR)
            .border_type(BORDER_TYPE)
            .style(Style::default().fg(COLOR_TRON)),
    );
    frame.render_widget(messages, parent[0]);

    let width = parent[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.text_buffer.visual_scroll(width as usize);
    let text_box = textbox(&app.input_mode, &app.text_buffer, scroll).wrap(Wrap { trim: false });
    frame.render_widget(text_box, parent[1]);

    match &app.input_mode {
        InputMode::Typing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                parent[1].x + ((app.text_buffer.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                // Move one line down, from the border to the input line
                parent[1].y + 1,
            )
        }
        InputMode::Info(msg) => display_popup(frame, "INFO", construct_paragraph(&msg)),
        InputMode::Help => display_help_popup(frame),
        _ => {}
    }
}
fn compose_msg<'a>(msg: &Message, app: &Session) -> Vec<Spans<'a>> {
    vec![Spans::from(vec![
        Span::styled(
            format!(" <{}>  ", app.nth_user(msg.user_id).name),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(msg.content.to_string()),
    ])]
}

fn textbox<'a>(state: &InputMode, input: &'a Input, scroll: usize) -> Paragraph<'a> {
    let text = input.value();
    let style = match state {
        InputMode::Typing => Style::default().fg(COLOR_CLU),
        _ => Style::default().fg(COLOR_TRON),
    };
    Paragraph::new(text)
        .style(style)
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(BORDERS_DIR)
                .border_type(BORDER_TYPE)
                .title(state.to_string()),
        )
}
fn construct_paragraph(message: &str) -> Paragraph {
    Paragraph::new(message).alignment(Alignment::Center)
}
fn display_help_popup<B: Backend>(frame: &mut Frame<B>) {
    const COMMANDS: &str = r#"
Normal Mode
Press * to ****
Press q to Quit
Press : to enter Command mode
Press t to enter Typing mode
Press h to show this help message
Press Esc to Switch back to Normal mode"#;

    display_popup(
        frame,
        " Welcome to the End of Line Club ",
        construct_paragraph(COMMANDS),
    );
}
fn display_popup<B: Backend>(frame: &mut Frame<B>, title: &str, paragraph: Paragraph) {
    let prompt_block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(BORDERS_DIR)
        .border_type(BORDER_TYPE)
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

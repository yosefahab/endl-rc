use super::server::Server;
use crate::models::{app::mode::InputMode, commands::Command};

pub fn execute_cmd(cmd: &mut str, server: &Server) -> Result<InputMode, ()> {
    // todo
    let mut info = String::new();
    match parse_cmd(cmd) {
        Command::Invite => {
            cli_clipboard::set_contents(server.get_invite_link()).unwrap();
            info = String::from("Invite Link copied to clipboard");
        }
        Command::Join(link) => {
            server
                .join(link)
                .unwrap_or_else(|_| info = String::from("Failed to join session"));
        }
        Command::Unknown => {
            info = String::from("Unknown Command!");
        }
    }
    Ok(InputMode::Info(info))
}

fn parse_cmd(cmd: &mut str) -> Command {
    // todo: parse and execute command
    let words: Vec<&str> = cmd.split_whitespace().collect();
    return match words.first() {
        Some(&"inv") => Command::Invite,
        Some(&"join") => if words.len() == 2 { Command::Join(String::from(words[1])) }
        else { Command::Unknown },
        _ => Command::Unknown,
    };
}

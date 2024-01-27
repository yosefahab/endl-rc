use ratatui::style::Color;

pub struct User {
    pub id: usize,
    pub name: String,
    pub color: Color,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,                     // root user
            name: String::from("You"), // TODO: read user from file
            color: Color::LightBlue,
        }
    }
}

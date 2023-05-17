use ratatui::style::Color;
pub struct Message {
    pub user_id: usize,
    pub content: String,
    pub color: Color,
}

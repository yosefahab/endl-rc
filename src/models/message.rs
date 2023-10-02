use ratatui::style::Color;
#[derive(Debug, Clone)]
pub struct Message {
    pub user_id: usize,
    pub content: String,
    pub color: Color,
}

impl Message {
    pub fn new(user_id: usize, content: String) -> Self {
        Self {
            user_id,
            content,
            color: Color::LightYellow,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.user_id.to_be_bytes());
        bytes.extend(self.content.as_bytes());
        bytes
    }
}

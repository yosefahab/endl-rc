use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub color: Color,
    pub source: String,
}

impl Message {
    pub fn new(content: String, color: Color, source: String) -> Self {
        Self {
            content,
            color,
            source,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // handle serialization
        // bytes.extend(self.user_id.to_be_bytes());
        bytes.extend(self.content.as_bytes());
        bytes
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        // Ensure sufficient data for deserialization
        if data.len() < std::mem::size_of::<usize>() + std::mem::size_of::<u16>() {
            panic!("Insufficient data to deserialize Message");
        }

        // Extract user_id using big-endian byte order
        let user_id =
            usize::from_be_bytes(data[..std::mem::size_of::<usize>()].try_into().unwrap());

        // Extract content length using big-endian byte order
        let content_len = u16::from_be_bytes(
            data[std::mem::size_of::<usize>()
                ..std::mem::size_of::<usize>() + std::mem::size_of::<u16>()]
                .try_into()
                .unwrap(),
        );

        // Ensure data has enough bytes for the content
        if data.len()
            < std::mem::size_of::<usize>() + std::mem::size_of::<u16>() + content_len as usize
        {
            panic!("Insufficient data for content in Message");
        }

        // Extract content as a string
        let content = String::from_utf8_lossy(
            &data[std::mem::size_of::<usize>() + std::mem::size_of::<u16>()..],
        );

        // Use a default color for now
        Self {
            content: content.to_string(),
            color: Color::LightYellow,
            source: String::from("Program"),
        }
    }
}

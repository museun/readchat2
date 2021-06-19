use super::{Badges, Color, Style};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Colors {
    pub timestamp: Style,
    pub badges: Badges,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            timestamp: Style::fg(Color(0xFF, 0x00, 0xFF)),
            badges: Badges::default(),
        }
    }
}

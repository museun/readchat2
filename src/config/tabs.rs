use super::{Color, Style};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tabs {
    pub active: Style,
    pub inactive: Style,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            active: Style::fg(Color(0xFF, 0x00, 0x00)),
            inactive: Style::fg(Color(0xFF, 0xFF, 0xFF)),
        }
    }
}

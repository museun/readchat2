use super::{Color, Style};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tabs {
    pub active: Style,
    pub inactive: Style,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            active: Style::fg(Color::RED),
            inactive: Style::fg(Color::WHITE),
        }
    }
}

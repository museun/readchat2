use crate::ui::Tab;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TabNameMapping {
    pub status: String,
    pub messages: String,
    pub links: String,
    pub highlights: String,
}

impl Default for TabNameMapping {
    fn default() -> Self {
        Self {
            status: "Status".to_string(),
            messages: "Messages".to_string(),
            links: "Links".to_string(),
            highlights: "Highlights".to_string(),
        }
    }
}

impl TabNameMapping {
    pub fn as_tabs(&self) -> impl Iterator<Item = Tab<'_>> + '_ {
        self.iter()
            .enumerate()
            .map(|(index, text)| Tab { index, text })
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> + '_ {
        <_>::into_iter([
            &*self.status,
            &*self.messages,
            &*self.links,
            &*self.highlights,
        ])
    }
}

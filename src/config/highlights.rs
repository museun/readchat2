use super::{Color, Effects, Style};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Highlights {
    pub mentions: Style,
    pub keywords: Vec<Keyword>,
}

impl Default for Highlights {
    fn default() -> Self {
        let keywords = std::array::IntoIter::new([
            ("Rust", false, Color(0xf5, 0x7c, 0x00), true),
            ("Twitch", true, Color(0x91, 0x46, 0xff), false),
        ])
        .map(|(n, case_sensitive, fg, bold)| Keyword {
            name: n.to_string(),
            case_sensitive,
            style: Style::fg(fg).with_effects(Effects::bold()),
        })
        .collect();

        Self {
            mentions: Style::fg(Color(0xFF, 0x00, 0x00)).with_effects(Effects::bold()),
            keywords,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Keyword {
    pub name: String,
    pub case_sensitive: bool,
    pub style: Style,
}

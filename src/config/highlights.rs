use super::{Color, Effects, Style};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Highlights {
    pub mentions: Style,
    pub keywords: Vec<Keyword>,
}

impl Default for Highlights {
    fn default() -> Self {
        let keywords = <_>::into_iter([
            ("Rust", false, Color::RUSTACEAN_ORANGE, Effects::bold()),
            ("Twitch", true, Color::TWITCH_PURPLE, Effects::empty()),
        ])
        .map(|(n, case_sensitive, fg, effects)| Keyword {
            name: n.to_string(),
            case_sensitive,
            style: Style::fg(fg).with_effects(effects),
        })
        .collect();

        Self {
            mentions: Style::fg(Color::RED).with_effects(Effects::bold()),
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

use super::{Color, Effects, Style};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Highlights {
    pub mention: Style,
    pub keywords: Vec<Keyword>,
}

impl Default for Highlights {
    fn default() -> Self {
        let keywords = <_>::into_iter([
            ("Rust", false, Color::RUSTACEAN_ORANGE, Effects::bold()),
            ("Twitch", true, Color::TWITCH_PURPLE, Effects::empty()),
        ])
        .map(|(name, case_sensitive, fg, effects)| Keyword {
            name: name.to_string(),
            case_sensitive,
            style: Style::fg(fg).with_effects(effects),
        })
        .collect();

        Self {
            mention: Style::fg(Color::RED).with_effects(Effects::bold()),
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

impl PartialEq<str> for Keyword {
    fn eq(&self, input: &str) -> bool {
        (!self.case_sensitive && self.name.eq_ignore_ascii_case(input)) || self.name == input
    }
}

impl Keyword {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            case_sensitive: false,
            style: <_>::default(),
        }
    }

    pub fn case_sensitive(self) -> Self {
        Self {
            case_sensitive: !self.case_sensitive,
            ..self
        }
    }

    pub fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }
}

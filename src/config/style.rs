use super::{Color, Effects};

#[derive(Copy, Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Style {
    #[serde(default = "super::Color::default_fg")]
    pub fg: Color,
    #[serde(default)]
    pub bg: Option<Color>,
    #[serde(default)]
    pub effects: Effects,
}

impl Style {
    pub fn fg(color: Color) -> Self {
        Self {
            fg: color,
            ..Self::default()
        }
    }
    pub const fn with_effects(self, effects: Effects) -> Self {
        Self { effects, ..self }
    }
}

impl From<Style> for cursive::theme::Style {
    fn from(style: Style) -> Self {
        use cursive::theme::{Color as CursiveColor, ColorStyle};

        let Color(r, g, b) = style.fg;
        let front = CursiveColor::Rgb(r, g, b).into();

        let back = style
            .bg
            .map(|Color(r, g, b)| CursiveColor::Rgb(r, g, b))
            .unwrap_or(CursiveColor::TerminalDefault)
            .into();

        Self {
            color: ColorStyle { front, back },
            ..style.effects.into()
        }
    }
}

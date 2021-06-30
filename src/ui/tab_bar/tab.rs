use crate::ui::SpannedAppender as _;
use cursive::{theme::Style, utils::span::SpannedString};

use crate::get_config;

pub struct Tab<'s> {
    pub index: usize,
    pub text: &'s str,
}

impl<'s> Tab<'s> {
    pub fn as_styled_string(&self, focused: bool) -> SpannedString<Style> {
        let crate::config::Tabs { active, inactive } = get_config().tabs;

        if !focused {
            return SpannedString::default()
                .append(" ", inactive)
                .append(self.index.to_string(), inactive)
                .append(". ", inactive)
                .append(self.text, inactive)
                .append(" ", inactive);
        }

        SpannedString::default()
            .append(" ", active)
            .append(self.index.to_string(), active)
            .append(". ", active)
            .append(self.text, active)
            .append(" ", active)
    }
}

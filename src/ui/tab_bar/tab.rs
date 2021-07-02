use crate::{config::Tabs, ui::SpannedAppender as _, Action};
use cursive::{theme::Style, utils::span::SpannedString};

use crate::get_config;

pub struct Tab<'s> {
    pub index: usize,
    pub text: &'s str,
}

impl<'s> Tab<'s> {
    pub fn as_styled_string(&self, focused: bool) -> SpannedString<Style> {
        const CANONICAL_TAB_ORDER: [Action; 4] = [
            Action::FocusStatusView,
            Action::FocusMessagesView,
            Action::FocusLinksView,
            Action::FocusHighlightsView,
        ];

        let config = get_config();
        let Tabs { active, inactive } = config.tabs;

        let name = config
            .keybinds
            .map
            .get(&CANONICAL_TAB_ORDER[self.index])
            .expect("tab to exist")
            .to_string();

        if !focused {
            return SpannedString::default()
                .append(" ", inactive)
                .append(name, inactive)
                .append(". ", inactive)
                .append(self.text, inactive)
                .append(" ", inactive);
        }

        SpannedString::default()
            .append(" ", active)
            .append(name, active)
            .append(". ", active)
            .append(self.text, active)
            .append(" ", active)
    }
}

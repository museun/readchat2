use super::OnView;
use super::SpannedAppender as _;
use crate::{get_config, TabsConfig};

use cursive::{theme::Style, utils::span::SpannedString, views::*, Cursive};

pub struct TabBar<'c>(&'c mut Cursive);
on_view! { TabBar => LinearLayout }

impl<'c> TabBar<'c> {
    pub(crate) const TABS: [Tab<'static>; 4] = [
        Tab {
            index: 0,
            text: " Status",
        },
        Tab {
            index: 1,
            text: " Messages",
        },
        Tab {
            index: 2,
            text: " Links",
        },
        Tab {
            index: 3,
            text: " Highlights",
        },
    ];

    pub(crate) fn select(&mut self, new: usize) {
        self.on(|view| {
            for index in Self::TABS.iter().enumerate().map(|(k, _)| k) {
                let view: &mut TextView = view
                    .get_child_mut(index)
                    .and_then(|view| view.downcast_mut())
                    .map(|view: &mut PaddedView<_>| view.get_inner_mut())
                    .unwrap_or_else(|| panic!("cannot find TextView for {}", index));

                view.get_shared_content()
                    .set_content(Self::TABS[index].as_styled_string(index == new))
            }
        });
    }
}

pub struct Tab<'s> {
    pub index: usize,
    pub text: &'s str,
}

impl<'s> Tab<'s> {
    pub fn as_styled_string(&self, focused: bool) -> SpannedString<Style> {
        let TabsConfig { active, inactive } = get_config().tabs;

        if !focused {
            return SpannedString::default()
                .append(" ", inactive)
                .append(self.index.to_string(), inactive)
                .append(".", inactive)
                .append(self.text, inactive)
                .append(" ", inactive);
        }

        SpannedString::default()
            .append(" ", active)
            .append(self.index.to_string(), active)
            .append(".", active)
            .append(self.text, active)
            .append(" ", active)
    }
}

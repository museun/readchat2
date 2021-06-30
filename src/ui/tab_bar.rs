use crate::get_config;

use super::OnView as _;
use cursive::{views::*, Cursive};

pub struct TabBar<'c>(&'c mut Cursive);
on_view! { TabBar => LinearLayout }

impl<'c> TabBar<'c> {
    pub(crate) fn select(&mut self, new: usize) {
        self.on(|view| {
            let config = get_config();

            for tab in config.tab_names.as_tabs() {
                let view: &mut TextView = view
                    .get_child_mut(tab.index)
                    .and_then(|view| view.downcast_mut())
                    .map(|view: &mut PaddedView<_>| view.get_inner_mut())
                    .unwrap_or_else(|| panic!("cannot find TextView for {}", tab.index));

                view.get_shared_content()
                    .set_content(tab.as_styled_string(tab.index == new))
            }
        });
    }
}

mod tab;
pub use tab::Tab;

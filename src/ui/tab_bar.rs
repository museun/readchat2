use super::OnView as _;
use cursive::{views::*, Cursive};

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

mod tab;
pub use tab::Tab;

use std::sync::atomic::{AtomicUsize, Ordering};

use cursive::{
    direction::Orientation,
    traits::{Nameable, Scrollable, View},
    view::{scroll::Scroller, Margins, ScrollStrategy},
    views::*,
};

pub fn build_ui() -> impl View {
    fn list_view() -> BoxedView {
        let mut list_view = LimitedListView::limited_to(50)
            .scrollable()
            .scroll_x(false)
            .scroll_y(true)
            .scroll_strategy(ScrollStrategy::StickToBottom)
            .show_scrollbars(false);
        list_view.get_scroller_mut().set_scrollbar_padding((0, 0));
        BoxedView::boxed(list_view)
    }

    fn status_view() -> BoxedView {
        BoxedView::boxed(
            ListView::new()
                .scrollable()
                .scroll_x(false)
                .scroll_y(true)
                .scroll_strategy(ScrollStrategy::StickToBottom)
                .show_scrollbars(true),
        )
    }

    fn tab_bar() -> impl View {
        BoxedView::boxed(
            std::array::IntoIter::new(TabBar::TABS)
                .map(|s| s.as_styled_string(false))
                .map(TextView::new)
                .map(|tv| PaddedView::new(Margins::lr(0, 1), tv))
                .fold(
                    LinearLayout::new(Orientation::Horizontal),
                    |layout, view| layout.child(view),
                ),
        )
        .with_name(TabBar::name())
    }

    fn screens_view() -> impl View {
        let mut screens = ScreensView::new();
        screens.add_active_screen(status_view().with_name(StatusView::name()));
        screens.add_screen(list_view().with_name(MessagesView::name()));
        screens.add_screen(list_view().with_name(LinksView::name()));
        screens.add_screen(list_view().with_name(HighlightsView::name()));

        LinearLayout::new(Orientation::Vertical)
            .child(tab_bar())
            .child(BoxedView::boxed(screens).with_name(MainScreen::name()))
    }

    screens_view()
}

pub(crate) fn next_unique_name() -> String {
    const PREFIX: &str = env!("CARGO_PKG_NAME");
    thread_local! { static COUNTER: AtomicUsize = AtomicUsize::new(0); }
    let n = std::thread::LocalKey::with(&COUNTER, move |c| c.fetch_add(1, Ordering::SeqCst));
    format!("{}_view_{}", PREFIX, n)
}

#[macro_use]
mod on_view;
pub use on_view::*;

mod scroll;
pub(crate) use scroll::*;

mod main_screen;
pub(crate) use main_screen::*;

mod messages_view;
pub use messages_view::MessagesView;

mod links_view;
pub(crate) use links_view::*;

mod highlights_view;
pub(crate) use highlights_view::*;

mod tab_bar;
pub(crate) use tab_bar::*;

mod status_view;
pub(crate) use status_view::*;

mod limited_list_view;
use limited_list_view::LimitedListView;

mod spanned_appender;
pub(crate) use spanned_appender::*;

use cursive::views::ScrollView;

use super::{limited_list_view::LimitedListView, OnView};

pub type ScrollableList = ScrollView<LimitedListView>;

pub trait ScrollToBottom<'c, T>
where
    Self: OnView<'c, View = ScrollView<T>>,
{
    #[track_caller]
    fn scroll_to_bottom(&mut self) {
        self.on(|view| view.scroll_to_bottom());
    }
}

impl<'c, T, E> ScrollToBottom<'c, T> for E where E: OnView<'c, View = ScrollView<T>> {}

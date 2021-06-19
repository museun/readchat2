use super::ScrollableList;
use cursive::Cursive;

pub struct MessagesView<'c>(&'c mut Cursive);
on_view! { MessagesView => ScrollableList }

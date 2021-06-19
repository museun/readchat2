use super::ScrollableList;
use cursive::Cursive;

pub struct LinksView<'c>(&'c mut Cursive);
on_view! { LinksView => ScrollableList }

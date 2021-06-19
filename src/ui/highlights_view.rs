use super::ScrollableList;
use cursive::Cursive;

pub struct HighlightsView<'c>(&'c mut Cursive);
on_view! { HighlightsView => ScrollableList }

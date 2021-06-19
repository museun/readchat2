use cursive::{views::*, Cursive};

use super::OnView;
use crate::App;

pub struct MainScreen<'c>(&'c mut Cursive);
on_view! { MainScreen => ScreensView<NamedView<BoxedView>> }

impl<'c> MainScreen<'c> {
    const STATUS_VIEW_INDEX: usize = 0;
    const MESSAGE_VIEW_INDEX: usize = 1;
    const LINKS_VIEW_INDEX: usize = 2;
    const HIGHLIGHTS_VIEW_INDEX: usize = 3;

    pub fn focus(&mut self, index: usize) {
        self.on(|view| view.set_active_screen(index));
        App::select_tab(self.cursive(), index);
    }

    pub fn focus_status_view(&mut self) {
        self.focus(Self::STATUS_VIEW_INDEX)
    }

    pub fn focus_messages_view(&mut self) {
        self.focus(Self::MESSAGE_VIEW_INDEX)
    }

    pub fn focus_links_view(&mut self) {
        self.focus(Self::LINKS_VIEW_INDEX)
    }

    pub fn focus_highlights_view(&mut self) {
        self.focus(Self::HIGHLIGHTS_VIEW_INDEX)
    }
}

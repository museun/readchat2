use cursive::{
    traits::Finder,
    view::{ScrollStrategy, Selector},
    views::{HideableView, TextView},
    Cursive, View,
};

use crate::{entry::Entry, state::AppState, ui::*, Config};

pub struct App;
impl App {
    pub fn quit(cursive: &mut Cursive) {
        cursive.quit()
    }

    pub fn select_tab(cursive: &mut Cursive, index: usize) {
        TabBar::with(cursive).select(index)
    }

    pub fn focus_messages_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_messages_view()
    }

    pub fn focus_links_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_links_view()
    }

    pub fn focus_status_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_status_view()
    }

    pub fn toggle_timestamp(cursive: &mut Cursive) {
        cursive.seek_and_toggle::<TextView, _>("timestamp", |s| &mut s.timestamps)
    }

    pub fn toggle_badges(cursive: &mut Cursive) {
        cursive.seek_and_toggle::<TextView, _>("badge", |s| &mut s.badges)
    }

    pub fn append_raw(cursive: &mut Cursive, raw: String) {
        StatusView::with(cursive).append(Status::Raw(raw))
    }

    #[track_caller]
    pub fn append_entry(cursive: &mut Cursive, entry: Entry) {
        fn append<V>(lv: &mut ScrollableList, view: Option<V>, config: &Config)
        where
            V: View,
        {
            if let Some(view) = view {
                lv.get_inner_mut().add_child(view);
                if lv.is_at_bottom() {
                    lv.set_scroll_strategy(ScrollStrategy::StickToBottom);
                }
            }

            std::array::IntoIter::new([("timestamp", config.timestamps), ("badge", config.badges)])
                .for_each(|(k, v)| {
                    lv.call_on_all(&Selector::Name(k), |view: &mut HideableView<TextView>| {
                        view.set_visible(v)
                    });
                });
        }

        let app_state = cursive
            .user_data::<AppState>()
            .expect("app state must be in the tree")
            .clone();

        MessagesView::with(cursive).on(|view| {
            append(view, entry.as_message_view(&*app_state), &*app_state);
        });

        LinksView::with(cursive).on(|view| {
            append(view, entry.as_links_view(&*app_state), &*app_state);
        });
    }

    pub fn on_ping(cursive: &mut Cursive) {
        StatusView::with(cursive).append(Status::Ping);
    }

    pub fn on_pong(cursive: &mut Cursive) {
        StatusView::with(cursive).append(Status::Pong);
    }

    pub fn on_connecting(cursive: &mut Cursive) {
        StatusView::with(cursive).append(Status::Connecting);
    }

    pub fn on_connected(cursive: &mut Cursive) {
        StatusView::with(cursive).append(Status::Connected);
    }

    pub fn on_joining(cursive: &mut Cursive, channel: String) {
        StatusView::with(cursive).append(Status::Joining(channel));
    }

    pub fn on_joined(cursive: &mut Cursive, channel: String) {
        StatusView::with(cursive).append(Status::Joined(channel));
        StatusView::with(cursive).append(Status::Information);
    }
}

trait FindView {
    fn seek_and_toggle<T, F>(&mut self, key: &str, extract: F)
    where
        T: View,
        F: Fn(&mut Config) -> &mut bool;
}

impl FindView for Cursive {
    fn seek_and_toggle<T, F>(&mut self, key: &str, extract: F)
    where
        T: View,
        F: Fn(&mut Config) -> &mut bool,
    {
        let mut state = self
            .user_data::<AppState>()
            .expect("appstate must be in the tree");
        let config = std::sync::Arc::get_mut(&mut state).expect("no outstanding clones");
        let show = extract(config);
        *show = !*show;
        let show = *show;
        self.call_on_all_named(key, |view: &mut HideableView<T>| view.set_visible(show))
    }
}

use cursive::{
    traits::Finder,
    view::{ScrollStrategy, Selector},
    views::{HideableView, TextView},
    Cursive, View,
};

use crate::{entry::Entry, get_config, ui::*, Config};

pub struct App;
impl App {
    pub fn quit(cursive: &mut Cursive) {
        cursive.quit()
    }

    pub fn select_tab(cursive: &mut Cursive, index: usize) {
        TabBar::with(cursive).select(index)
    }

    pub fn focus_status_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_status_view()
    }

    pub fn focus_messages_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_messages_view()
    }

    pub fn focus_links_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_links_view()
    }

    pub fn focus_highlights_view(cursive: &mut Cursive) {
        MainScreen::with(cursive).focus_highlights_view()
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
        fn append<V>(lv: &mut ScrollableList, view: Option<V>)
        where
            V: View,
        {
            if let Some(view) = view {
                lv.get_inner_mut().add_child(view);
                if lv.is_at_bottom() {
                    lv.set_scroll_strategy(ScrollStrategy::StickToBottom);
                }
            }

            let Config {
                timestamps, badges, ..
            } = *get_config();

            std::array::IntoIter::new([
                ("timestamp", timestamps), //
                ("badge", badges),
            ])
            .for_each(|(k, v)| {
                lv.call_on_all(&Selector::Name(k), |view: &mut HideableView<TextView>| {
                    view.set_visible(v)
                });
            });
        }

        MessagesView::with(cursive).on(|view| {
            append(view, entry.as_message_view());
        });

        LinksView::with(cursive).on(|view| {
            append(view, entry.as_links_view());
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

mod find_view;
pub use find_view::FindView;

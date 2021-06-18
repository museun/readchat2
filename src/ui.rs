use std::sync::atomic::{AtomicUsize, Ordering};

use cursive::{
    direction::Orientation,
    theme::{Color, Effect, Style},
    traits::{Nameable, Scrollable},
    utils::span::SpannedString,
    view::{scroll::Scroller, Margins, ScrollStrategy},
    views::*,
    Cursive, View,
};

use once_cell::sync::Lazy;

mod limited_list_view;
use limited_list_view::LimitedListView;

use crate::App;

fn next_unique_name() -> String {
    const PREFIX: &str = env!("CARGO_PKG_NAME");
    thread_local! { static COUNTER: AtomicUsize = AtomicUsize::new(0); }
    let n = std::thread::LocalKey::with(&COUNTER, move |c| c.fetch_add(1, Ordering::SeqCst));
    format!("{}_view_{}", PREFIX, n)
}

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

        LinearLayout::new(Orientation::Vertical)
            .child(tab_bar())
            .child(BoxedView::boxed(screens).with_name(MainScreen::name()))
    }

    screens_view()
}

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

pub trait OnView<'c>: Sized + 'c {
    type View: View;

    fn with(cursive: &'c mut Cursive) -> Self;
    fn cursive(&mut self) -> &mut Cursive;
    fn name() -> &'static str;

    #[track_caller]
    fn on<F, R>(&mut self, callback: F) -> R
    where
        F: FnOnce(&mut Self::View) -> R,
    {
        let name = Self::name();
        self.cursive()
            .call_on_name(name, |bview: &mut BoxedView| {
                callback(bview.downcast_mut::<Self::View>().unwrap_or_else(|| {
                    let ty = std::any::type_name::<Self::View>();
                    panic!("the wrong type was supplied for {}: {}", name, ty)
                }))
            })
            .unwrap_or_else(|| panic!("cannot find name: {}", name))
    }
}

// #[rustfmt::skip]
macro_rules! on_view {
    ($name:ident => $view:ty) => {
        impl<'c> OnView<'c> for $name<'c> {
            type View = $view;
            fn with(cursive: &'c mut Cursive) -> Self {
                Self(cursive)
            }
            fn cursive(&mut self) -> &mut Cursive {
                &mut self.0
            }
            fn name() -> &'static str {
                static NAME: Lazy<String> = Lazy::new(next_unique_name);
                &*NAME
            }
        }
    };
}

pub struct MainScreen<'c>(&'c mut Cursive);
on_view! { MainScreen => ScreensView<NamedView<BoxedView>> }

impl<'c> MainScreen<'c> {
    const STATUS_VIEW_INDEX: usize = 0;
    const MESSAGE_VIEW_INDEX: usize = 1;
    const LINKS_VIEW_INDEX: usize = 2;

    #[track_caller]
    pub fn focus(&mut self, index: usize) {
        self.on(|view| view.set_active_screen(index));
        App::select_tab(self.cursive(), index);
    }

    #[track_caller]
    pub fn focus_messages_view(&mut self) {
        self.focus(Self::MESSAGE_VIEW_INDEX)
    }

    #[track_caller]
    pub fn focus_links_view(&mut self) {
        self.focus(Self::LINKS_VIEW_INDEX)
    }

    #[track_caller]
    pub fn focus_status_view(&mut self) {
        self.focus(Self::STATUS_VIEW_INDEX)
    }
}

pub struct MessagesView<'c>(&'c mut Cursive);
on_view! { MessagesView => ScrollableList }

pub struct LinksView<'c>(&'c mut Cursive);
on_view! { LinksView => ScrollableList }

pub struct TabBar<'c>(&'c mut Cursive);
on_view! { TabBar => LinearLayout }

impl<'c> TabBar<'c> {
    const TABS: [Tab<'static>; 3] = [
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

struct Tab<'s> {
    index: usize,
    text: &'s str,
}

impl<'s> Tab<'s> {
    fn as_styled_string(&self, focused: bool) -> SpannedString<Style> {
        if !focused {
            return SpannedString::default()
                .append_plain(" ")
                .append_plain(self.index.to_string())
                .append_plain(".")
                .append_plain(self.text)
                .append_plain(" ");
        }

        SpannedString::default()
            .reversed(" ")
            .reversed(self.index.to_string())
            .reversed(".")
            .reversed(self.text)
            .reversed(" ")
    }
}

pub enum Status {
    Raw(String),
    Connecting,
    Connected,
    Ping,
    Pong,
    Joining(String),
    Joined(String),
    Information,
}

pub struct StatusView<'c>(&'c mut Cursive);
on_view! { StatusView => ScrollView<ListView> }

impl<'c> StatusView<'c> {
    #[track_caller]

    pub fn append(&mut self, status: Status) {
        const TEAL: Color = Color::Rgb(0, 128, 128);
        type S = SpannedString<Style>;

        let text = match status {
            Status::Connecting => S::plain("connecting to Twitch..."),
            Status::Connected => S::plain("connected!"),
            Status::Ping => S::plain("ping!"),
            Status::Pong => S::plain("pong!"),
            Status::Joining(channel) => S::plain("joining: ").append(channel, TEAL),
            Status::Joined(channel) => S::plain("joined: ").append(channel, TEAL),
            Status::Raw(..) => return, // ignore this
            Status::Information => {
                App::focus_messages_view(self.cursive());
                return;
            }
        };

        let ts = chrono::Local::now().format("[%c]").to_string();
        let view = TextView::new(text);

        self.on(|inner| inner.get_inner_mut().add_child(&*ts, view))
    }
}

pub trait SpannedAppender {
    fn append<T: Into<String>, S: Into<Style>>(self, text: T, style: S) -> Self;
    fn reversed<T: Into<String>>(self, text: T) -> Self
    where
        Self: Sized,
    {
        self.append(text, Effect::Reverse)
    }
    fn append_plain<T: Into<String>>(self, text: T) -> Self;
}

impl SpannedAppender for SpannedString<Style> {
    fn append<T: Into<String>, S: Into<Style>>(mut self, text: T, style: S) -> Self {
        let this = &mut self;
        this.append_styled(text, style);
        self
    }

    fn append_plain<T: Into<String>>(mut self, text: T) -> Self {
        let this = &mut self;
        this.append_plain(text);
        self
    }
}

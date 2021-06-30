use super::{OnView, SpannedAppender as _};
use crate::App;

use cursive::{theme::Style, utils::span::SpannedString, views::*, Cursive};

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
        type S = SpannedString<Style>;

        let text = match status {
            Status::Connecting => S::plain("connecting to Twitch..."),
            Status::Connected => S::plain("connected!"),
            Status::Ping => S::plain("ping!"),
            Status::Pong => S::plain("pong!"),
            Status::Joining(channel) => S::plain("joining: ").append(channel, crate::Color::TEAL),
            Status::Joined(channel) => S::plain("joined: ").append(channel, crate::Color::TEAL),
            Status::Raw(..) => return, // ignore this
            Status::Information => return App::focus_messages_view(self.cursive()),
        };

        let ts = chrono::Local::now().format("[%c]").to_string();
        let view = TextView::new(text);

        self.on(|inner| inner.get_inner_mut().add_child(&*ts, view))
    }
}

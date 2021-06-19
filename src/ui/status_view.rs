use super::{OnView, SpannedAppender as _};
use crate::App;

use cursive::{
    theme::{Color, Style},
    utils::span::SpannedString,
    views::*,
    Cursive,
};

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

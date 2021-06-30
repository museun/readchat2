use cursive::{
    direction::Orientation,
    theme::Color,
    traits::{Boxable, Nameable},
    utils::span::SpannedString,
    views::*,
    View,
};

use twitchchat::{messages::Privmsg, twitch::BadgeKind};

use crate::{get_config, ui::SpannedAppender, Config};

mod badge;
pub use badge::Badge;

#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub data: String,
    pub badge: Option<Badge>,
    pub ts: chrono::DateTime<chrono::Local>,
    pub color: Color,
}

impl Entry {
    pub(crate) fn as_header_view(entry: &Self) -> impl View {
        let ts = {
            let Config {
                timestamp_fmt,
                colors,
                ..
            } = &*get_config();

            SpannedString::styled(
                entry.ts.format(&format!("{} ", timestamp_fmt)).to_string(),
                colors.timestamp,
            )
        };

        let left = {
            let name = SpannedString::styled(&*entry.name.trim(), entry.color);
            let tv = TextView::new(name).no_wrap().full_width();
            let mut sub = LinearLayout::new(Orientation::Horizontal).child(tv);
            if let Some(badge) = entry.badge {
                let tv = TextView::new(badge.as_spanned_string().append_plain(" ")).no_wrap();
                sub.add_child(HideableView::new(tv).with_name("badge"))
            }
            sub
        };

        let tv = TextView::new(ts).no_wrap();
        let right = HideableView::new(tv).with_name("timestamp");

        LinearLayout::new(Orientation::Horizontal)
            .child(left)
            .child(right)
    }

    pub(crate) fn as_message_view(&self) -> Option<impl View> {
        Some(
            LinearLayout::new(Orientation::Vertical)
                .child(Self::as_header_view(self))
                .child(TextView::new(&*self.data))
                .child(TextView::new("\n")),
        )
    }

    pub(crate) fn as_links_view(&self) -> Option<impl View> {
        if !self.contains_links() {
            return None;
        }

        Some(
            self.find_links()
                .fold(
                    LinearLayout::new(Orientation::Vertical).child(Self::as_header_view(self)),
                    |layout, link| layout.child(TextView::new(&*link).full_width()),
                )
                .child(TextView::new("\n"))
                .full_width(),
        )
    }

    pub(crate) fn contains_links(&self) -> bool {
        self.data
            .split_whitespace()
            .flat_map(url::Url::parse)
            .any(|url| matches!(url.scheme(), "http" | "https"))
    }

    pub(crate) fn as_highlights_view(&self) -> Option<impl View> {
        Option::<DummyView>::None
    }

    pub(crate) fn contains_keywords(&self) -> bool {
        let crate::config::Highlights { keywords, .. } = &get_config().highlights;

        self.data.split_whitespace().any(|s| {
            // keywords
            todo!();
        })
    }

    pub(crate) fn find_links(&self) -> impl Iterator<Item = String> + '_ {
        self.data
            .split_whitespace()
            .flat_map(url::Url::parse)
            .filter(|url| matches!(url.scheme(), "http" | "https"))
            .map(Into::into)
    }
}

#[derive(Default)]
pub struct UserCache {
    map: Vec<(String, i64)>,
}

impl UserCache {
    pub fn insert(&mut self, name: impl Into<String>, id: i64) {
        self.map.push((name.into(), id));
    }

    pub fn contains_id(&self, id: i64) -> bool {
        self.map.iter().any(|&(_, v)| v == id)
    }

    pub fn update_name(&mut self, id: i64, name: impl Into<String>) {
        if let Some((k, _)) = self.map.iter_mut().find(|(_, v)| *v == id) {
            *k = name.into()
        } else {
            self.insert(name, id)
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.map.iter().any(|(k, _)| k.eq_ignore_ascii_case(name))
    }
}

impl<'a> From<Privmsg<'a>> for Entry {
    fn from(pm: Privmsg<'a>) -> Self {
        use twitchchat::twitch::{color::RGB, Color as TwitchColor};
        let conv = |color: TwitchColor| {
            let RGB(r, g, b) = color.rgb;
            Color::Rgb(r, g, b)
        };

        Self {
            name: pm.display_name().unwrap_or_else(|| pm.name()).to_string(),
            data: pm.data().to_string(),
            ts: chrono::Local::now(),
            badge: pm
                .iter_badges()
                .flat_map(|b| Badge::from_badge_kind(&b.kind))
                .max(),
            color: conv(pm.color().unwrap_or_default()),
        }
    }
}

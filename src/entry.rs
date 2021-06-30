use cursive::{
    direction::Orientation,
    theme::Color,
    traits::{Boxable, Nameable},
    utils::span::SpannedString,
    views::*,
    View,
};

use twitchchat::messages::Privmsg;

use crate::{
    config::{Keyword, Style},
    get_config,
    ui::SpannedAppender,
    Config,
};

mod badge;
pub use badge::Badge;

mod user_cache;
use user_cache::UserCache;

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
                entry.ts.format(&format!("{}", timestamp_fmt)).to_string(),
                colors.timestamp,
            )
        };

        let left = {
            let name = SpannedString::styled(&*entry.name.trim(), entry.color);
            let tv = TextView::new(name).no_wrap().full_width();
            let mut sub = LinearLayout::new(Orientation::Horizontal).child(tv);
            if let Some(badge) = entry.badge {
                let tv = TextView::new(badge.as_spanned_string()).no_wrap();
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
        let keywords = &*get_config().highlights.keywords;
        self.as_row_entry(keywords)
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

    pub(crate) fn as_highlights_view(&self) -> Option<impl View> {
        let keywords = &*get_config().highlights.keywords;
        if !self.contains_keywords(keywords) {
            return None;
        }

        self.as_row_entry(keywords)
    }

    fn as_row_entry(&self, keywords: &[Keyword]) -> Option<impl View> {
        Some(
            LinearLayout::new(Orientation::Vertical)
                .child(Self::as_header_view(self))
                .child(TextView::new(self.highlight_keywords(keywords)))
                .child(TextView::new("\n")),
        )
    }
}

impl Entry {
    pub(crate) fn highlight_keywords(
        &self,
        keywords: &[Keyword],
    ) -> SpannedString<cursive::theme::Style> {
        self.find_keywords(keywords).fold(
            SpannedString::<cursive::theme::Style>::new(),
            |mut s, part| {
                if !s.is_empty() {
                    s = s.append_plain(" ");
                }
                match part {
                    Part::Matched(_start, text, style) => s.append(text, style),
                    Part::NotMatched(_start, text) => s.append_plain(text),
                }
            },
        )
    }

    pub(crate) fn contains_links(&self) -> bool {
        self.data
            .split_whitespace()
            .flat_map(url::Url::parse)
            .any(|url| matches!(url.scheme(), "http" | "https"))
    }

    pub(crate) fn find_links(&self) -> impl Iterator<Item = String> + '_ {
        self.data
            .split_whitespace()
            .flat_map(url::Url::parse)
            .filter(|url| matches!(url.scheme(), "http" | "https"))
            .map(Into::into)
    }

    pub(crate) fn find_keywords<'a: 'b, 'b>(
        &'b self,
        keywords: &'a [Keyword],
    ) -> impl Iterator<Item = Part<'b>> + 'b {
        index_split_iter(&self.data)
            .map(move |(i, s)| {
                keywords
                    .iter()
                    .find_map(|kw| (kw == s).then(|| (i, s, kw.style)))
                    .map(|(i, n, s)| Part::Matched(i, n, s))
                    .or_else(|| Some(Part::NotMatched(i, s)))
            })
            .flatten()
    }

    pub(crate) fn contains_keywords(&self, keywords: &[Keyword]) -> bool {
        self.data
            .split_whitespace()
            .any(|s| keywords.iter().any(|kw| kw == s))
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

pub(crate) enum Part<'a> {
    Matched(usize, &'a str, Style),
    NotMatched(usize, &'a str),
}

fn index_split_iter(input: &str) -> impl Iterator<Item = (usize, &str)> + '_ {
    use std::{cell::RefCell, rc::Rc};
    let pos = Rc::new(RefCell::new(0_usize));
    let mut iter = input
        .char_indices()
        .filter_map(|(i, e)| e.is_whitespace().then(|| i));

    std::iter::from_fn({
        let pos = Rc::clone(&pos);
        move || {
            let (start, end) = (*pos.borrow(), iter.next()?);
            *pos.borrow_mut() = end + 1;
            Some((start, &input[start..end]))
        }
    })
    .chain(
        std::iter::once_with({
            let pos = Rc::clone(&pos);
            move || {
                let pos = *pos.borrow();
                input
                    .get(pos..)
                    .map(|c| (pos, c.chars().take_while(|c| !c.is_whitespace()).count()))
                    .map(|(start, end)| (start, &input[start..start + end]))
            }
        })
        .flatten(),
    )
}

fn index_split(input: &str) -> Vec<(usize, &str)> {
    let mut data = vec![];
    let mut pos = 0;
    for index in input
        .char_indices()
        .filter_map(|(i, e)| e.is_whitespace().then(|| i))
    {
        data.push((pos, &input[pos..index]));
        pos = index + 1;
    }
    if let Some(end) = input
        .get(pos..)
        .map(|c| c.chars().take_while(|c| !c.is_whitespace()).count())
    {
        data.push((pos, &input[pos..pos + end]))
    }
    data
}

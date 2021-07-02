use cursive::{
    direction::Orientation,
    theme::Color,
    traits::{Boxable, Nameable},
    utils::span::SpannedString,
    view::Margins,
    views::*,
    View,
};

use twitchchat::messages::Privmsg;

use crate::{
    config::{Highlights, Keyword, Style},
    get_config,
    ui::SpannedAppender,
    Config,
};

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
                entry.ts.format(&format!("{}", timestamp_fmt)).to_string(),
                colors.timestamp,
            )
        };

        let left = {
            let name = SpannedString::styled(&*entry.name.trim(), entry.color);
            let tv = TextView::new(name).no_wrap().full_width();
            let mut sub = LinearLayout::new(Orientation::Horizontal).child(tv);
            if let Some(badge) = entry.badge {
                let tv = PaddedView::new(
                    Margins::lr(0, 1),
                    TextView::new(badge.as_spanned_string()).no_wrap(),
                );
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
        let Highlights { mention, keywords } = &get_config().highlights;
        self.as_row_entry(&**keywords, *mention)
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
        let Highlights { mention, keywords } = &get_config().highlights;
        let name = &*crate::state::get_channel();

        if !self.contains_keywords(keywords) && !self.contains_mention(name) {
            return None;
        }

        self.as_row_entry(&**keywords, *mention)
    }

    fn as_row_entry(&self, keywords: &[Keyword], style: Style) -> Option<impl View> {
        let name = &*crate::state::get_channel();
        Some(
            LinearLayout::new(Orientation::Vertical)
                .child(Self::as_header_view(self))
                .child(TextView::new(self.highlight(keywords, name, style)))
                .child(TextView::new("\n")),
        )
    }
}

impl Entry {
    pub(crate) fn highlight(
        &self,
        keywords: &[Keyword],
        name: &str,
        style: Style,
    ) -> SpannedString<cursive::theme::Style> {
        let mut string = self.find_keywords(keywords).fold(
            SpannedString::<cursive::theme::Style>::new(),
            |mut s, part| {
                if !s.is_empty() {
                    s = s.append_plain(" ");
                }
                match part {
                    Part::Matched(text, style) => s.append(text, style),
                    Part::NotMatched(text) => s.append_plain(text),
                }
            },
        );

        for span in string.spans_attr_mut() {
            if trim_punc(span.content).eq_ignore_ascii_case(name) {
                *span.attr = span.attr.combine(style);
            }
        }

        string
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
        self.data
            .split_ascii_whitespace()
            .map(move |s| {
                let trimmed = trim_punc(s);
                keywords
                    .iter()
                    .find_map(|kw| (kw == trimmed).then(|| (trimmed, kw.style)))
                    .map(|(n, s)| Part::Matched(n, s))
                    .or_else(|| Some(Part::NotMatched(s)))
            })
            .flatten()
    }

    pub(crate) fn contains_mention(&self, name: &str) -> bool {
        self.data
            .split_whitespace()
            .any(|s| trim_punc(s).eq_ignore_ascii_case(name))
    }

    pub(crate) fn contains_keywords(&self, keywords: &[Keyword]) -> bool {
        self.data
            .split_whitespace()
            .any(|s| keywords.iter().any(|kw| kw == s))
    }
}

fn trim_punc(mut input: &str) -> &str {
    while input.starts_with('@') {
        input = &input[1..]
    }
    while input.ends_with(|c: char| c.is_ascii_punctuation()) {
        input = &input[..input.len() - 1]
    }
    input
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
    Matched(&'a str, Style),
    NotMatched(&'a str),
}

use cursive::{
    direction::Orientation,
    theme::Color,
    traits::{Boxable, Nameable},
    utils::span::SpannedString,
    views::*,
    View,
};

use twitchchat::{messages::Privmsg, twitch::BadgeKind};

use crate::{ui::SpannedAppender, Config, ConfigBadges};

/// NOTE: this must remain in this order for Iterator::max to work
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Badge {
    Partner,
    VIP,
    Premium,
    Bits,
    Turbo,
    Subscriber,
    Moderator,
    Broadcaster,
    GlobalMod,
    Staff,
    Admin,
}

impl Badge {
    fn as_spanned_string(&self, config: &Config) -> SpannedString<cursive::theme::Style> {
        let ConfigBadges {
            partner,
            vip,
            premium,
            bits,
            turbo,
            subscriber,
            moderator,
            broadcaster,
            global_mod,
            staff,
            admin,
        } = config.colors.badges;

        match self {
            Self::Partner => SpannedString::styled("partner", partner),
            Self::VIP => SpannedString::styled("vip", vip),
            Self::Premium => SpannedString::styled("premium", premium),
            Self::Bits => SpannedString::styled("bits", bits),
            Self::Turbo => SpannedString::styled("turbo", turbo),
            Self::Subscriber => SpannedString::styled("sub", subscriber),
            Self::Moderator => SpannedString::styled("mod", moderator),
            Self::Broadcaster => SpannedString::styled("broadcaster", broadcaster),
            Self::GlobalMod => SpannedString::styled("global_mod", global_mod),
            Self::Staff => SpannedString::styled("staff", staff),
            Self::Admin => SpannedString::styled("admin", admin),
        }
    }

    const fn from_badge_kind(bk: &BadgeKind<'_>) -> Option<Self> {
        let badge = match bk {
            BadgeKind::Admin => Self::Admin,
            BadgeKind::Bits => Self::Bits,
            BadgeKind::Broadcaster => Self::Broadcaster,
            BadgeKind::GlobalMod => Self::GlobalMod,
            BadgeKind::Moderator => Self::Moderator,
            BadgeKind::Subscriber => Self::Subscriber,
            BadgeKind::Staff => Self::Staff,
            BadgeKind::Turbo => Self::Turbo,
            BadgeKind::Premium => Self::Premium,
            BadgeKind::VIP => Self::VIP,
            BadgeKind::Partner => Self::Partner,
            _ => return None,
        };
        Some(badge)
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub data: String,
    pub badge: Option<Badge>,
    pub ts: chrono::DateTime<chrono::Local>,
    pub color: Color,
}

impl Entry {
    pub(crate) fn as_header_view(entry: &Self, config: &Config) -> impl View {
        let ts = SpannedString::styled(
            // XXX: cursive is still calculating the width of the scrollbar even
            // if its been disabled, so instead of forking it to fix that
            // padding issue, we'll just adding our own padding so the scrollbar
            // can still be rendered and not resize the elements in the
            // LinearView
            entry.ts.format("%X ").to_string(),
            config.colors.timestamp,
        );

        let left = {
            let mut sub = LinearLayout::new(Orientation::Horizontal).child(
                TextView::new(SpannedString::styled(&*entry.name.trim(), entry.color))
                    .no_wrap()
                    .full_width(),
            );

            if let Some(badge) = entry.badge {
                sub.add_child(
                    HideableView::new(TextView::new(
                        badge.as_spanned_string(config).append_plain(" "),
                    ))
                    .with_name("badge"),
                )
            }

            sub
        };

        let right = HideableView::new(TextView::new(ts).no_wrap()).with_name("timestamp");

        LinearLayout::new(Orientation::Horizontal)
            .child(left)
            .child(right)
    }

    pub(crate) fn as_message_view(&self, app_state: &Config) -> Option<impl View> {
        Some(
            LinearLayout::new(Orientation::Vertical)
                .child(Self::as_header_view(self, app_state))
                .child(TextView::new(&*self.data))
                .child(TextView::new("\n")),
        )
    }

    pub(crate) fn as_links_view(&self, app_state: &Config) -> Option<impl View> {
        if !self.contains_links() {
            return None;
        }

        Some(
            self.find_links()
                .fold(
                    LinearLayout::new(Orientation::Vertical)
                        .child(Self::as_header_view(self, app_state)),
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

    pub(crate) fn find_links(&self) -> impl Iterator<Item = String> + '_ {
        self.data
            .split_whitespace()
            .flat_map(url::Url::parse)
            .filter(|url| matches!(url.scheme(), "http" | "https"))
            .map(Into::into)
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

use cursive::{theme::Style, utils::span::SpannedString};
use twitchchat::twitch::BadgeKind;

use crate::get_config;

// NOTE: this must remain in this order for Iterator::max to work
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Badge {
    Partner,
    Vip,
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
    pub(crate) fn as_spanned_string(&self) -> SpannedString<Style> {
        let config = get_config();

        let badges = &config.colors.badges;
        let mapping = &config.badge_names;

        match self {
            Self::Partner => SpannedString::styled(&mapping.partner, badges.partner),
            Self::Vip => SpannedString::styled(&mapping.vip, badges.vip),
            Self::Premium => SpannedString::styled(&mapping.premium, badges.premium),
            Self::Bits => SpannedString::styled(&mapping.bits, badges.bits),
            Self::Turbo => SpannedString::styled(&mapping.turbo, badges.turbo),
            Self::Subscriber => SpannedString::styled(&mapping.subscriber, badges.subscriber),
            Self::Moderator => SpannedString::styled(&mapping.moderator, badges.moderator),
            Self::Broadcaster => SpannedString::styled(&mapping.broadcaster, badges.broadcaster),
            Self::GlobalMod => SpannedString::styled(&mapping.global_mod, badges.global_mod),
            Self::Staff => SpannedString::styled(&mapping.staff, badges.staff),
            Self::Admin => SpannedString::styled(&mapping.admin, badges.admin),
        }
    }

    pub(crate) const fn from_badge_kind(bk: &BadgeKind<'_>) -> Option<Self> {
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
            BadgeKind::VIP => Self::Vip,
            BadgeKind::Partner => Self::Partner,
            _ => return None,
        };
        Some(badge)
    }
}

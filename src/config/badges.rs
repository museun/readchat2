use super::{Color, Effects, Style};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Badges {
    pub admin: Style,
    pub bits: Style,
    pub broadcaster: Style,
    pub global_mod: Style,
    pub moderator: Style,
    pub partner: Style,
    pub premium: Style,
    pub staff: Style,
    pub subscriber: Style,
    pub turbo: Style,
    pub vip: Style,
}

impl Default for Badges {
    fn default() -> Self {
        let (admin, bits, global_mod, partner, premium, staff, moderator, turbo, vip) =
            <_>::default();

        Self {
            broadcaster: Style::fg(Color::RED).with_effects(Effects::bold()),
            subscriber: Style::fg(Color::RED),
            admin,
            bits,
            global_mod,
            partner,
            premium,
            staff,
            moderator,
            turbo,
            vip,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BadgeNameMapping {
    pub admin: String,
    pub bits: String,
    pub broadcaster: String,
    pub global_mod: String,
    pub moderator: String,
    pub partner: String,
    pub premium: String,
    pub staff: String,
    pub subscriber: String,
    pub turbo: String,
    pub vip: String,
}

impl Default for BadgeNameMapping {
    fn default() -> Self {
        Self {
            admin: "admin".to_string(),
            bits: "bits".to_string(),
            broadcaster: "broadcaster".to_string(),
            global_mod: "global_mod".to_string(),
            moderator: "moderator".to_string(),
            partner: "partner".to_string(),
            premium: "premium".to_string(),
            staff: "staff".to_string(),
            subscriber: "subscriber".to_string(),
            turbo: "turbo".to_string(),
            vip: "vip".to_string(),
        }
    }
}

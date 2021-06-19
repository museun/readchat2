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

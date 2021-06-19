use anyhow::Context as _;

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Default for Color {
    fn default() -> Self {
        Self::default_fg()
    }
}

impl Color {
    pub(crate) const RED: Self = Self(0xFF, 0x00, 0x00);
    pub(crate) const WHITE: Self = Self(0xFF, 0xFF, 0xFF);
    pub(crate) const MAGENTA: Self = Self(0xFF, 0x00, 0xFF);
    pub(crate) const RUSTACEAN_ORANGE: Self = Self(0xf5, 0x7c, 0x00);
    pub(crate) const TWITCH_PURPLE: Self = Self(0x91, 0x46, 0xff);

    pub(crate) const fn default_fg() -> Self {
        Self::WHITE
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{r:02X}{g:02X}{b:02X}", r = r, g = g, b = b)
    }
}

impl std::str::FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        let s = match s.len() {
            7 if s.starts_with('#') => &s[1..],
            6 if s.chars().all(|c| c.is_ascii_hexdigit()) => s,
            _ => anyhow::bail!("invalid hex string"),
        };

        u32::from_str_radix(s, 16)
            .map(|s| {
                Self(
                    ((s >> 16) & 0xFF) as _,
                    ((s >> 8) & 0xFF) as _,
                    (s & 0xFF) as _,
                )
            })
            .with_context(|| "cannot parse hex string")
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let Self(r, g, b) = self;
        ser.collect_str(&format_args!("#{r:02X}{g:02X}{b:02X}", r = r, g = g, b = b))
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <std::borrow::Cow<'_, str>>::deserialize(de)?
            .parse()
            .map_err(|_| serde::de::Error::custom("invalid hex string"))
    }
}

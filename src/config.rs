use anyhow::Context as _;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub timestamps: bool,
    pub badges: bool,
    pub timestamp_fmt: String,
    pub colors: Colors,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channel: None,
            timestamps: true,
            badges: true,
            timestamp_fmt: "%X".into(),
            colors: Colors::default(),
        }
    }
}

impl Config {
    pub fn from_toml(data: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        toml::from_slice(data.as_ref()).with_context(|| anyhow::anyhow!("cannot parse config"))
    }

    pub fn config_path() -> anyhow::Result<std::path::PathBuf> {
        Self::config_dir().map(|p| p.join("config.toml"))
    }

    pub fn config_dir() -> anyhow::Result<std::path::PathBuf> {
        dirs::config_dir()
            .map(|f| f.join("museun").join("readchat"))
            .with_context(|| anyhow::anyhow!("system does not have a configuration directory"))
    }

    pub const fn default_config() -> &'static str {
        r##"
channel       = "#museun"
timestamps    = true
badges        = true
timestamp_fmt = "%X"

# syntax  { fg = "#hex"   , bg = "#hex" , bold = bool }
# default { fg = "#FFFFFF", bg = <unset>, bold = false }

[colors.timestamp]
fg = "#FF00FF"

[colors.badges]
admin       = { fg = "#FFFFFF" }
bits        = { fg = "#FFFFFF" }
broadcaster = { fg = "#FF0000", bold = true }
global_mod  = { fg = "#FFFFFF" }
moderator   = { fg = "#FFC0CB" }
partner     = { fg = "#FFFFFF" }
premium     = { fg = "#FFFFFF" }
staff       = { fg = "#FFFFFF" }
subscriber  = { fg = "#FF0000" }
turbo       = { fg = "#FFFFFF" }
vip         = { fg = "#FFFFFF" }
        "##
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Colors {
    pub timestamp: Style,
    pub badges: Badges,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            timestamp: Style {
                fg: Color(0xFF, 0x00, 0xFF),
                bg: None,
                bold: false,
            },
            badges: Badges::default(),
        }
    }
}

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
            broadcaster: Style {
                fg: Color(0xFF, 0x00, 0x00),
                bg: None,
                bold: true,
            },
            subscriber: Style {
                fg: Color(0xFF, 0x00, 0x00),
                bg: None,
                bold: false,
            },
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

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Style {
    #[serde(default = "Color::default_fg")]
    pub fg: Color,
    #[serde(default)]
    pub bg: Option<Color>,
    #[serde(default)]
    pub bold: bool,
}

impl From<Style> for cursive::theme::Style {
    fn from(style: Style) -> Self {
        let Color(r, g, b) = style.fg;
        let front = cursive::theme::Color::Rgb(r, g, b).into();

        let back = style
            .bg
            .map(|Color(r, g, b)| cursive::theme::Color::Rgb(r, g, b))
            .unwrap_or(cursive::theme::Color::TerminalDefault)
            .into();

        let effects = if style.bold {
            cursive::theme::Effect::Bold
        } else {
            cursive::theme::Effect::Simple
        }
        .into();

        cursive::theme::Style {
            color: cursive::theme::ColorStyle { front, back },
            effects,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color(0xFF, 0xFF, 0xFF),
            bg: None,
            bold: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);
impl Color {
    const fn default_fg() -> Color {
        Color(0xFF, 0xFF, 0xFF)
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

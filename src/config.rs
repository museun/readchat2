use anyhow::Context as _;
use cursive::theme::Effect;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub timestamps: bool,
    pub badges: bool,
    pub timestamp_fmt: String,
    pub tabs: TabsConfig,
    pub colors: Colors,
    pub highlights: Highlights,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channel: None,
            timestamps: true,
            badges: true,
            timestamp_fmt: "%X".into(),
            tabs: TabsConfig::default(),
            colors: Colors::default(),
            highlights: Highlights::default(),
        }
    }
}

impl Config {
    pub fn from_yaml(data: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        serde_yaml::from_slice(data.as_ref())
            .with_context(|| anyhow::anyhow!("cannot parse config"))
    }

    pub fn config_path() -> anyhow::Result<std::path::PathBuf> {
        Self::config_dir().map(|p| p.join("config.yaml"))
    }

    pub fn config_dir() -> anyhow::Result<std::path::PathBuf> {
        dirs::config_dir()
            .map(|f| f.join("museun").join("readchat2"))
            .with_context(|| anyhow::anyhow!("system does not have a configuration directory"))
    }

    pub const fn default_config() -> &'static str {
        r##"
# syntax  { fg = "#hex"   , bg = "#hex" , bold = bool }
# default { fg = "#FFFFFF", bg = <unset>, bold = false }

channel: ~
timestamps: true
badges: true
timestamp_fmt: "%X"

tabs:
  active:
    fg: "#FF0000"
    bg: ~
    effects: ~
  inactive:
    fg: "#FFFFFF"
    bg: ~
    effects: ~

colors:
  timestamp:
    fg: "#FF00FF"
    bg: ~
    effects: ~
  badges:
    admin:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    bits:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    broadcaster:
      fg: "#FF0000"
      bg: ~
      effects: bold
    global_mod:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    moderator:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    partner:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    premium:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    staff:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    subscriber:
      fg: "#FF0000"
      bg: ~
      effects: ~
    turbo:
      fg: "#FFFFFF"
      bg: ~
      effects: ~
    vip:
      fg: "#FFFFFF"
      bg: ~
      effects: ~

highlights:
  mentions:
    fg: "#FF0000"
    bg: ~
    effects: bold
  keywords:
    - name: Rust
      case_sensitive: false
      style:
        fg: "#F57C00"
        bg: ~
        effects: bold
    - name: Twitch
      case_sensitive: true
      style:
        fg: "#9146FF"
        bg: ~
        effects: bold
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
            timestamp: Style::fg(Color(0xFF, 0x00, 0xFF)),
            badges: Badges::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TabsConfig {
    pub active: Style,
    pub inactive: Style,
}

impl Default for TabsConfig {
    fn default() -> Self {
        Self {
            active: Style::fg(Color(0xFF, 0x00, 0x00)),
            inactive: Style::fg(Color(0xFF, 0xFF, 0xFF)),
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
        const RED: Color = Color(0xFF, 0x00, 0x00);

        let (admin, bits, global_mod, partner, premium, staff, moderator, turbo, vip) =
            <_>::default();

        Self {
            broadcaster: Style::fg(RED).with_effects(Effects::bold()),
            subscriber: Style::fg(RED),
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

// TODO support effects
#[derive(Copy, Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Style {
    #[serde(default = "Color::default_fg")]
    pub fg: Color,
    #[serde(default)]
    pub bg: Option<Color>,
    #[serde(default)]
    pub effects: Effects,
}

impl Style {
    pub fn fg(color: Color) -> Self {
        Self {
            fg: color,
            ..Self::default()
        }
    }
    pub const fn with_effects(self, effects: Effects) -> Self {
        Self { effects, ..self }
    }
}

macro_rules! make_effects {
    ($($id:ident)*) => {
        #[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
        pub struct Effects {
            $(
                pub $id: bool,
            )*
        }

        impl Effects {
            pub const fn empty() -> Self {
                Self {
                    $($id: false,)*
                }
            }

            $(
                pub const fn $id() -> Self {
                    let mut e = Self::empty();
                    e.$id = true;
                    e
                }
            )*

            pub fn into_iter(self) -> impl Iterator<Item = (&'static str, bool)> + DoubleEndedIterator + ExactSizeIterator {
                <_>::into_iter([$(stringify!($id),)*]).zip([$(self.$id,)*])
            }
        }
    };
}

make_effects! {
    simple
    reverse
    bold
    italic
    strikethrough
    underline
    blink
}

impl std::ops::BitOr<Effect> for Effects {
    type Output = Self;

    fn bitor(mut self, rhs: Effect) -> Self::Output {
        use cursive::theme::Effect::*;

        *match rhs {
            Simple => &mut self.simple,
            Reverse => &mut self.reverse,
            Bold => &mut self.bold,
            Italic => &mut self.italic,
            Strikethrough => &mut self.strikethrough,
            Underline => &mut self.underline,
            Blink => &mut self.blink,
        } |= true;

        self
    }
}

impl<'de> serde::Deserialize<'de> for Effects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Effects;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.split('|')
                    .map(str::trim)
                    .try_fold(Effects::default(), |mut eff, key| {
                        *match key {
                            "simple" => &mut eff.simple,
                            "reverse" => &mut eff.reverse,
                            "bold" => &mut eff.bold,
                            "italic" => &mut eff.italic,
                            "strikethrough" => &mut eff.strikethrough,
                            "underline" => &mut eff.underline,
                            "blink" => &mut eff.blink,
                            e => return Err(E::custom(&format!("unknown effect: {}", e))),
                        } = true;
                        Ok(eff)
                    })
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl serde::Serialize for Effects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s =
            self.into_iter()
                .filter_map(|(k, v)| v.then(|| k))
                .fold(String::new(), |mut a, c| {
                    if !a.is_empty() {
                        a.push_str(" | ");
                    }
                    a.push_str(c.as_ref());
                    a
                });

        if s.is_empty() {
            serializer.serialize_none()
        } else {
            serializer.serialize_some(&s)
        }
    }
}

impl From<Style> for cursive::theme::Style {
    fn from(style: Style) -> Self {
        use cursive::theme::{Color as CursiveColor, ColorStyle};

        let Color(r, g, b) = style.fg;
        let front = CursiveColor::Rgb(r, g, b).into();

        let back = style
            .bg
            .map(|Color(r, g, b)| CursiveColor::Rgb(r, g, b))
            .unwrap_or(CursiveColor::TerminalDefault)
            .into();

        Self {
            color: ColorStyle { front, back },
            ..style.effects.into()
        }
    }
}

impl From<Effects> for cursive::theme::Style {
    fn from(effects: Effects) -> Self {
        Self {
            effects: effects
                .into_iter()
                .zip([
                    Effect::Simple,
                    Effect::Reverse,
                    Effect::Bold,
                    Effect::Italic,
                    Effect::Strikethrough,
                    Effect::Underline,
                    Effect::Blink,
                ])
                .fold(Default::default(), |eff, ((_, v), e)| {
                    v.then(|| eff | e).unwrap_or_else(|| eff & e)
                }),
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Default for Color {
    fn default() -> Self {
        Self::default_fg()
    }
}

impl Color {
    const fn default_fg() -> Self {
        Self(0xFF, 0xFF, 0xFF)
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Highlights {
    pub mentions: Style,
    pub keywords: Vec<Keyword>,
}

impl Default for Highlights {
    fn default() -> Self {
        let keywords = std::array::IntoIter::new([
            ("Rust", false, Color(0xf5, 0x7c, 0x00), true),
            ("Twitch", true, Color(0x91, 0x46, 0xff), false),
        ])
        .map(|(n, case_sensitive, fg, bold)| Keyword {
            name: n.to_string(),
            case_sensitive,
            style: Style::fg(fg).with_effects(Effects::bold()),
        })
        .collect();

        Self {
            mentions: Style::fg(Color(0xFF, 0x00, 0x00)).with_effects(Effects::bold()),
            keywords,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Keyword {
    pub name: String,
    pub case_sensitive: bool,
    pub style: Style,
}

#[test]
fn asdf() {
    // eprintln!("{}", toml::to_string(&Config::default()).unwrap());
    println!("{}", serde_yaml::to_string(&Config::default()).unwrap());
}

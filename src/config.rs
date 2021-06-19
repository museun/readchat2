use anyhow::Context as _;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub timestamps: bool,
    pub badges: bool,
    pub timestamp_fmt: String,
    pub tabs: Tabs,
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
            tabs: Tabs::default(),
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

mod colors;
pub use colors::Colors;

mod tabs;
pub use tabs::Tabs;

mod badges;
pub use badges::Badges;

mod style;
pub use style::Style;

mod effects;
pub use effects::Effects;

mod color;
pub use color::Color;

mod highlights;
pub use highlights::{Highlights, Keyword};

use std::path::PathBuf;

use anyhow::Context as _;

mod colors;
pub use colors::Colors;

mod tabs;
pub use tabs::{TabNameMapping, Tabs};

mod badges;
pub use badges::{BadgeNameMapping, Badges};

mod style;
pub use style::Style;

mod effects;
pub use effects::Effects;

mod color;
pub use color::Color;

mod highlights;
pub use highlights::{Highlights, Keyword};

mod keybinds;
pub use keybinds::{Action, Input, KeyBinds};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub timestamps: bool,
    pub badges: bool,
    pub badge_names: BadgeNameMapping,
    pub timestamp_fmt: String,
    pub tabs: Tabs,
    pub tab_names: TabNameMapping,
    pub colors: Colors,
    pub highlights: Highlights,
    pub keybinds: KeyBinds,
}

impl Default for Config {
    // TODO this should maybe default from the file to ensure they are in sync
    fn default() -> Self {
        let (channel, tabs, tab_names, badge_names, colors, highlights, keybinds) = <_>::default();

        Self {
            timestamps: true,
            badges: true,
            timestamp_fmt: "%X".into(),

            badge_names,
            channel,
            tabs,
            tab_names,
            colors,
            highlights,
            keybinds,
        }
    }
}

impl Config {
    pub fn from_yaml(data: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        serde_yaml::from_slice(data.as_ref())
            .with_context(|| anyhow::anyhow!("cannot parse config"))
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        Self::config_dir().map(|p| p.join("config.yaml"))
    }

    pub fn config_dir() -> anyhow::Result<PathBuf> {
        dirs::config_dir()
            .map(|f| f.join("museun").join("readchat2"))
            .with_context(|| anyhow::anyhow!("system does not have a configuration directory"))
    }

    pub fn data_dir() -> anyhow::Result<PathBuf> {
        dirs::data_dir()
            .map(|f| f.join("museun").join("readchat2"))
            .with_context(|| anyhow::anyhow!("system does not have a configuration directory"))
    }

    pub const fn default_config() -> &'static str {
        include_str!("./config/default.yaml")
    }
}

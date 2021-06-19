use anyhow::Context as _;

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
    // TODO this should maybe default from the file to ensure they are in sync
    fn default() -> Self {
        let (channel, tabs, colors, highlights) = <_>::default();

        Self {
            timestamps: true,
            badges: true,
            timestamp_fmt: "%X".into(),

            channel,
            tabs,
            colors,
            highlights,
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
        include_str!("./config/default.yaml")
    }
}

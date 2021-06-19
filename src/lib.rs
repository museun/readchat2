#![cfg_attr(any(debug_assertions, test), allow(dead_code, unused_variables,))]

pub mod colors;

pub mod panic_logger;
pub mod twitch;

mod app;
pub use app::App;

mod ui;
pub use ui::{build_ui, MessagesView, OnView};

mod state;
pub use state::{get_config, CONFIG};

mod config;
pub use config::{
    Badges as ConfigBadges, Color as ConfigColor, Config, Highlights, Style as ConfigStyle, Tabs,
};

mod entry;

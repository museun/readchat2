#![cfg_attr(debug_assertions, allow(dead_code,))]

pub mod colors;

pub mod panic_logger;
pub mod twitch;

mod app;
pub use app::App;

pub mod ui;
pub use ui::{build_ui, OnView};

mod state;
pub use state::{get_config, CONFIG};

pub(crate) mod config;
pub use config::Config;

mod entry;

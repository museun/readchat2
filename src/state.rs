use crate::Config;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub static CONFIG: OnceCell<Arc<RwLock<Config>>> = OnceCell::new();

pub fn get_config_mut() -> RwLockWriteGuard<'static, Config> {
    CONFIG
        .get()
        .expect("config must be initialized")
        .write()
        .unwrap()
}

pub fn get_config() -> RwLockReadGuard<'static, Config> {
    CONFIG
        .get()
        .expect("config must be initialized")
        .read()
        .unwrap()
}

pub static CONNECTED_CHANNEL: OnceCell<Arc<str>> = OnceCell::new();

pub fn get_channel() -> Arc<str> {
    CONNECTED_CHANNEL
        .get()
        .map(Arc::clone)
        .expect("channel must be initialized")
}

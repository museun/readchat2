use crate::Config;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex, MutexGuard};

pub static CONFIG: OnceCell<Arc<Mutex<Config>>> = OnceCell::new();

pub fn get_config() -> MutexGuard<'static, Config> {
    CONFIG
        .get()
        .expect("config must be initialized")
        .lock()
        .unwrap()
}

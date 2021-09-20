use std::env;

use log::Level;

pub const LOGGER_NAME: &str = "server";

pub fn init() {
    env::set_var("RUST_LOG", LOGGER_NAME);
    env_logger::init();
}

pub fn write_log(level: Level, content: String) {
    log::log!(target: LOGGER_NAME, level, "{}", content);
}

pub fn info(content: String) {
    write_log(Level::Info, content);
}

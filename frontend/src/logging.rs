use log::{Level, LevelFilter, Metadata, Record};
use wasm_bindgen::prelude::*;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_style = match record.level() {
                Level::Error => "color: red; font-weight: bold",
                Level::Warn => "color: orange; font-weight: bold",
                Level::Info => "color: blue",
                Level::Debug => "color: gray",
                Level::Trace => "color: lightgray",
            };

            let message = format!(
                "%c[{}] {} - {}",
                record.level(),
                record.target(),
                record.args()
            );

            match record.level() {
                Level::Error => web_sys::console::error_2(
                    &message.into(),
                    &format!("{}", level_style).into(),
                ),
                Level::Warn => web_sys::console::warn_2(
                    &message.into(),
                    &format!("{}", level_style).into(),
                ),
                _ => web_sys::console::log_2(
                    &message.into(),
                    &format!("{}", level_style).into(),
                ),
            }
        }
    }

    fn flush(&self) {}
}

#[wasm_bindgen(start)]
pub fn init_logger() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(LevelFilter::Debug);
} 
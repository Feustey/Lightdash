use log::{Level, Log, Metadata, Record};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("[{}] {}", record.level(), record.args());
            match record.level() {
                Level::Error => {
                    console::error_1(&message.into());
                }
                Level::Warn => {
                    console::warn_1(&message.into());
                }
                Level::Info => {
                    console::info_1(&message.into());
                }
                Level::Debug => {
                    console::log_1(&message.into());
                }
                Level::Trace => {
                    console::debug_1(&message.into());
                }
            }
        }
    }

    fn flush(&self) {}
}

#[wasm_bindgen(start)]
pub fn init_logger() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Logger initialisé avec succès");
}

// Macros utilitaires pour le logging
#[macro_export]
macro_rules! log_api_call {
    ($method:expr, $url:expr) => {
        log::debug!("Appel API {} vers {}", $method, $url);
    };
}

#[macro_export]
macro_rules! log_api_response {
    ($status:expr, $url:expr) => {
        log::debug!("Réponse API {} de {}", $status, $url);
    };
}

#[macro_export]
macro_rules! log_error {
    ($message:expr, $error:expr) => {
        log::error!("{}: {}", $message, $error);
    };
}

#[macro_export]
macro_rules! log_performance {
    ($operation:expr, $duration:expr) => {
        log::info!("Performance - {} : {}ms", $operation, $duration.as_millis());
    };
} 
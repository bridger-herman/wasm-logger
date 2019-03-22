//! A logger that prints all messages with a readable output format.

extern crate log;
extern crate wasm_bindgen;

use log::{Level, Log, Metadata, Record, SetLoggerError};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

struct SimpleLogger {
    level: Level,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log(&format!(
                "{:<5} [{}] {}",
                record.level().to_string(),
                record.module_path().unwrap_or_default(),
                record.args()
            ));
        }
    }

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{:<5} [{}] {}",
                record.level().to_string(),
                record.module_path().unwrap_or_default(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

/// Initializes the global logger with a SimpleLogger instance with
/// `max_log_level` set to a specific log level.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate simple_logger;
/// #
/// # fn main() {
/// simple_logger::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = SimpleLogger { level };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes the global logger with a SimpleLogger instance with
/// `max_log_level` set to `LogLevel::Trace`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate simple_logger;
/// #
/// # fn main() {
/// simple_logger::init().unwrap();
/// warn!("This is an example message.");
/// # }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}

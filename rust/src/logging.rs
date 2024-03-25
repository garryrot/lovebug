use std::{
    fs::File,
    sync::Mutex,
};

use tracing::{debug, error, info, Level};

use crate::settings::ModSettings;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn lb_init_logging(logPath: String) -> bool;
        fn lb_init_logging_stdout() -> bool;
        fn lb_log_debug(message: String);
        fn lb_log_info(message: String);
        fn lb_log_error(message: String);
    }
}

pub fn lb_log_debug(message: String) {
    debug!(message);
}

pub fn lb_log_info(message: String) {
    info!(message);
}

pub fn lb_log_error(message: String) {
    error!(message);
}

pub fn lb_init_logging_stdout() -> bool {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(false)
        .with_thread_ids(true)
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Setting global tracing subscriber failed.");
        return false;
    }
    true
}

pub fn lb_init_logging(file_path: String) -> bool {
    let settings = ModSettings::try_read_or_default();
    let file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Couldn't write to log file, no logs available: {:?}", err);
            return false;
        }
    };
    let log_lvl : Level = settings.log_level.into();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(log_lvl)
        .with_ansi(false)
        .with_writer(Mutex::new(file))
        .with_thread_ids(true)
        .finish();
    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Setting global tracing subscriber failed.");
        return false;
    }
    true
}

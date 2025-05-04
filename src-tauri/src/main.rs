// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::logger::setup_logger;

mod config;
fn main() {
    // app_lib::run();
    if let Err(e) = setup_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }
    log::info!("Logger initialized successfully.");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use server::app_state::{get_database, get_tally};

use crate::config::logger::setup_logger;

mod config;
mod server;
fn main() {
    // app_lib::run();
    if let Err(e) = setup_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }
    log::info!("Logger initialized successfully.");

    let args = config::cli::parse_args();
    let mut tally_instance = get_tally();
    let mut database_instance = get_database();

    database_instance.update_command_line_config(args.clone());
    tally_instance.update_command_line_config(args.clone());

    tauri::async_runtime::block_on(async {
        tally_instance.import_data(database_instance).await;
    });
}

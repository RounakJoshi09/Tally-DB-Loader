// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use server::{database, tally};

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

    let mut database = database::database_init();
    let mut tally = tally::tally_init();

    database.update_command_line_config(args.clone());
    tally.update_command_line_config(args.clone());

    tally.import_data();
}

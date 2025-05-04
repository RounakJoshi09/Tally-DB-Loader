use fern::Dispatch;
use log::{debug, error, info, warn};
use std::{env, fs};
use std::io;
use std::path::Path;
use dotenv::dotenv;
pub fn setup_logger() -> Result<(), fern::InitError> {
    dotenv().ok();

    let log_file_path = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "log/app.log".to_string());

    if let Some(parent) = Path::new(&log_file_path).parent() {
        fs::create_dir_all(parent).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to create log directory: {}", e))
        })?;
    }

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info) // Set default log level
        .chain(io::stdout()) // Log to console
        .chain(fern::log_file(log_file_path)?) // Log to file
        .apply()?; // Apply the logger configuration

    info!("Logger initialized successfully.");
    Ok(())
}
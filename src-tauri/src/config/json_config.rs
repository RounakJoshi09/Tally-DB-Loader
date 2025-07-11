use std::{env, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub technology: String,
    pub server: String,
    pub port: u16,
    pub ssl: bool,
    pub schema: String,
    pub username: String,
    pub password: String,
    pub loadmethod: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TallyConfig {
    pub definition: String,
    pub server: String,
    pub port: u16,
    pub fromdate: String,
    pub todate: String,
    pub sync: String,
    pub frequency: u32,
    pub company: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub tally: TallyConfig,
}

impl Config {
    pub fn new(database: DatabaseConfig, tally: TallyConfig) -> Self {
        Config { database, tally }
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let config: Config = serde_json::from_str(json_str)?;
        Ok(config)
    }

    pub fn update_database(&mut self, database: DatabaseConfig) {
        self.database = database;
    }
    pub fn update_tally(&mut self, tally: TallyConfig) {
        self.tally = tally;
    }
}

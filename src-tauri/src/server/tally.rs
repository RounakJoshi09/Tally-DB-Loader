use std::{error::Error, sync::MutexGuard};

use super::database::Database;
use crate::config::{
    cli::CliArgs,
    definition::YamlTableConfig,
    json_config::{Config, TallyConfig},
    utility::Utility,
    yaml_config::DataExportConfig,
};
pub struct Tally {
    config: TallyConfig,
    table_master: Vec<YamlTableConfig>,
    table_transaction: Vec<YamlTableConfig>,
    import_master: bool,
    import_transaction: bool,
}

impl Tally {
    pub fn new() -> Self {
        let config_path = Utility::get_config_path();
        let config = Config::from_json(
            &std::fs::read_to_string(config_path).expect("Error reading config.json"),
        )
        .expect("Failed to parse config.json");
        Tally {
            config: config.tally,
            table_master: vec![],
            table_transaction: vec![],
            import_master: true,
            import_transaction: true,
        }
    }

    pub fn update_command_line_config(&mut self, args: CliArgs) {
        self.config.definition = args.tally_definition;
        self.config.server = args.tally_server;
        self.config.port = args.tally_port;
        self.config.fromdate = args.tally_fromdate;
        self.config.todate = args.tally_todate;
        self.config.sync = args.tally_sync;
        self.config.frequency = args.tally_frequency;
        self.config.company = args.tally_company;

        log::info!("Tally configuration updated from command line arguments.");
    }

    fn parse_tally_export_config(&mut self, path_tally_export_defi: std::path::PathBuf) {
        let data_export_config = DataExportConfig::parse_yaml_file(path_tally_export_defi)
            .expect("Failed to parse YAML file");
        if let Some(master_data) = data_export_config.master {
            for master in master_data {
                self.table_master.push(YamlTableConfig {
                    master_transaction: master,
                    cascade_deleted: vec![],
                    cascade_updated: vec![],
                });
            }
        }
        if let Some(transaction_data) = data_export_config.transaction {
            for transaction in transaction_data {
                self.table_transaction.push(YamlTableConfig {
                    master_transaction: transaction,
                    cascade_deleted: vec![],
                    cascade_updated: vec![],
                });
            }
        }
        log::info!("YAML file parsed successfully.");
    }

    pub async fn import_data(&mut self, database_instance: MutexGuard<'static, Database>) {
        log::info!("Importing data from Tally...");

        let tally_export_defi = &self.config.definition;
        let path_tally_export_defi = Utility::get_app_base_path().join(tally_export_defi);
        if std::fs::metadata(&path_tally_export_defi).is_ok() {
            self.parse_tally_export_config(path_tally_export_defi);
        } else {
            log::error!(
                "Tally export definition file does not exist: {}",
                path_tally_export_defi.display()
            );
            std::process::exit(1);
        }

        if let Err(e) = database_instance.open_connection_pool().await {
            log::error!("Failed to open database connection pool: {}", e);
            std::process::exit(1);
        }

        if (self.config.sync == "incremental") {
            //TBD
        } else {
            let mut lst_table: Vec<YamlTableConfig> = Vec::new();

            if self.import_master {
                lst_table.append(&mut self.table_master);
            }
            if self.import_transaction {
                lst_table.append(&mut self.table_transaction);
            }

            if matches!(
                database_instance.config.technology.as_str(),
                "mssql" | "mysql" | "postgres" | "bigquery" | "csv"
            ) {
                log::info!(
                    "Updating company information configuration table [{}]",
                    chrono::Local::now().format("%Y-%m-%d")
                );

                if let Err(e) = self.save_company_info().await {
                    log::error!("Failed to save company information: {}", e);
                    std::process::exit(1);
                }
            } else {
                log::error!(
                    "Unsupported database technology: {}",
                    database_instance.config.technology
                );
                std::process::exit(1);
            }
        }
    }

    async fn save_company_info(&mut self) -> Result<(), Box<dyn Error>> {
        let company_req_path = Utility::get_tally_request_path("company");
        let company_xml = Utility::read_xml_file(company_req_path)?.replace(
            "##SVCurrentCompany",
            &Utility::escape_html(&self.config.company),
        );

        let mut company_info = self.post_tally_xml(company_xml).await?;

        if company_info.ends_with(",\"†\",\r\n") {
            company_info = company_info.replace(",\"†\",\r\n", "");
            let company_info_parts: Vec<&str> = company_info.split("\",\"").collect();

            let company_name = company_info_parts[0].replace("\"", "");

            if self.config.todate == "auto" || self.config.fromdate == "auto" {
                self.config.fromdate =
                    Utility::format_date(&company_info_parts[2].replace("\"", ""))?;
                self.config.todate =
                    Utility::format_date(&company_info_parts[1].replace("\"", ""))?;
            }
        }

        Ok(())
    }

    async fn post_tally_xml(&self, msg: String) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = format!("http://{}:{}", self.config.server, self.config.port);

        let response = client
            .post(&url)
            .header("Content-Length", msg.len().to_string())
            .header("Content-Type", "text/xml;charset=utf-8")
            .body(msg.clone())
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            log::error!(
                "Unable to connect with Tally. Ensure Tally XML port is enabled. Status: {}",
                response.status()
            );
            return Err(format!(
                "Failed to connect to Tally server. Status: {}",
                response.status()
            )
            .into());
        }

        let data = response.text_with_charset("utf-16").await?;

        log::info!("Received response from Tally: {}", data);

        Ok(data)
    }
}

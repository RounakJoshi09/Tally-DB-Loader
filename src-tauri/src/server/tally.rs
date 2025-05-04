use crate::config::{
    cli::CliArgs,
    definition::YamlTableConfig,
    json_config::{get_app_base_path, get_config_path, Config, TallyConfig},
    yaml_config::DataExportConfig,
};

pub struct Tally {
    config: TallyConfig,
    table_master: Vec<YamlTableConfig>,
    table_transaction: Vec<YamlTableConfig>,
}

impl Tally {
    pub fn new() -> Self {
        let config_path = get_config_path();
        let config = Config::from_json(
            &std::fs::read_to_string(config_path).expect("Error reading config.json"),
        )
        .expect("Failed to parse config.json");
        Tally {
            config: config.tally,
            table_master: vec![],
            table_transaction: vec![],
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

    fn parse_yaml(&mut self, path_tally_export_defi: std::path::PathBuf) {
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

    pub fn import_data(&mut self) {
        log::info!("Importing data from Tally...");

        let tally_export_defi = &self.config.definition;
        let path_tally_export_defi = get_app_base_path().join(tally_export_defi);
        if std::fs::metadata(&path_tally_export_defi).is_ok() {
            self.parse_yaml(path_tally_export_defi);
        } else {
            log::error!(
                "Tally export definition file does not exist: {}",
                path_tally_export_defi.display()
            );
            std::process::exit(1);
        }
    }
}

pub fn tally_init() -> Tally {
    Tally::new()
}

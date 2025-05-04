use tauri::utils::platform::resource_dir;

use crate::config::{
    cli::CliArgs,
    json_config::{get_config_path, Config, TallyConfig},
};

pub struct tally {
    config: TallyConfig,
}

impl tally {
    pub fn new() -> Self {
        let config_path = get_config_path();
        let config = Config::from_json(
            &std::fs::read_to_string(config_path).expect("Error reading config.json"),
        )
        .expect("Failed to parse config.json");
        tally {
            config: config.tally,
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
}

pub fn tally_init() -> tally {
    tally::new()
}

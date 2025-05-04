use crate::config::{
    cli::CliArgs,
    json_config::{get_config_path, Config, DatabaseConfig},
    logger,
};

const MAX_QUERY_SIZE: usize = 50000;

pub struct Database {
    config: DatabaseConfig,
}

impl Database {
    pub fn new() -> Self {
        let config_path = get_config_path();
        println!("Config path: {:?}", config_path);
        let config = Config::from_json(
            &std::fs::read_to_string(config_path).expect("Error reading config.json"),
        )
        .expect("Failed to parse config.json");

        Database {
            config: config.database,
        }
    }

    pub fn update_command_line_config(&mut self, args: CliArgs) {
        self.config.technology = args.database_technology;
        self.config.server = args.database_server;
        self.config.port = args.database_port;
        self.config.ssl = args.database_ssl;
        self.config.schema = args.database_schema;
        self.config.username = args.database_username;
        self.config.password = args.database_password;
        self.config.loadmethod = args.database_loadmethod;

        self.config.technology = self.config.technology.to_lowercase();

        if self.config.port == 0 {
            self.config.port = match self.config.technology.as_str() {
                "mssql" => 1433,
                "mysql" => 3306,
                "postgresql" => 5432,
                _ => panic!("Unsupported database technology"),
            };
        }
        log::info!("Database configuration updated from command line arguments.");
    }
}

// Create a public function to initialize the database
pub fn database_init() -> Database {
    Database::new()
}

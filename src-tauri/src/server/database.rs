use crate::config::{
    cli::CliArgs,
    json_config::{Config, DatabaseConfig},
    utility::Utility,
};
use mysql_async::Error as MySQLError;
use mysql_async::{OptsBuilder, Pool};

pub struct Database {
    pub config: DatabaseConfig,
}

impl Database {
    pub fn new() -> Self {
        let config_path = Utility::get_config_path();
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

    pub async fn open_connection_pool(&self) -> Result<(), String> {
        if self.config.technology == "mysql" {
            let opts = OptsBuilder::default()
                .ip_or_hostname(&self.config.server)
                .tcp_port(self.config.port)
                .db_name(Some(&self.config.schema))
                .user(Some(&self.config.username))
                .pass(Some(&self.config.password))
                .ssl_opts(if self.config.ssl {
                    Some(mysql_async::SslOpts::default())
                } else {
                    None
                });

            let pool = Pool::new(opts);

            match pool.get_conn().await {
                Ok(conn) => {
                    log::info!("MySQL connection established successfully.");
                    conn.disconnect().await.unwrap_or_else(|e| {
                        log::warn!("Failed to disconnect MySQL connection: {:?}", e);
                    });
                    Ok(())
                }
                Err(err) => {
                    let error_message = match err {
                        MySQLError::Io(_) => "Unable to make MySQL connection on specified port",
                        MySQLError::Url(_) => {
                            "Unable to make MySQL connection to servername or IP address"
                        }
                        MySQLError::Server(server_error) => match server_error.code {
                            1049 => "Invalid MySQL database name",
                            1045 => "Invalid MySQL username/password",
                            1251 => "Invalid MySQL username/password/Authentication",
                            _ => "Unknown MySQL server error",
                        },
                        _ => "An unknown error occurred",
                    };
                    Err(error_message.to_string())
                }
            }
        } else {
            Ok(())
        }
    }
}

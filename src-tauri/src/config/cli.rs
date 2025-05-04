use clap::Parser;
#[derive(Debug, Parser)]
#[command(author,version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, short, value_name = "mssql")]
    pub database_technology: String,
    #[arg(long, short, value_name = "localhost")]
    pub database_server: String,
    #[arg(long, short, value_name = "1433")]
    pub database_port: u16,
    #[arg(long, short, value_name = "false")]
    pub database_ssl: bool,
    #[arg(long, short, value_name = "master")]
    pub database_schema: String,
    #[arg(long, short, value_name = "sa")]
    pub database_username: String,
    #[arg(long, short, value_name = "password")]
    pub database_password: String,
    #[arg(long, short, value_name = "bulk")]
    pub database_loadmethod: String,

    // Tally Configuration
    #[arg(long, short, value_name = "tally.xml")]
    pub tally_definition: String,
    #[arg(long, short, value_name = "localhost")]
    pub tally_server: String,
    #[arg(long, short, value_name = "9000")]
    pub tally_port: u16,
    #[arg(long, short, value_name = "2023-01-01")]
    pub tally_fromdate: String,
    #[arg(long, short, value_name = "2023-12-31")]
    pub tally_todate: String,
    #[arg(long, short, value_name = "full")]
    pub tally_sync: String,
    #[arg(long, short, value_name = "0")]
    pub tally_frequency: u32,
    #[arg(long, short, value_name = "company")]
    pub tally_company: String,
    #[arg(long, short)]
    pub tally_master: bool,
    #[arg(long, short)]
    pub tally_transaction: bool,
    #[arg(long, short)]
    pub tally_truncate: bool,
    #[arg(long, short)]
    pub tally_export: bool,
}

use clap::Parser;
#[derive(Debug, Parser, Clone)]
#[command(author,version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, value_name = "mssql")]
    pub database_technology: String,
    #[arg(long, value_name = "localhost")]
    pub database_server: String,
    #[arg(long, value_name = "1433")]
    pub database_port: u16,
    #[arg(long, value_name = "false")]
    pub database_ssl: bool,
    #[arg(long, value_name = "master")]
    pub database_schema: String,
    #[arg(long, value_name = "sa")]
    pub database_username: String,
    #[arg(long, value_name = "password")]
    pub database_password: String,
    #[arg(long, value_name = "bulk")]
    pub database_loadmethod: String,

    // Tally Configuration
    #[arg(long, value_name = "tally.xml")]
    pub tally_definition: String,
    #[arg(long, value_name = "localhost")]
    pub tally_server: String,
    #[arg(long, value_name = "9000")]
    pub tally_port: u16,
    #[arg(long, value_name = "2023-01-01")]
    pub tally_fromdate: String,
    #[arg(long, value_name = "2023-12-31")]
    pub tally_todate: String,
    #[arg(long, value_name = "full")]
    pub tally_sync: String,
    #[arg(long, value_name = "0")]
    pub tally_frequency: u32,
    #[arg(long, value_name = "company")]
    pub tally_company: String,
    #[arg(long)]
    pub tally_master: bool,
    #[arg(long)]
    pub tally_transaction: bool,
    #[arg(long)]
    pub tally_truncate: bool,
    #[arg(long)]
    pub tally_export: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

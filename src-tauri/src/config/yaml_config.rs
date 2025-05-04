use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
    pub name: String,
    pub collection: String,
    pub nature: String,
    pub fields: Option<Vec<Field>>,
    pub fetch: Option<Vec<String>>,
    pub filters: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub name: String,
    pub collection: String,
    pub nature: String,
    pub fields: Option<Vec<Field>>,
    pub fetch: Option<Vec<String>>,
    pub filters: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataExportConfig {
    pub master: Option<Vec<Master>>,
    pub transaction: Option<Vec<Transaction>>,
}

impl DataExportConfig {
    pub fn parse_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: DataExportConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

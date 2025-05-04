use super::yaml_config::MasterTransaction;

pub struct TableFieldYaml {
    pub table: String,
    pub field: String,
}

pub struct YamlTableConfig {
    pub master_transaction: MasterTransaction,
    pub cascade_updated: Vec<TableFieldYaml>,
    pub cascade_deleted: Vec<TableFieldYaml>,
}

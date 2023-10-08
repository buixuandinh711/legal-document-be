use cloud_storage::Client;
use deadpool_postgres::Pool;
use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub chain_rpc_url: String,
    pub legal_document_address: String,
    pub pg: deadpool_postgres::Config,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

pub struct CloudStorage {
    pub client: Client,
    pub bucket_name: String,
    pub base_url: String,
}

pub struct AppState {
    pub db_pool: Pool,
    pub cloud_storage: CloudStorage,
}

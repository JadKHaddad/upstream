use std::sync::Arc;

use rustls::ServerConfig;

use crate::FileTlsServerConfigLoader;

#[derive(Clone)]
pub struct StaticFileTlsServerConfigProvider {
    config: Arc<ServerConfig>,
}

impl StaticFileTlsServerConfigProvider {
    pub async fn new(loader: FileTlsServerConfigLoader) -> anyhow::Result<Self> {
        let config = loader.load().await?;

        Ok(Self {
            config: Arc::new(config),
        })
    }

    pub fn get_server_config(&self) -> Arc<ServerConfig> {
        self.config.clone()
    }
}

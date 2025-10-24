use std::sync::Arc;

use rustls::ClientConfig;

use crate::loader::FileTlsClientConfigLoader;

#[derive(Clone)]
pub struct StaticFileTlsClientConfigProvider {
    config: Arc<ClientConfig>,
}

impl StaticFileTlsClientConfigProvider {
    pub async fn new(loader: FileTlsClientConfigLoader) -> anyhow::Result<Self> {
        let config = loader.load().await?;

        Ok(Self {
            config: Arc::new(config),
        })
    }

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        self.config.clone()
    }
}

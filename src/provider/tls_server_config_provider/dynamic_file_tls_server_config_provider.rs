use std::sync::Arc;

use arc_swap::ArcSwap;
use rustls::ServerConfig;

use crate::FileTlsServerConfigLoader;

#[derive(Clone)]
pub struct DynamicFileTlsServerConfigProvider {
    loader: FileTlsServerConfigLoader,
    /// Used a fallback if loading the file fails,
    config: Arc<ArcSwap<ServerConfig>>,
}

impl DynamicFileTlsServerConfigProvider {
    pub async fn new(loader: FileTlsServerConfigLoader) -> anyhow::Result<Self> {
        let config = loader.load().await?;

        Ok(Self {
            loader,
            config: Arc::new(ArcSwap::new(Arc::new(config))),
        })
    }

    pub async fn get_server_config(&self) -> Arc<ServerConfig> {
        match self.loader.load().await {
            Ok(config) => {
                self.config.store(Arc::new(config));

                self.config.load_full()
            }
            Err(err) => {
                tracing::error!(%err, "Failed to load TLS server config");

                self.config.load_full()
            }
        }
    }
}

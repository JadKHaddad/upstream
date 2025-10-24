use std::sync::Arc;

use crate::{FileTlsServerConfigLoader, config::Watch};

mod static_file_tls_server_config_provider;
use static_file_tls_server_config_provider::StaticFileTlsServerConfigProvider;

mod watch_file_tls_server_config_provider;
use watch_file_tls_server_config_provider::WatchFileTlsServerConfigProvider;

#[derive(Clone)]
pub enum TlsServerConfigProvider {
    StaticFile(StaticFileTlsServerConfigProvider),
    WatchFile(WatchFileTlsServerConfigProvider),
}

impl TlsServerConfigProvider {
    pub async fn static_file(loader: FileTlsServerConfigLoader) -> anyhow::Result<Self> {
        let provider = StaticFileTlsServerConfigProvider::new(loader).await?;

        Ok(Self::StaticFile(provider))
    }

    pub async fn watch_file(
        loader: FileTlsServerConfigLoader,
        watch: Watch,
    ) -> anyhow::Result<Self> {
        let provider = WatchFileTlsServerConfigProvider::new(loader, watch).await?;

        Ok(Self::WatchFile(provider))
    }

    pub fn get_server_config(&self) -> Arc<rustls::ServerConfig> {
        match self {
            TlsServerConfigProvider::StaticFile(provider) => provider.get_server_config(),
            TlsServerConfigProvider::WatchFile(provider) => provider.get_server_config(),
        }
    }
}

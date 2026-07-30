use std::sync::Arc;

use crate::{FileTlsServerConfigLoader, config::WatchMethod};

mod static_file_tls_server_config_provider;
use static_file_tls_server_config_provider::StaticFileTlsServerConfigProvider;

mod dynamic_file_tls_server_config_provider;
use dynamic_file_tls_server_config_provider::DynamicFileTlsServerConfigProvider;

mod watch_file_tls_server_config_provider;
use watch_file_tls_server_config_provider::WatchFileTlsServerConfigProvider;

#[derive(Clone)]
pub enum TlsServerConfigProvider {
    Static(StaticFileTlsServerConfigProvider),
    Dynamic(DynamicFileTlsServerConfigProvider),
    Watch(WatchFileTlsServerConfigProvider),
}

impl TlsServerConfigProvider {
    pub async fn static_file(loader: FileTlsServerConfigLoader) -> anyhow::Result<Self> {
        let provider = StaticFileTlsServerConfigProvider::new(loader).await?;

        Ok(Self::Static(provider))
    }

    pub async fn dynamic_file(loader: FileTlsServerConfigLoader) -> anyhow::Result<Self> {
        let provider = DynamicFileTlsServerConfigProvider::new(loader).await?;

        Ok(Self::Dynamic(provider))
    }

    pub async fn watch_file(
        loader: FileTlsServerConfigLoader,
        watch: WatchMethod,
    ) -> anyhow::Result<Self> {
        let provider = WatchFileTlsServerConfigProvider::new(loader, watch).await?;

        Ok(Self::Watch(provider))
    }

    pub async fn get_server_config(&self) -> Arc<rustls::ServerConfig> {
        match self {
            TlsServerConfigProvider::Static(provider) => provider.get_server_config(),
            TlsServerConfigProvider::Dynamic(provider) => provider.get_server_config().await,
            TlsServerConfigProvider::Watch(provider) => provider.get_server_config(),
        }
    }
}

mod webpki_tls_client_config_provider;
use std::sync::Arc;

use rustls::ClientConfig;
use webpki_tls_client_config_provider::WebPkiTlsClientConfigProvider;

mod static_file_tls_client_config_provider;
use static_file_tls_client_config_provider::StaticFileTlsClientConfigProvider;

mod watch_file_tls_client_config_provider;
use watch_file_tls_client_config_provider::WatchFileTlsClientConfigProvider;

use crate::{config::Watch, loader::FileTlsClientConfigLoader};

#[derive(Clone)]
pub enum TlsClientConfigProvider {
    WebPki(WebPkiTlsClientConfigProvider),
    StaticFile(StaticFileTlsClientConfigProvider),
    WatchFile(WatchFileTlsClientConfigProvider),
}

impl TlsClientConfigProvider {
    pub fn webpki() -> Self {
        let provider = WebPkiTlsClientConfigProvider::new();

        Self::WebPki(provider)
    }

    pub async fn static_file(loader: FileTlsClientConfigLoader) -> anyhow::Result<Self> {
        let provider = StaticFileTlsClientConfigProvider::new(loader).await?;

        Ok(Self::StaticFile(provider))
    }

    pub async fn watch_file(
        loader: FileTlsClientConfigLoader,
        watch: Watch,
    ) -> anyhow::Result<Self> {
        let provider = WatchFileTlsClientConfigProvider::new(loader, watch).await?;

        Ok(Self::WatchFile(provider))
    }

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        match self {
            TlsClientConfigProvider::WebPki(provider) => provider.get_client_config(),
            TlsClientConfigProvider::StaticFile(provider) => provider.get_client_config(),
            TlsClientConfigProvider::WatchFile(provider) => provider.get_client_config(),
        }
    }
}

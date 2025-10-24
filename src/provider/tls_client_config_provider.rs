mod webpki_tls_client_config_provider;
use std::sync::Arc;

use rustls::ClientConfig;
use webpki_tls_client_config_provider::WebPkiTlsClientConfigProvider;

mod static_file_tls_client_config_provider;
use static_file_tls_client_config_provider::StaticFileTlsClientConfigProvider;

use crate::loader::FileTlsClientConfigLoader;

#[derive(Clone)]
pub enum TlsClientConfigProvider {
    WebPki(WebPkiTlsClientConfigProvider),
    StaticFile(StaticFileTlsClientConfigProvider),
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

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        match self {
            TlsClientConfigProvider::WebPki(provider) => provider.get_client_config(),
            TlsClientConfigProvider::StaticFile(provider) => provider.get_client_config(),
        }
    }
}

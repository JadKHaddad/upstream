mod webpki_tls_client_config_provider;
use std::sync::Arc;

use rustls::ClientConfig;
use webpki_tls_client_config_provider::WebPkiTlsClientConfigProvider;

#[derive(Clone)]
pub enum TlsClientConfigProvider {
    WebPki(WebPkiTlsClientConfigProvider),
}

impl TlsClientConfigProvider {
    pub fn webpki() -> Self {
        let provider = WebPkiTlsClientConfigProvider::new();

        Self::WebPki(provider)
    }

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        match self {
            TlsClientConfigProvider::WebPki(provider) => provider.get_client_config(),
        }
    }
}

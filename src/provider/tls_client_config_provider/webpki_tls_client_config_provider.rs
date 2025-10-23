use std::sync::Arc;

use rustls::ClientConfig;

// TODO: can be made static global since it uses webpki roots only (cheap to clone, very cheap)
#[derive(Clone)]
pub struct WebPkiTlsClientConfigProvider {
    config: Arc<ClientConfig>,
}

impl WebPkiTlsClientConfigProvider {
    pub fn new() -> Self {
        let mut root_store = rustls::RootCertStore::empty();

        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Self {
            config: Arc::new(config),
        }
    }

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        self.config.clone()
    }
}

impl Default for WebPkiTlsClientConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}

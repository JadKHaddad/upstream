use std::path::PathBuf;

use rustls::ServerConfig;

mod file_tls_server_config_loader;
pub use file_tls_server_config_loader::FileTlsServerConfigLoader;

#[derive(Clone)]
pub enum TlsServerConfigLoader {
    File(FileTlsServerConfigLoader),
}

impl TlsServerConfigLoader {
    pub fn file(certs: PathBuf, key: PathBuf) -> Self {
        Self::File(FileTlsServerConfigLoader::new(certs, key))
    }

    pub async fn load(&self) -> anyhow::Result<ServerConfig> {
        match self {
            TlsServerConfigLoader::File(loader) => loader.load().await,
        }
    }
}

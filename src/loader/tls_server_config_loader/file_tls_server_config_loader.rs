use std::path::PathBuf;

use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

#[derive(Clone)]
pub struct FileTlsServerConfigLoader {
    certs: PathBuf,
    key: PathBuf,
}

impl FileTlsServerConfigLoader {
    pub fn new(certs: PathBuf, key: PathBuf) -> Self {
        Self { certs, key }
    }

    pub async fn load(&self) -> anyhow::Result<ServerConfig> {
        let certs = tokio::fs::read(&self.certs).await?;
        let key = tokio::fs::read(&self.key).await?;

        let certs = CertificateDer::pem_slice_iter(&certs).collect::<Result<Vec<_>, _>>()?;
        let key = PrivateKeyDer::from_pem_slice(&key)?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(config)
    }

    pub fn paths(&self) -> [PathBuf; 2] {
        [self.certs.clone(), self.key.clone()]
    }
}

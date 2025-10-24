use std::path::PathBuf;

use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, pem::PemObject},
};

#[derive(Clone)]
pub struct FileTlsClientConfigLoader {
    certs: PathBuf,
}

impl FileTlsClientConfigLoader {
    pub fn new(certs: PathBuf) -> Self {
        Self { certs }
    }

    pub async fn load(&self) -> anyhow::Result<ClientConfig> {
        let mut root_store = RootCertStore::empty();

        let certs = tokio::fs::read(&self.certs).await?;

        let certs = CertificateDer::pem_slice_iter(&certs).collect::<Result<Vec<_>, _>>()?;

        root_store.add_parsable_certificates(certs);

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(config)
    }

    pub fn paths(&self) -> [PathBuf; 1] {
        [self.certs.clone()]
    }
}

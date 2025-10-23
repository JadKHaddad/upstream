use std::net::SocketAddr;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::DnsResolver;

#[derive(Debug)]
pub struct Upstream {
    pub domain: &'static str,
    pub port: u16,
    pub resolver: DnsResolver,
}

impl Clone for Upstream {
    fn clone(&self) -> Self {
        #[cfg(debug_assertions)]
        tracing::debug!("Cloned");

        Self {
            domain: self.domain,
            port: self.port,
            resolver: self.resolver.clone(),
        }
    }
}

impl Upstream {
    pub const fn new(domain: &'static str, port: u16, resolver: DnsResolver) -> Self {
        Self {
            domain,
            port,
            resolver,
        }
    }

    pub async fn connect(&self) -> anyhow::Result<(impl AsyncRead + AsyncWrite, SocketAddr)> {
        // TODO: iterate over the addr iter and try connect to all
        let addr = self
            .resolver
            .lookup_with_port(self.domain, self.port)
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Failed to resolve upstream domain"))?;

        tracing::info!(%addr, "Connecting to upstream");

        Ok((tokio::net::TcpStream::connect(addr).await?, addr))
    }
}

use std::net::SocketAddr;

use tokio::io::{AsyncRead, AsyncWrite};

pub struct Upstream {
    pub domain: &'static str,
    pub port: u16,
}

impl Clone for Upstream {
    fn clone(&self) -> Self {
        #[cfg(debug_assertions)]
        tracing::debug!("Cloned");

        Self {
            domain: self.domain,
            port: self.port,
        }
    }
}

impl Upstream {
    pub const fn new(domain: &'static str, port: u16) -> Self {
        Self { domain, port }
    }

    pub async fn connect(&self, addr: SocketAddr) -> anyhow::Result<impl AsyncRead + AsyncWrite> {
        Ok(tokio::net::TcpStream::connect(addr).await?)
    }
}

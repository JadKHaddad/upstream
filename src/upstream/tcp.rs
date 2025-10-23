use std::net::SocketAddr;

use crate::{DnsResolver, upstream::UpstreamAddress};

mod tcp_stream;
pub use tcp_stream::ClientTcpStream;

#[derive(Clone)]
pub struct TcpUpstream {
    addr: UpstreamAddress,
    resolver: DnsResolver,
    kind: TcpUpstreamKind,
}

impl std::fmt::Debug for TcpUpstream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpUpstream")
            .field("addr", &self.addr)
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum TcpUpstreamKind {
    Plain,
}

impl TcpUpstream {
    pub fn plain(addr: UpstreamAddress, resolver: DnsResolver) -> Self {
        Self {
            addr,
            resolver,
            kind: TcpUpstreamKind::Plain,
        }
    }

    // TODO: do the same in tcp_server_stream
    pub async fn connect(&self) -> anyhow::Result<(ClientTcpStream, SocketAddr)> {
        // TODO: iterate over the addr iter and try connect to all
        let addr = self
            .resolver
            .lookup_with_port(self.addr.domain, self.addr.port)
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Failed to resolve upstream domain"))?;

        tracing::info!(%addr, "Connecting to upstream");

        Ok((
            ClientTcpStream::plain(tokio::net::TcpStream::connect(addr).await?),
            addr,
        ))
    }
}

use std::net::SocketAddr;

use crate::{DnsResolver, provider::TlsClientConfigProvider, upstream::UpstreamAddress};

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

#[derive(Clone)]
pub enum TcpUpstreamKind {
    Plain,
    Tls(TlsClientConfigProvider),
}

impl std::fmt::Debug for TcpUpstreamKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TcpUpstreamKind::Plain => f.debug_tuple("Plain").finish(),
            TcpUpstreamKind::Tls(_) => f.debug_tuple("Tls").finish(),
        }
    }
}

impl TcpUpstreamKind {
    async fn connect(
        &self,
        socket_addr: SocketAddr,
        upstream_addr: UpstreamAddress,
    ) -> anyhow::Result<ClientTcpStream> {
        let stream = tokio::net::TcpStream::connect(socket_addr).await?;

        match self {
            TcpUpstreamKind::Plain => Ok(ClientTcpStream::plain(stream)),
            TcpUpstreamKind::Tls(provider) => {
                let config = provider.get_client_config();

                let connector = tokio_rustls::TlsConnector::from(config);

                let domain = rustls_pki_types::ServerName::try_from(upstream_addr.domain)?;

                let stream = connector.connect(domain, stream).await?;

                Ok(ClientTcpStream::tls(stream))
            }
        }
    }
}

impl TcpUpstream {
    pub fn plain(addr: UpstreamAddress, resolver: DnsResolver) -> Self {
        Self {
            addr,
            resolver,
            kind: TcpUpstreamKind::Plain,
        }
    }

    pub fn tls(
        addr: UpstreamAddress,
        resolver: DnsResolver,
        provider: TlsClientConfigProvider,
    ) -> Self {
        Self {
            addr,
            resolver,
            kind: TcpUpstreamKind::Tls(provider),
        }
    }

    pub async fn connect(&self) -> anyhow::Result<(ClientTcpStream, SocketAddr)> {
        // TODO: iterate over the addr iter and try connect to all
        let addr = self
            .resolver
            .lookup_with_port(self.addr.domain, self.addr.port)
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Failed to resolve upstream domain"))?;

        tracing::info!(%addr, "Connecting to upstream");

        let stream = self.kind.connect(addr, self.addr.clone()).await?;

        Ok((stream, addr))
    }
}

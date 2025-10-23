use std::net::SocketAddr;

use crate::{
    DnsResolver, TlsClientConfigProvider,
    config::{UpstreamConfig, UpstreamConfigKind, UpstreamConfigTlsCertsKind},
    upstream::connected::ConnectedUpstream,
};

mod tcp;
pub use tcp::TcpUpstream;

mod address;
pub use address::UpstreamAddress;

mod connected;

#[derive(Clone, Debug)]
pub struct Upstream {
    kind: UpstreamKind,
}

#[derive(Clone, Debug)]
pub enum UpstreamKind {
    Tcp(TcpUpstream),
}

impl Upstream {
    pub fn tcp(upstream: TcpUpstream) -> Self {
        Self {
            kind: UpstreamKind::Tcp(upstream),
        }
    }

    pub fn from_config(upstream: UpstreamConfig, resolver: DnsResolver) -> Self {
        let upstream = match upstream.kind {
            UpstreamConfigKind::Tcp => TcpUpstream::plain(
                UpstreamAddress::new(upstream.domain.leak(), upstream.port),
                resolver.clone(),
            ),
            UpstreamConfigKind::Tls { certs } => match certs {
                UpstreamConfigTlsCertsKind::WebPki => TcpUpstream::tls(
                    UpstreamAddress::new(upstream.domain.leak(), upstream.port),
                    resolver.clone(),
                    TlsClientConfigProvider::webpki(),
                ),
            },
        };

        Self::tcp(upstream)
    }

    pub async fn connect(&self) -> anyhow::Result<(ConnectedUpstream, SocketAddr)> {
        match &self.kind {
            UpstreamKind::Tcp(upstream) => {
                let (stream, addr) = upstream.connect().await?;

                Ok((ConnectedUpstream::tcp(stream), addr))
            }
        }
    }
}

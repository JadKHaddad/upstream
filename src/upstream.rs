use std::net::SocketAddr;

use crate::{
    DnsResolver, TlsClientConfigProvider,
    config::{UpstreamConfig, UpstreamConfigKind, UpstreamConfigTlsCertsKind},
    loader::FileTlsClientConfigLoader,
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

    pub async fn try_from_config(
        upstream: UpstreamConfig,
        resolver: DnsResolver,
    ) -> anyhow::Result<Self> {
        let upstream = match upstream.kind {
            UpstreamConfigKind::Tcp => TcpUpstream::plain(
                UpstreamAddress::new(upstream.domain.leak(), upstream.port),
                resolver.clone(),
            ),
            UpstreamConfigKind::Tls { certs } => {
                let provider = match certs {
                    UpstreamConfigTlsCertsKind::WebPki => TlsClientConfigProvider::webpki(),
                    UpstreamConfigTlsCertsKind::File { file } => {
                        let loader = FileTlsClientConfigLoader::new(file.certs);

                        match file.watch {
                            None => TlsClientConfigProvider::static_file(loader).await?,
                            Some(watch) => {
                                TlsClientConfigProvider::watch_file(loader, watch).await?
                            }
                        }
                    }
                };
                TcpUpstream::tls(
                    UpstreamAddress::new(upstream.domain.leak(), upstream.port),
                    resolver.clone(),
                    provider,
                )
            }
        };

        Ok(Self::tcp(upstream))
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

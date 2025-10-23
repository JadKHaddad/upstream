use std::net::SocketAddr;

use crate::upstream::connected::ConnectedUpstream;

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

    pub async fn connect(&self) -> anyhow::Result<(ConnectedUpstream, SocketAddr)> {
        match &self.kind {
            UpstreamKind::Tcp(upstream) => {
                let (stream, addr) = upstream.connect().await?;

                Ok((ConnectedUpstream::tcp(stream), addr))
            }
        }
    }
}

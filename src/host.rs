mod tcp;
use tcp::TcpHost;

use crate::LoadBalancer;

pub struct Host {
    kind: HostKind,
}

pub enum HostKind {
    Tcp(TcpHost),
}

impl Host {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.kind {
            HostKind::Tcp(host) => host.run().await,
        }
    }
}

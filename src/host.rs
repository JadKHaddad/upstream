mod tcp;
pub use tcp::TcpHost;

pub struct Host {
    kind: HostKind,
}

pub enum HostKind {
    Tcp(TcpHost),
}

impl Host {
    pub fn tcp(host: TcpHost) -> Self {
        Self {
            kind: HostKind::Tcp(host),
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        match self.kind {
            HostKind::Tcp(host) => host.run().await,
        }
    }
}

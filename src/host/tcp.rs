use std::net::SocketAddr;

mod tcp_stream;
use tcp_stream::ServerTcpStream;

use crate::{LoadBalancer, TlsServerConfigProvider};

pub struct TcpHost {
    addr: SocketAddr,
    load_balancer: LoadBalancer,
    kind: TcpHostKind,
}

impl TcpHost {
    pub async fn run(mut self) -> anyhow::Result<()> {
        let addr = self.addr;

        let listener = self.kind.bind(addr).await?;

        loop {
            let (mut stream, addr) = listener.accept().await?;

            tracing::info!(%addr, "Accepted connection");

            let Some(upstream) = self.load_balancer.next() else {
                unreachable!("Load balancer should emit an upstream")
            };

            tracing::info!(%addr, domain=%upstream.domain, port=%upstream.port, "Found upstream");

            let fut = async move {
                let (mut upstream, upstream_addr) = upstream.connect().await?;

                tokio::io::copy_bidirectional(&mut stream, &mut upstream).await?;

                tracing::info!(%addr, %upstream_addr, "Connection closed");

                Ok::<(), anyhow::Error>(())
            };

            tokio::spawn(async move {
                if let Err(err) = fut.await {
                    tracing::error!(%err, %addr, "Connection error");
                }
            });
        }
    }
}

pub enum TcpHostKind {
    Plain,
    Tls(TlsServerConfigProvider),
}

impl TcpHostKind {
    async fn bind(self, addr: SocketAddr) -> anyhow::Result<Listener> {
        let listener = tokio::net::TcpListener::bind(addr).await?;

        let listener = match self {
            TcpHostKind::Plain => Listener::plain(listener),
            TcpHostKind::Tls(provider) => Listener::tls(listener, provider),
        };

        Ok(listener)
    }
}

struct Listener {
    listener: tokio::net::TcpListener,
    kind: ListenerKind,
}

impl Listener {
    fn plain(listener: tokio::net::TcpListener) -> Self {
        Self {
            listener,
            kind: ListenerKind::Plain,
        }
    }

    fn tls(listener: tokio::net::TcpListener, provider: TlsServerConfigProvider) -> Self {
        Self {
            listener,
            kind: ListenerKind::Tls(provider),
        }
    }
}

enum ListenerKind {
    Plain,
    Tls(TlsServerConfigProvider),
}

impl Listener {
    async fn accept(&self) -> anyhow::Result<(ServerTcpStream, SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;

        match &self.kind {
            ListenerKind::Plain => Ok((ServerTcpStream::plain(stream), addr)),
            ListenerKind::Tls(provider) => {
                let config = provider.get_server_config();

                let acceptor = tokio_rustls::TlsAcceptor::from(config);

                let stream = acceptor.accept(stream).await?;

                Ok((ServerTcpStream::tls(stream), addr))
            }
        }
    }
}

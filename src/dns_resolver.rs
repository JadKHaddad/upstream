use std::net::{IpAddr, SocketAddr};

mod hickory_dns_resolver;
pub use hickory_dns_resolver::HickoryDnsResolver;

mod tokio_dns_resolver;
use tokio_dns_resolver::TokioDnsResolver;

use crate::dns_resolver::global_hickory_dns_resolver::GlobalHickoryDnsResolver;

mod global_hickory_dns_resolver;

#[derive(Clone)]
pub enum DnsResolver {
    Hickory(GlobalHickoryDnsResolver),
    Tokio(TokioDnsResolver),
}

impl DnsResolver {
    pub fn hickory() -> anyhow::Result<Self> {
        Ok(Self::Hickory(GlobalHickoryDnsResolver::new()?))
    }

    pub fn tokio() -> Self {
        Self::Tokio(TokioDnsResolver::new())
    }

    async fn lookup(
        &self,
        domain: impl AsRef<str> + Send,
    ) -> anyhow::Result<impl Iterator<Item = IpAddr>> {
        enum I<A, B> {
            A(A),
            B(B),
        }

        impl<A: Iterator<Item = IpAddr>, B: Iterator<Item = IpAddr>> Iterator for I<A, B> {
            type Item = IpAddr;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::A(iter) => iter.next(),
                    Self::B(iter) => iter.next(),
                }
            }
        }

        match self {
            Self::Hickory(resolver) => Ok(I::A(resolver.lookup(domain).await?)),
            Self::Tokio(resolver) => Ok(I::B(resolver.lookup(domain).await?)),
        }
    }

    pub async fn lookup_with_port(
        &self,
        domain: impl AsRef<str> + Send,
        port: u16,
    ) -> anyhow::Result<impl Iterator<Item = SocketAddr>> {
        let iter = self
            .lookup(domain)
            .await?
            .map(move |ip| SocketAddr::new(ip, port));

        Ok(iter)
    }
}

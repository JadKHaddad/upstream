use std::net::IpAddr;

use hickory_resolver::TokioResolver;

pub struct HickoryDnsResolver {
    resolver: TokioResolver,
}

impl HickoryDnsResolver {
    pub fn new() -> anyhow::Result<Self> {
        let resolver = TokioResolver::builder_tokio()?.build();

        Ok(Self { resolver })
    }

    pub async fn lookup(
        &self,
        domain: impl AsRef<str>,
    ) -> anyhow::Result<impl Iterator<Item = IpAddr>> {
        let iter = self.resolver.lookup_ip(domain.as_ref()).await?.into_iter();

        Ok(iter)
    }
}

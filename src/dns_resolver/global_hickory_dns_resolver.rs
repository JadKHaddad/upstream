use std::{net::IpAddr, sync::OnceLock};

use anyhow::ensure;

use crate::dns_resolver::HickoryDnsResolver;

static HICKORY_DNS_RESOLVER: OnceLock<HickoryDnsResolver> = OnceLock::new();

fn get() -> &'static HickoryDnsResolver {
    HICKORY_DNS_RESOLVER
        .get()
        .expect("HICKORY_DNS_RESOLVER must have been initialized")
}

fn initialized() -> bool {
    HICKORY_DNS_RESOLVER.get().is_some()
}

fn init(resolver: HickoryDnsResolver) {
    HICKORY_DNS_RESOLVER.get_or_init(|| resolver);
}

#[derive(Debug, Clone)]
pub struct GlobalHickoryDnsResolver {
    _priv: (),
}

impl GlobalHickoryDnsResolver {
    /// Creates a new [`GlobalHickoryDnsResolver`] and initializes the global resolver.
    ///
    /// Must be created only once. Successive calls to this method will return an error.
    pub fn new() -> anyhow::Result<Self> {
        ensure!(
            !initialized(),
            "HICKORY_DNS_RESOLVER has already been initialized"
        );

        let resolver = HickoryDnsResolver::new()?;

        init(resolver);

        Ok(Self { _priv: () })
    }

    pub async fn lookup(
        &self,
        domain: impl AsRef<str> + Send,
    ) -> anyhow::Result<impl Iterator<Item = IpAddr>> {
        get().lookup(domain).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_only_be_created_once() {
        let _ = GlobalHickoryDnsResolver::new().expect("Must be created");
        let _ = GlobalHickoryDnsResolver::new().expect_err("Must not be created");
    }
}

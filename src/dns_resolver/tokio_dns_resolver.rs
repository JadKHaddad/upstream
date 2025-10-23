use std::net::IpAddr;

#[derive(Clone, Default, Debug)]
pub struct TokioDnsResolver {
    _priv: (),
}

impl TokioDnsResolver {
    pub fn new() -> Self {
        Self { _priv: () }
    }

    pub async fn lookup(
        &self,
        domain: impl AsRef<str>,
    ) -> anyhow::Result<impl Iterator<Item = IpAddr>> {
        let iter = tokio::net::lookup_host(String::from(domain.as_ref()))
            .await?
            .map(|addr| addr.ip());

        Ok(iter)
    }
}

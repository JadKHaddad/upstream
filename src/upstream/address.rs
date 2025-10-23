#[derive(Clone, Debug)]
pub struct UpstreamAddress {
    pub domain: &'static str,
    pub port: u16,
}

impl UpstreamAddress {
    pub fn new(domain: &'static str, port: u16) -> Self {
        Self { domain, port }
    }
}

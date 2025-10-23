use crate::Upstream;

pub struct IdentityLoadBalancer {
    upstream: Upstream,
}

impl IdentityLoadBalancer {
    pub fn new(upstream: Upstream) -> Self {
        Self { upstream }
    }
}

impl Iterator for IdentityLoadBalancer {
    type Item = Upstream;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.upstream.clone())
    }
}

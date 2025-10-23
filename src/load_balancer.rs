use crate::Upstream;

mod fifo_load_balancer;
use fifo_load_balancer::FifoLoadBalancer;

mod static_fifo_load_balancer;
use static_fifo_load_balancer::StaticFifoLoadBalancer;

mod identity_load_balancer;
use identity_load_balancer::IdentityLoadBalancer;

pub enum LoadBalancer {
    Identity(IdentityLoadBalancer),
    StaticFifo(StaticFifoLoadBalancer),
    Fifo(FifoLoadBalancer),
}

impl LoadBalancer {
    pub fn identity(upstream: Upstream) -> Self {
        Self::Identity(IdentityLoadBalancer::new(upstream))
    }

    pub fn static_fifo(upstreams: &'static [Upstream]) -> Self {
        Self::StaticFifo(StaticFifoLoadBalancer::new(upstreams))
    }

    pub fn fifo(upstreams: Vec<Upstream>) -> Self {
        Self::Fifo(FifoLoadBalancer::new(upstreams))
    }
}

impl Iterator for LoadBalancer {
    type Item = Upstream;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Identity(iter) => iter.next(),
            Self::StaticFifo(iter) => iter.next(),
            Self::Fifo(iter) => iter.next(),
        }
    }
}

use crate::Upstream;

pub struct StaticFifoLoadBalancer {
    iter: std::iter::Cycle<std::slice::Iter<'static, Upstream>>,
}

impl StaticFifoLoadBalancer {
    pub fn new(upstreams: &'static [Upstream]) -> Self {
        Self {
            iter: upstreams.iter().cycle(),
        }
    }
}

impl Iterator for StaticFifoLoadBalancer {
    type Item = Upstream;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().cloned()
    }
}

use std::{iter::Cycle, vec::IntoIter};

use crate::Upstream;

pub struct FifoLoadBalancer {
    iter: Cycle<IntoIter<Upstream>>,
}

impl FifoLoadBalancer {
    pub fn new(upstreams: Vec<Upstream>) -> Self {
        let iter = upstreams.into_iter().cycle();

        Self { iter }
    }
}

impl Iterator for FifoLoadBalancer {
    type Item = Upstream;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

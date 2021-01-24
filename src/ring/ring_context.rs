use super::ring_elm::RingElm;
use super::Element;

#[derive(Debug)]
pub struct RingContext {
    modulo: Element,
}

impl RingContext {
    pub fn new(x: u64) -> Self
    {
        Self { modulo: x }
    }
    pub fn elm(&self, x: u64) -> RingElm
    {
        RingElm::new(x, self.modulo)
    }
}

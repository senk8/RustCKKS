use super::Element;
use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RingElm {
    pub value: Element,
    pub modulo: Element,
}

use std::fmt;
impl fmt::Display for RingElm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0} mod {1}",
            self.value.to_string(),
            self.value.to_string()
        )
    }
}

impl RingElm {
    pub fn new(value: Element, modulo: Element) -> Self {
        Self {
            value: value % modulo,
            modulo: modulo,
        }
    }
}

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
            self.modulo.to_string()
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

impl Add for RingElm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value, self.modulo)
    }
}

impl Sub for RingElm {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.modulo != other.modulo {
            panic!("This operation is not enclose in its field.")
        }
        Self::new(
            if self.value < other.value {
                self.modulo - (other.value - self.value)
            } else {
                self.value - other.value
            },
            self.modulo,
        )
    }
}

impl Mul for RingElm {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value + self.modulo, self.modulo)
    }
}

impl AddAssign for RingElm {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for RingElm {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for RingElm {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

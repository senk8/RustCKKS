use super::*;
use std::ops::*;

#[derive(Debug)]
pub struct Polynomial(Vec<GF>);


impl Add for Polynomial{
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self {
        Polynomial(self.0)
    }
}

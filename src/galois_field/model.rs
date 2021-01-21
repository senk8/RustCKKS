use super::*;
use std::ops::*;

#[derive(Debug)]
pub struct Polynomial(Vec<GF>);


impl Add for Polynomial{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.0 + rhs.0
    }
}

pub fn is_irreducible(poly:Polynomial) -> bool
{
    let p =3;
    let mut irreducible = true;
    if poly[0]%p==0 && poly[0]*poly[0]%p {
    }

    for i in 1..k{
        irreducible &= poly[i]%3==0;
    }
 
    if poly[k]%p != 0
    irreducible
}
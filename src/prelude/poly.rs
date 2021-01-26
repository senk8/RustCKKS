use super::gf_context::GFContext;
use super::gf_elm::GFElm;
use std::fmt;
use std::ops::*;

pub struct Polynomial(Vec<GFElm>);

impl Polynomial {
    pub fn new(vec: Vec<u64>, gc: &GFContext) -> Polynomial {
        Polynomial(vec.iter().map(|x| gc.elm(*x)).collect())
    }
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Polynomial(self.0.iter().zip(rhs.0).map(|(x, y)| *x + y).collect())
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Polynomial(
            self.0
                .iter()
                .rev()
                .zip(rhs.0)
                .map(|(x, y)| *x * y)
                .collect(),
        )
    }
}

/*
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().map(|num|std::char::from_digit(num as u32, 10));
        write!(f,format!(self.0.map.) )
    }
}
*/

/*
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
*/

//pub fn crt()

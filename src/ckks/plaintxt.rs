use num::Complex;
use std::ops::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Plaintxt(Vec<f64>);

impl Plaintxt {
    pub fn new(vec: Vec<f64>) -> Plaintxt {
        Plaintxt(vec)
    }

    pub fn eval(&self, root: Complex<f64>) -> Complex<f64> {
        let mut sum = Complex::new(0f64, 0f64);
        for i in 0..self.0.len() {
            sum = sum + root.powu(i as u32) * self.0[i];
        }
        sum
    }

    /// size function calculate below
    ///|h| = (a_0^2 + a_1^2 + ... + a_n^2 )^1/2
    pub fn size(&self) -> Complex<f64> {
        self.0
            .iter()
            .fold(Complex::new(0f64, 0f64), |sum, a| sum + a.powi(2))
            .powf(0.5)
    }
}

impl Add for Plaintxt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Plaintxt(self.0.iter().zip(rhs.0).map(|(x, y)| *x + y).collect())
    }
}

impl Mul for Plaintxt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let d = self.0.len() + rhs.0.len() - 1;
        let mut poly = vec![0f64; d];

        for k in 0..=d {
            for i in 0..=k {
                if self.0.len() <= i || rhs.0.len() <= k - i {
                    continue;
                }
                poly[k] = poly[k] + self.0[i] * rhs.0[k - i];
            }
        }

        Plaintxt(poly)
    }
}

/*
use std::fmt;
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

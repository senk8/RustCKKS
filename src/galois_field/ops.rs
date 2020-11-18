use super::*;
use std::ops::*;

impl Add for GF
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value,self.modulo) 
    }
}

impl Sub for GF
{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        if self.modulo != other.modulo {panic!("This operation is not enclose in its field.")}
        Self::new(self.value - other.value,self.modulo) 
    }
}

impl Mul for GF
{
    type Output = Self;
    fn mul(self, other: Self) -> Self{
        Self::new(self.value * other.value,self.modulo) 
    }
}

impl Div for GF
{
    type Output = Self;
    fn div(self, other: Self) -> Self{
        self * other.inv()
    }
}

impl AddAssign for GF {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for GF {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for GF {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for GF {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}


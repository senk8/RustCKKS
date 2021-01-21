use std::ops::*;
use super::Element;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GFElm{
    pub value : Element,
    pub modulo : Element,
}

impl Add for GFElm
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value,self.modulo) 
    }
}

impl Sub for GFElm
{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        if self.modulo != other.modulo {panic!("This operation is not enclose in its field.")}
        Self::new(self.value - other.value + self.modulo,self.modulo) 
    }
}

impl Mul for GFElm
{
    type Output = Self;
    fn mul(self, other: Self) -> Self{
        Self::new(self.value * other.value + self.modulo,self.modulo) 
    }
}

impl Div for GFElm
{
    type Output = Self;
    fn div(self, other: Self) -> Self{
        self * other.inv()
    }
}

impl AddAssign for GFElm {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for GFElm {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for GFElm {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for GFElm {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl GFElm{
    pub fn new(value:Element,modulo:Element) -> Self
    {
        Self {value: value % modulo,modulo: modulo}
    }

    pub fn get(self)-> Element{
        self.value
    }

    pub fn set<T>(&mut self,x:T)-> ()
    where Element: From<T>
    {
        self.value=Element::from(x);
    }
}

impl GFElm{
    pub fn inv(self) -> Self{
        let mut a = self.value;
        let mut b = self.modulo;
        let (mut x1,mut y1) = (1,0);
        let (mut x2,mut y2) = (0,1);
        while a%b!=0 {
            let q:Element=a/b;
            let r:Element=a%b;
            let nx:Element=x1-q*x2;
            let ny:Element=y1-q*y2;
       
            a=b;
            b=r;
            x1=x2;
            y1=y2;
            x2=nx;
            y2=ny;
        }
        Self::new(x2+self.modulo,self.modulo)
    }
}


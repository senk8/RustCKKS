use super::*;
use std::cmp::{max,min};
impl GF{
    pub fn new(value:Numeral,modulo:Numeral) -> Self
    {
        Self {value: value % modulo,modulo: modulo}
    }

    pub fn get(self)-> Numeral{
        self.value
    }

    pub fn set<T>(&mut self,x:T)-> ()
    where Numeral: From<T>
    {
        self.value=Numeral::from(x);
    }
}

impl GF{
    pub fn inv(self) -> Self{
        let mut a = self.value;
        let mut b = self.modulo;
        let (mut x,mut y) = (1,0);
        let (mut nx,mut ny) = (0,1);
        while a%b!=0 {
            let q:Numeral=a/b;
            let r:Numeral=a%b;
            let tmpx:Numeral=x-q*nx;
            let tmpy:Numeral=y-q*ny;
       
            a=b;b=r;
            x=nx;y=ny;
            nx=tmpx;ny=tmpy;
        }
        return Self::new(nx,self.modulo);
    }
}
use super::*;

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
        let (mut x1,mut y1) = (1,0);
        let (mut x2,mut y2) = (0,1);
        while a%b!=0 {
            let q:Numeral=a/b;
            let r:Numeral=a%b;
            let nx:Numeral=x1-q*x2;
            let ny:Numeral=y1-q*y2;
       
            a=b;
            b=r;
            x1=x2;
            y1=y2;
            x2=nx;
            y2=ny;
        }
        return Self::new(x2+self.modulo,self.modulo);
    }
}
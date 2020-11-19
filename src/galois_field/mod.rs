mod ops;
pub mod prim;

pub type Numeral = i64;
//pub const MOD :Numeral = 1000_000_007;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GF{
    value : Numeral,
    modulo : Numeral,
}

#[derive(Debug)]
pub struct FieldGenerator{
    modulo: Numeral,
}

impl FieldGenerator{
    pub fn new<T>(x:T) -> Self
    where Numeral: From<T>,
    {
        let modulo = Numeral::from(x);
        Self {modulo: modulo}
    }
    pub fn elm<T>(&self,x:T) -> GF
    where Numeral: From<T>,
    {
        GF::new(Numeral::from(x), self.modulo) 
    }
}

pub fn gf_init(prime: Numeral)-> Box<dyn Fn(i64) -> GF>
{
    if false {
        panic!("Error: First Argument `prime` is not prime or prime")
    }
    Box::new(move |value|GF::new(value,prime))
}

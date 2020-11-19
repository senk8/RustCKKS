use super::*;

/// implemantation for Field Generator
/// usage :
/// ```
/// let gf5 = FieldGenerator::new(5);
/// println!("{:?}",gf5);
/// println!("{:?}",gf5.elm(3));
/// ```
/// 
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


mod ops;
pub mod prim;
pub mod archive;

pub type Numeral = i64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GF{
    pub value : Numeral,
    pub modulo : Numeral,
}

pub fn gf_init(prime: Numeral)-> Box<dyn Fn(i64) -> GF>
{
    if false {
        panic!("Error: First Argument `prime` is not prime or prime")
    }

    Box::new(move |value|GF::new(value,prime))
}

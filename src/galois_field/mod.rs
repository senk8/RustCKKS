mod ops;
pub mod prim;
pub mod archive;
pub mod model;

pub type Numeral = i64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GF{
    pub value : Numeral,
    pub modulo : Numeral,
}
/// 
/// ```
/// let gf97 = gf_init(97);
/// let x = gf97(11);
/// let y = gf97(53);
/// println!("{:?}",x); // GF { value: 11, modulo: 97 }
/// println!("{:?}",x+y); // GF { value: 64, modulo: 97 }
/// println!("{:?}",x/x); // GF { value: 1, modulo: 97 }
/// ```
/// 
pub fn gf_init(prime: Numeral)-> Box<dyn Fn(i64) -> GF>
{
    if false {
        panic!("Error: First Argument `prime` is not prime or prime")
    }

    Box::new(move |value|GF::new(value,prime))
}

pub fn gcd(a:Numeral,b:Numeral)->Numeral
where Numeral:Clone
{
    if a%b == 0 { a }
    else{ gcd(b,a%b) }
}



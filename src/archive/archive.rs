use super::*;

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
pub fn gf_init(prime: Element)-> Box<dyn Fn(i64) -> GFElm>
{
    if false {
        panic!("Error: First Argument `prime` is not prime or prime")
    }

    Box::new(move |value|GFElm::new(value,prime))
}


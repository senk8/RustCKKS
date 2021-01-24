mod basic;
mod field;
use crate::basic::Polynomial;
use crate::field::gf_context::GFContext;

fn main() {
    let gf5 = GFContext::new(5);
    println!("{}", gf5.elm(3));
    println!("{}", gf5.elm(2));

    let gf53 = GFContext::new(53);
    let x = gf53.elm(97);

    let gf97 = GFContext::new(97);
    let x = gf97.elm(11);
    let y = gf97.elm(53);
    println!("{}", x);
    println!("{}", x + y);
    println!("{}", x - y);
    println!("{}", x / x);
    println!("{}", x.inv());

    let ans = Polynomial::new(vec![1, 2], &gf5) * Polynomial::new(vec![2, 3], &gf5);

    //println!("{:?}",x*x.inv());
    //println!("{:?}",y*y.inv());
    //println!("{:?}",-52%53);
}

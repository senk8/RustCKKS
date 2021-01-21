mod galois_field;
use crate::galois_field::gf_elm::GFElm;
use crate::galois_field::gf_context::GFContext;

fn main() {
    let gf5 = GFContext::new(5isize);
    println!("{:?}",gf5.elm(3isize));
    println!("{:?}",gf5.elm(2isize));

    let gf53 = GFContext::new(53isize);
    let x = gf53.elm(97isize);

    let gf97 = GFContext::new(97isize);
    let x = gf97.elm(11isize);
    let y = gf97.elm(53isize);
    println!("{:?}",x);
    println!("{:?}",x+y);
    println!("{:?}",x/x);
    println!("{:?}",x.inv());

    //println!("{:?}",x*x.inv());
    //println!("{:?}",y*y.inv());
    //println!("{:?}",-52%53);
}
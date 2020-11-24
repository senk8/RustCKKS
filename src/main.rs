extern crate galois_field;
use galois_field::galois_field::gf_init;

fn main() {
    let gf5 = gf_init(5);
    println!("{:?}",gf5(3));
    println!("{:?}",gf5(2));

    let gf53 = gf_init(53);
    let x = gf53(97);

    let gf97 = gf_init(97);
    let x = gf97(11);
    let y = gf97(53);
    println!("{:?}",x);
    println!("{:?}",x+y);
    println!("{:?}",x/x);

    //println!("{:?}",x*x.inv());
    //println!("{:?}",y*y.inv());
    //println!("{:?}",-52%53);
}
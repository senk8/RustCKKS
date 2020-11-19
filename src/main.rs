extern crate galois_field;
use galois_field::galois_field::{GF,FieldGenerator};

#[test]
fn test_const()
{
    //let modint = |x|GField::new(x,1000_000_007);
    let x = GField::new(1i64);
    let y = GField::new(2i64);
    let z = GField::new(0i64);
    let w = GField::new(1000_000_007i64);
    assert_eq!(x,x);
    assert_eq!(z,w);
    assert_eq!(x.get(),1);
    assert_eq!(y.get(),2);
    assert_eq!(z.get(),0);
    assert_eq!(w.get(),0);
}


#[test]
fn test_inv()
{
    let x = GField::new(53i64);
    assert_eq!(11,x.inv().val);
}

#[test]
fn test_add()
{
    let mut x = GField::new(1i64);
    let mut y = GField::new(2i64);
    let mut z = GField::new(0i64);
    let mut w = GField::new(1000_000_007i64);
    assert_eq!(y,x+x);
    assert_eq!(x,x+z);
    assert_eq!(x,x+w);
    assert_eq!(x,w+x);

    x += y;
    assert_eq!(x.val,3);
}

#[test]
fn test_sub()
{
    let x = GField::new(1i64);
    let y = GField::new(2i64);
    let z = GField::new(0i64);
    let w = GField::new(1000_000_007i64);
    assert_eq!(z,x-x);
    assert_eq!(x,x-z);
    assert_eq!(x,y-x);
    assert_eq!(z-x,x-y);
    assert_eq!(x,x-w);
}

#[test]
fn test_mul()
{
    let x = GField::new(1i64);
    let y = GField::new(2i64);
    let z = GField::new(0i64);
    let w = GField::new(1000_000_007i64);
    assert_eq!(y,x*y);
    assert_eq!(y,y*x);
    assert_eq!(z,x*z);
    assert_eq!(4,(y*y).val);
    assert_eq!(w,w*w);
}

#[test]
fn test_div()
{
    let x = GF::new(1i64);
    let y = GF::new(2i64);
    let z = GF::new(0i64);
    let w = GF::new(1000_000_007i64);
    assert_eq!(x,x/x);
    assert_eq!(x,y/x);
}

fn main() {
    let field = FieldGenerator::new(53);
    println!("{:?}",field);
    println!("{:?}",field.elm(10));
    println!("{:?}",field.elm(12));
    //let x = GField::new(97,53);
    //println!("{:?}",x.inv());
}
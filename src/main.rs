extern crate galois_field;
use galois_field::galois_field::gf_init;

#[test]
fn test_const()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(97i64);

    assert_eq!(x,x);
    assert_eq!(z,w);
}


#[test]
fn test_inv()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let y = gf97(11i64);
    assert_eq!(y,x.inv());
}

#[test]
fn test_add()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(x,x+z);
    assert_eq!(x,z+x);
    assert_eq!(w,y+y);

    y += y;
    assert_eq!(y,w);
}

#[test]
fn test_sub()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(y,w-y);
    assert_eq!(x,x-z);
    assert_eq!(z,y-y);

    y -= y;
    assert_eq!(y,z);
}

#[test]
fn test_mul()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(w,y*w);
    assert_eq!(y,y*y);
    assert_eq!(z,y*z);

    y *= w;
    assert_eq!(w,y);
}

#[test]
fn test_div()
{
    let gf97=gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(y,x/x);
    assert_eq!(w,w/y);

    y /= y;
    assert_eq!(y,y);
}

fn main() {
    let gf5 = gf_init(5);
    println!("{:?}",gf5(3));
    println!("{:?}",gf5(2));

    let gf53 = gf_init(53);
    let x = gf53(97);

    let gf97 = gf_init(97);
    let y = gf97(53);

    println!("{:?}",x*x.inv());
    println!("{:?}",y*y.inv());
    println!("{:?}",x/x);
    println!("{:?}",-52%53);
}
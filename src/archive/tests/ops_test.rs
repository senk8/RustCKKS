extern crate galois_field;
use galois_field::galois_field::gf_init;

fn test_add() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(x, x + z);
    assert_eq!(x, z + x);
    assert_eq!(w, y + y);

    y += y;
    assert_eq!(y, w);
}

fn test_sub() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(y, w - y);
    assert_eq!(x, x - z);
    assert_eq!(z, y - y);

    y -= y;
    assert_eq!(y, z);
}

fn test_mul() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(w, y * w);
    assert_eq!(y, y * y);
    assert_eq!(z, y * z);

    y *= w;
    assert_eq!(w, y);
}

#[test]
fn test_div() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let mut y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(2i64);

    assert_eq!(y, x / x);
    assert_eq!(w, w / y);

    y /= y;
    assert_eq!(y, y);
}

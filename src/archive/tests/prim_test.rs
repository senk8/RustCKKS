extern crate galois_field;
use galois_field::galois_field::gf_init;

fn test_const() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let y = gf97(1i64);
    let z = gf97(0i64);
    let w = gf97(97i64);

    assert_eq!(x, x);
    assert_eq!(z, w);
}

fn test_inv() {
    let gf97 = gf_init(97i64);

    let x = gf97(53i64);
    let y = gf97(11i64);
    assert_eq!(y, x.inv());
}

pub type Element = u64;
pub mod gf_context;
pub mod gf_elm;
pub mod ring_context;
pub mod ring_elm;

#[allow(dead_code)]
pub fn gcd(a: isize, b: isize) -> isize {
    if a % b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(dead_code)]
pub fn lcm(a: isize, b: isize) -> isize {
    (a * b) / gcd(a, b)
}

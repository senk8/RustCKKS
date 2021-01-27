pub type Element = u64;
pub mod cyc;
pub mod gf_context;
pub mod gf_elm;
pub mod poly;
pub mod ring_context;
pub mod ring_elm;

pub fn gcd(a: isize, b: isize) -> isize {
    if a % b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: isize, b: isize) -> isize {
    (a * b) / gcd(a, b)
}

use ndarray_linalg::types::c64;
const E: f64 = 1e-10;

pub fn cyc(m: usize) -> c64 {
    let rad = 2f64 / (m as f64) * std::f64::consts::PI;
    c64::new(rad.cos(), rad.sin())
}

pub fn complex_eq(c1: c64, c2: c64) -> bool {
    let re_diff = c1.re - c2.re;
    let im_diff = c1.im - c2.im;
    if re_diff < E && im_diff < E {
        true
    } else {
        false
    }
}

use num::Complex;

const E: f64 = 1e-10;

pub fn cyc(m: usize) -> Complex<f64> {
    let rad = 2f64 / (m as f64) * std::f64::consts::PI;
    Complex::new(rad.cos(), rad.sin())
}

pub fn complex_eq(c1: Complex<f64>, c2: Complex<f64>) -> bool {
    let re_diff = c1.re - c2.re;
    let im_diff = c1.im - c2.im;
    if re_diff < E && im_diff < E {
        true
    } else {
        false
    }
}

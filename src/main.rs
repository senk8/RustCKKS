mod ckks;
mod prelude;
use ckks::encoder::CKKSEncoder;
use ndarray::{Array1, Array2,array};
use ndarray_linalg::types::c64;


fn main() {
    let encoder = CKKSEncoder::new(8);
    let ptxt = encoder.encode(array![c64::new(1f64,0f64),c64::new(2f64,0f64),c64::new(3f64,0f64),c64::new(4f64,0f64)]);
    let res = encoder.decode(ptxt);
    return ();
}

mod tests {
    use crate::ckks::encoder::CKKSEncoder;
    use crate::ckks::plaintxt::Plaintxt;

    use crate::prelude::gf_context::GFContext;
    use crate::prelude::ring_context::RingContext;

    use ndarray_linalg::types::c64;
    use ndarray::{Array1, Array2,array};

    const E: f64 = 1e-10;

    fn complex_eq(c1: c64, c2: c64) -> bool {
        let re_diff = c1.re - c2.re;
        let im_diff = c1.im - c2.im;

        if re_diff < E && im_diff < E {
            true
        } else {
            false
        }
    }

    fn cyc(m: usize) -> c64 {
        let rad = 2f64 / (m as f64) * std::f64::consts::PI;
        c64::new(rad.cos(), rad.sin())
    }
    
    #[test]
    fn test_unity(){
        let encoder = CKKSEncoder::new(8);
        let x = encoder.get_unity();

        assert_eq!(true, complex_eq(c64::new(-1f64, 0f64), x * x * x * x));
    }

    #[test]
    fn test_gf() {
        let gf5 = GFContext::new(5);

        assert_eq!(gf5.elm(0), gf5.elm(5));
        assert_eq!(gf5.elm(1), gf5.elm(21));
        assert_eq!(gf5.elm(2), gf5.elm(17));
        assert_eq!(gf5.elm(3), gf5.elm(73));
        assert_eq!(gf5.elm(4), gf5.elm(104));

        let gf97 = GFContext::new(97);
        let x = gf97.elm(11);
        let y = gf97.elm(53);

        assert_eq!(gf97.elm(64), x + y);
        assert_eq!(gf97.elm(42), y - x);
        assert_eq!(gf97.elm(1), x * y);
        assert_eq!(y, x.inv());
        assert_eq!(gf97.elm(1), x / x);
    }

    #[test]
    fn test_ring() {
        let ring5 = RingContext::new(5);

        assert_eq!(ring5.elm(0), ring5.elm(5));
        assert_eq!(ring5.elm(1), ring5.elm(21));
        assert_eq!(ring5.elm(2), ring5.elm(17));
        assert_eq!(ring5.elm(3), ring5.elm(73));
        assert_eq!(ring5.elm(4), ring5.elm(104));
    }

    /*
    #[test]
    fn test_poly() {
        let one = c64::new(1f64, 0f64);
        let two = c64::new(2f64, 0f64);
        let three = c64::new(3f64, 0f64);
        let poly1 = Plaintxt::new(vec![1f64, 2f64]);
        let poly2 = Plaintxt::new(vec![1f64, 2f64]);
        let poly3 = Plaintxt::new(vec![1f64, 4f64, 4f64]);

        assert_eq!(true, complex_eq(c64::new(3f64, 0f64), poly1.eval(one)));
        assert_eq!(true, complex_eq(c64::new(5f64, 0f64), poly1.eval(two)));
        assert_eq!(
            true,
            complex_eq(c64::new(7f64, 0f64), poly1.eval(three))
        );
        assert_eq!(poly3, poly1 * poly2);
    }

    #[test]
    fn test_pi_inverse() {
        let encoder = CKKSEncoder::new(8);

        let x = array![c64::new(1f64,0f64),c64::new(0f64,0f64)];
        let res = array![c64::new(1f64,0f64),c64::new(0f64,0f64),c64::new(0f64,0f64),c64::new(1f64,0f64)];

        assert_eq!(true, complex_eq(res,encoder.pi_inverse(x)));
    }
    */
}

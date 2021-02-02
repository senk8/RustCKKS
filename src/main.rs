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

    #[test]
    fn test_cyc(){
        use crate::ckks::encoder::CKKSEncoder;
        use crate::prelude::cyc::complex_eq;
        use ndarray_linalg::types::c64;

        let encoder = CKKSEncoder::new(4);
        let x = encoder.get_unity();

        assert_eq!(true, complex_eq(c64::new(1f64, 0f64), x * x * x * x))
    }

    #[test]
    fn test_gf() {
        use crate::prelude::gf_context::GFContext;
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
        use crate::prelude::ring_context::RingContext;

        let ring5 = RingContext::new(5);

        assert_eq!(ring5.elm(0), ring5.elm(5));
        assert_eq!(ring5.elm(1), ring5.elm(21));
        assert_eq!(ring5.elm(2), ring5.elm(17));
        assert_eq!(ring5.elm(3), ring5.elm(73));
        assert_eq!(ring5.elm(4), ring5.elm(104));
    }

    #[test]
    fn test_cyclotomic() {
        use crate::prelude::cyc::{complex_eq, cyc};
        use ndarray_linalg::types::c64;
        let m = 4;
        let one = c64::new(1f64, 0f64);
        let cyclotomic = cyc(m);
        assert_eq!(
            true,
            complex_eq(one, cyclotomic * cyclotomic * cyclotomic * cyclotomic)
        );
    }

    #[test]
    fn test_poly() {
        use crate::ckks::plaintxt::Plaintxt;
        use crate::prelude::cyc::{complex_eq, cyc};
        use ndarray_linalg::types::c64;

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
    fn test_unity() {
        use crate::prelude::cyc::complex_eq;
        use ndarray_linalg::types::c64;

        let encoder = crate::ckks::encoder::CKKSEncoder::new(4);
        let x = encoder.get_unity();

        println!("{:?}", x*x*x*x);
        assert_eq!(true, complex_eq(c64::new(1f64, 0f64), x * x * x * x));
    }
}

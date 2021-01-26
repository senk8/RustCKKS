mod prelude;
use crate::prelude::poly::Polynomial;

fn main() {
    //let ans = Polynomial::new(vec![1, 2], &gf5) * Polynomial::new(vec![2, 3], &gf5);
    return ();
}

mod tests {
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
    fn test_cyclotomic(){
        use crate::prelude::cyc::{cyc,complex_eq};
        use num::Complex;
        let m = 4;
        let one = Complex::new(1f64,0f64);
        let cyclotomic = cyc(m);
        assert_eq!(true,complex_eq(one,cyclotomic * cyclotomic * cyclotomic * cyclotomic));
    }
}

pub mod galois_field{
    use std::ops::*;

    type Numeral = u64;
    const MOD :Numeral = 10000000007;
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct GField{
        pub val : Numeral,
    }

    impl GField{
        pub fn new<T>(x:T) -> Self
        where Numeral: From<T>,
        {
            Self {val: Numeral::from(x) % MOD}
        }
    }

    impl Add for GField
    {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self::new(self.val + other.val) 
        }
    }

    impl Sub for GField
    {
        type Output = Self;
        fn sub(self, other: Self) -> Self{
            Self::new(self.val - other.val) 
        }
    }

    #[test]
    fn test_const()
    {
        let x = GField::new(1u64);
        let y = GField::new(0u64);
        let z = GField::new(1000_000_0007u64);
        assert_eq!(x,x);
        assert_eq!(y,z);
    }

    #[test]
    fn test_add()
    {
        let x = GField{val:1};
        let y = GField{val:2};
        let z = GField{val:0};
        let w = GField{val:10^10+7};
        assert_eq!(y,x+x);
        assert_eq!(y,y+z);
        assert_eq!(x,x+w);
        assert_eq!(x,w+x);
    }

    #[test]
    fn test_sub()
    {
        let x = GField{val:1};
        let y = GField{val:2};
        let z = GField{val:0};
        let w = GField{val:10^10+7};
        assert_eq!(z,x-x);
        assert_eq!(y,y-z);
        assert_eq!(x,x-w);
    }

}
use self::galois_field::*;

fn main() {
    let x = GField{val:1};
    let y = GField{val:1};
    let z = GField{val:1};
}
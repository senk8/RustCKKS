
const MOD :usize = 10^10+7;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GField{
    val : usize,
}

impl Add for GField
{
    fn add(&self, other: GField) -> GField {
        self.val += other.val;
        self.val %= MOD;
        return self.val;
    }
}

#[test]
fn test_add()
{
    assert_eq!(2,GField{value:1}+GField({value:1}));
}

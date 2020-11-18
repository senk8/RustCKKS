//mod ops;
pub mod prim;

pub type Numeral = i64;
//pub const MOD :Numeral = 1000_000_007;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct GF{
    value : Numeral,
    modulo : Numeral,
}

// TODO 初期化の方法
// 1. GFと要素でクラスを分ける。
//    利点　簡単
//    欠点　名前が衝突する
// 2. クロージャを返すようにする。
//    利点　名前が衝突しない
//　　 欠点　newで初期化できない
// 3. staticを使う
//    利点　newでの初期化が可能であり、衝突もしない
//    欠点　危険

// TODO 逆元計算
// 逆元の計算が順番によっては失敗する。

pub fn gf_init(prime:Numeral)-> fn(Numeral) -> GF
{
    if false {
        panic!("Error: First Argument `prime` is not prime or prime")
    }
    let constructor: fn(Numeral) -> GF = |value|GF::new(value,prime);
    return constructor;
}

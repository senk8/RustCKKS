use super::plaintxt::Plaintxt;
//use ndarray_linalg::types::c64;

use nalgebra::{DVector,DMatrix};
use nalgebra::Complex;
type c64 = Complex<f64>;

pub struct CKKSEncoder {
    m: usize,
    unity: c64,
}

impl CKKSEncoder {
    /*
    CKKSは整数多項式環の性質を利用するが、多くの場合で実数ベクトルが入力されるのでエンコードが必要。

    N =  2^k

    encode :: C mod N/2 -> m(X) ∈ R = ℤ[X]/(X^N+1)

    M = 2N 次元の円分多項式　ΦM(X)=XN+1.

    */

    /*
        make vandermode
        1,1,1,1,1
        x^1,x^1,x^1,x^1
    */

    pub fn new(m: usize) -> CKKSEncoder {
        let unity =
            (2f64 * std::f64::consts::PI * c64::i() / c64::new(m as f64, 0f64)).exp();
        CKKSEncoder { m, unity }
    }

    pub fn get_unity(&self) -> c64 {
        self.unity
    }

    fn vandermonde(&self) -> DMatrix<c64> {
        let n: usize = self.m / 2;
        let mut mat = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            for j in 0..n {
                mat.push(root.powu(j as u32));
            }
        }

        let res = DMatrix::from_vec(n, n, mat);
        res 
    }

    pub fn encode(&self,z:DVector<c64>)->Plaintxt
    {
        let vand = self.vandermonde();
        let coeffs = vand.try_inverse().unwrap() * z;
        Plaintxt::new(coeffs)
    }
    

    pub fn decode(&self,poly: Plaintxt)->DVector<c64>
    {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

        DVector::from_vec(z)
    }

    /*
    pub fn pi(self,z:Array1<c64>)->Array1<c64>
    {
        //H->C^N/2
        let n = self.m / 4;
        z[..n]
    }

    pub fn pi_inverse(self,z:Array1<c64>)->Array1<c64>
    {
       //C^N/2 を共役で拡張
       /*
        1. zを反転してz'にする
        2. z'の要素を共役に変換する
        3. zとz'を結合する
       */
    }
    */
}

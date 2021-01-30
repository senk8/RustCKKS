use super::plaintxt::Plaintxt;
use ndarray::{Array, Array1, Array2};
use num::Complex;

pub struct CKKSEncoder {
    m: usize,
    unity: Complex<f64>,
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
            (2f64 * std::f64::consts::PI * Complex::i() / Complex::new(m as f64, 0f64)).exp();
        CKKSEncoder { m, unity }
    }

    pub fn get_unity(&self) -> Complex<f64> {
        self.unity
    }

    fn vandermonde(&self) -> Array2<Complex<f64>> {
        let n: usize = self.m / 2;
        let mut mat = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            for j in 0..n {
                mat.push(root.powu(j as u32));
            }
        }

        Array::from_shape_vec((n, n), mat).unwrap()
    }

    /*
    pub fn encode(self,z:Array1<Complex<f64>>)->Plaintxt
    {
        let A = self.vandermonde();
        let coeffs = linalg.solve(A,z);

        Plaintxt::new(coeffs)
    }

    pub fn decode(self,poly: Plaintxt)->Array1<Complex<f64>>
    {
        let N = self.m / 2;
        let mut z = vec![];

        for i in 0..N {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

        Array::from_vec(z)
    }
    */
}

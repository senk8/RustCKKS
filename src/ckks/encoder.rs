use super::plaintxt::Plaintxt;
use ndarray::{Array, Array1, Array2};
use ndarray::prelude::*;
use ndarray_linalg::*;
use ndarray_linalg::types::c64;

pub struct CKKSEncoder {
    m: usize,
    unity: c64,
    //sigma_r_basis: Array1<c64>,
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
            (2f64 *  c64::i() / c64::new(m as f64, 0f64) * std::f64::consts::PI).exp();
        //self.create_sigma_r_basis();
        //self.scale
        CKKSEncoder { m, unity }
    }

    

    pub fn get_unity(&self) -> c64 {
        self.unity
    }

    fn vandermonde(&self) -> Array2<c64> {
        let n: usize = self.m / 2;
        let mut mat = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            for j in 0..n {
                mat.push(root.powu(j as u32));
            }
        }

        let res = Array::from_shape_vec((n, n), mat).unwrap();
        res 
    }

    pub fn encode(&self,z:Array1<c64>)->Plaintxt
    {
        let vand = self.vandermonde();
        let coeffs = vand.solve_into(z).unwrap();
        Plaintxt::new(coeffs)
    }
    

    pub fn decode(&self,poly: Plaintxt)->Array1<c64>
    {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

        Array::from_vec(z)
    }

    /*
    fn create_sigma_R_basis(&self){
        """Creates the basis (sigma(1), sigma(X), ..., sigma(X** N-1))."""
        self.basis = Array1::new(self.vandermonde(self.unity, self.M)).transpose()
    }

    pub fn pi(self,z:Array1<c64>)->Array1<c64>
    {
        let n = self.m / 4;

        /* H->C^N/2 ベクトルを半分にする*/
        z[..n]
    }

    pub fn pi_inverse(self,z:Array1<c64>)->Array1<c64>
    {
        let zd = z.reverse();//1. zを反転してz'にする
        let zd_conjugate = zd;//2. z'の要素を共役に変換する

        concate(zd,zd_conjugate);//3. zとz'を結合する
    }
    */
}

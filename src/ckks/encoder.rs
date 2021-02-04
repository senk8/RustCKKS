use super::plaintxt::Plaintxt;
use error::LinalgError;
use ndarray::prelude::*;
use ndarray::{s, Array, Array1, Array2};
use ndarray_linalg::types::c64;
use ndarray_linalg::*;

pub struct CKKSEncoder {
    m: usize,
    unity: c64,
    basis: Array2<c64>,
}

impl CKKSEncoder {
    pub fn new(m: usize) -> CKKSEncoder {
        let unity = (2f64 * c64::i() / c64::new(m as f64, 0f64) * std::f64::consts::PI).exp();
        let basis = CKKSEncoder::vandermonde(m, unity).t().to_owned();
        //self.scale
        CKKSEncoder { m, unity, basis }
    }

    fn vandermonde(m: usize, unity: c64) -> Array2<c64> {
        let n: usize = m / 2;
        let mut mat = vec![];

        for i in 0..n {
            let root = unity.powu(2 * i as u32 + 1);
            for j in 0..n {
                mat.push(root.powu(j as u32));
            }
        }

        let res = Array::from_shape_vec((n, n), mat).unwrap();
        res
    }

    pub fn get_unity(&self) -> c64 {
        self.unity
    }

    pub fn get_basis(&self) -> Array2<c64> {
        self.basis.clone()
    }
}

impl CKKSEncoder {
    pub fn encode(&self, z: Array1<c64>) -> Result<Plaintxt,LinalgError> {
        let vand = CKKSEncoder::vandermonde(self.m, self.unity);
        let coeffs = vand.solve_into(z)?;
        Ok(Plaintxt::new(coeffs))
    }

    pub fn decode(&self, poly: Plaintxt) -> Result<Array1<c64>,LinalgError> {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

       
        Ok(Array::from_vec(z))
    }

    pub fn pi(self, z: Array1<c64>) -> Array1<c64> {
        let n = self.m / 4;

        /* H->C^N/2 ベクトルを半分にする*/
        z.slice(s![..n]).to_owned()
    }

    /*
    pub fn pi_inverse(self,z:Array1<c64>)->Array1<c64>
    {
        let zd = z.slice(s![..;-1]);//1. zを反転してz'にする
        let zd_conjugate = zd.map(|x|x.conj());//2. z'の要素を共役に変換する

        let x = zd.iter();
        let y = zd_conjugate.iter();//3. zとz'を結合する

        Array::from_vec(x.chain(y).collect::Vec<c64>())
    }
    */
}

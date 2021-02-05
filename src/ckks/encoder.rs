use core::f64;

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
    /// encode: $\sigma(\mathbf{R})->\mathcal{R}$
    /// solve following equation:
    /// $ z = A^-1 x $
    /// 
    /// where A is vandermonde matrix. 
    /// 
    pub fn encode(&self, z: Array1<c64>) -> Result<Plaintxt,LinalgError> {
        let vand = CKKSEncoder::vandermonde(self.m, self.unity);
        let coeffs = vand.solve_into(z)?;
        Ok(Plaintxt::new(coeffs))
    }

    /// decode: \mathcal{R}->\sigma(\mathcal{R})$
    /// calculate x .
    /// x = A z
    /// 
    /// Calculate x by assigning root into A, evaluating it, and taking the product with z.
    /// A is vandermonde matrix. $z \in \mathcal{R},x \in \mathcal{\sigma{R}}$
    /// 
    pub fn decode(&self, poly: Plaintxt) -> Result<Array1<c64>,LinalgError> {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }
       
        // TODO: enable us to detect isize::MAX < res.len()
        Ok(Array::from_vec(z))
    }

    pub fn pi(&self, z: &Array1<c64>) -> Array1<c64> {
        let n = self.m / 4;

        /* H->C^N/2 ベクトルを半分にする*/
        z.slice(s![..n]).to_owned()
    }

    pub fn pi_inverse(&self,z: &Array1<c64>)->Array1<c64>
    {
        let zd = z.slice(s![..;-1]).to_owned();//1. zを反転してz'にする
        let zd_conjugate = zd.map(|x|x.conj());//2. z'の要素を共役に変換する

        let mut res = vec![];
        for i in z.into_iter().chain(zd_conjugate.into_iter()) {
            res.push(*i)
        }

       // TODO: enable us to detect isize::MAX < res.len()
       Array::from_vec(res)
    }
}

impl CKKSEncoder {
    /// compute
    /// 
    /// $<z,bi>/|b_i|^2$ 
    /// 
    /// 
    fn compute_basis_coordinate(&self,z:Array1<c64>)->Array1<f64>{
        let mut tmp = vec![];
        for i in 0..self.basis.len(){
            let bi=self.basis.row(i).to_owned();
            let bb=bi.map(|b|b.conj());
            let zb =z.dot(&bb);
            let bsize = bi.dot(&bi);
            let zi = zb / bsize;
            tmp.push(zi.re);
        }
        Array1::from_vec(tmp)
    }

    fn coordinates_wise_rrounding(&self,coordinates:Array1<f64>)->Array1<i64>{
        use rand::prelude::*;
        use rand::distributions::weighted::WeightedIndex;
        let round_coordinates= coordinates - coordinates.map(|x|x.floor());

        let f = round_coordinates.map(|c|{
            let choices = [c,1.0-c];
            let weights = [1.0-c,c-1.0];
            let mut rng = rand::thread_rng();
            let dist = WeightedIndex::new(&weights).unwrap();
            choices[dist.sample(&mut rng)]
        });

        let rounded_coordinates = coordinates - f;
        round_coordinates.map(|x|*x as i64)
    }

    pub fn into_integer_basis(&self,z:Array1<c64>)->Array1<i64>{
        let real_coordinates = self.compute_basis_coordinate(z);
        let rounded_coordinates = self.coordinates_wise_rrounding(real_coordinates);
        self.basis.t().dot(&rounded_coordinates)
    }

}

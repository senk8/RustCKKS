use core::f64;

use super::plaintxt::Plaintxt;
use error::LinalgError;
use ndarray::{s, Array, Array1, Array2};
use ndarray_linalg::types::c64;
use ndarray_linalg::*;

pub struct CKKSEncoder {
    m: usize,
    unity: c64,
    basis: Array2<c64>,
    scale: usize,
}

impl CKKSEncoder {
    pub fn new(m: usize, scale: usize) -> CKKSEncoder {
        let unity = (2f64 * c64::i() / c64::new(m as f64, 0f64) * std::f64::consts::PI).exp();
        let basis = CKKSEncoder::vandermonde(m, unity).t().to_owned();
        CKKSEncoder {
            m,
            unity,
            basis,
            scale,
        }
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
    pub fn encode(&self, z: Array1<c64>) -> Result<Plaintxt, LinalgError> {
        /* C^n/2 -> H */
        let pi_z = self.pi_inverse(&z);

        /* keep a presicion */
        let scaled = pi_z.mapv(|x| self.scale as f64 * x);

        /* H -> sigma(R) s.t. sigma(R) \in H */
        let rounded = self.into_integer_basis(scaled);

        /* sigma(R) -> R . It is inverting sigma */
        let coeffs = self.sigma_inverse(rounded)?;

        /* coeffs contains c64 values. You have to convert it into integers. */
        Ok(Plaintxt::new(
            coeffs.0.mapv(|x| c64::new(x.re.round(), x.im)),
        ))
    }

    ///
    /// decode :: R ->  sigma(R) -> C^n/2
    ///
    pub fn decode(&self, p: Plaintxt) -> Result<Array1<c64>, LinalgError> {
        /*  divide p coeffs by scale  .*/
        let rescaled: Plaintxt = p / self.scale;

        /*  R -> sigma(R) .*/
        let rounded = self.sigma(rescaled)?;

        /* sigma(R) -> C^n/2 */
        Ok(self.pi(&rounded))
    }

    /// decode: \mathcal{R}->\sigma(\mathcal{R})$
    /// calculate x .
    /// x = A z
    ///
    /// Calculate x by assigning root into A, evaluating it, and taking the product with z.
    /// A is vandermonde matrix. $z \in \mathcal{R},x \in \mathcal{\sigma{R}}$
    ///
    pub fn sigma(&self, poly: Plaintxt) -> Result<Array1<c64>, LinalgError> {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

        // TODO: enable us to detect isize::MAX < res.len()
        Ok(Array::from(z))
    }
    /// encode: $\sigma(\mathbf{R})->\mathcal{R}$
    /// solve following equation:
    /// $ z = A^-1 x $
    ///
    /// where A is vandermonde matrix.
    ///
    pub fn sigma_inverse(&self, z: Array1<c64>) -> Result<Plaintxt, LinalgError> {
        let vand = CKKSEncoder::vandermonde(self.m, self.unity);
        let coeffs = vand.solve_into(z)?;
        Ok(Plaintxt::new(coeffs))
    }

    pub fn pi(&self, z: &Array1<c64>) -> Array1<c64> {
        let n = self.m / 4;

        /* H->C^N/2 ベクトルを半分にする*/
        z.slice(s![..n]).to_owned()
    }

    pub fn pi_inverse(&self, z: &Array1<c64>) -> Array1<c64> {
        let zd = z.slice(s![..;-1]).to_owned(); //1. zを反転してz'にする
        let zd_conjugate = zd.mapv(|x| x.conj()); //2. z'の要素を共役に変換する

        let mut res = vec![];
        for i in z.into_iter().chain(zd_conjugate.into_iter()) {
            res.push(*i)
        }

        // TODO: enable us to detect isize::MAX < res.len()
        Array::from(res)
    }

    pub fn into_integer_basis(&self, z: Array1<c64>) -> Array1<c64> {
        let real_coordinates = CKKSEncoder::compute_basis_coordinate(&self.basis, z);
        let rounded_coordinates = CKKSEncoder::coordinates_wise_rrounding(real_coordinates);
        self.basis.t().dot(&rounded_coordinates)
    }
}

impl CKKSEncoder {
    /// perform below computation.
    ///
    /// $<z,bi>/|b_i|^2$
    ///
    ///
    fn compute_basis_coordinate(basis: &Array2<c64>, z: Array1<c64>) -> Array1<c64> {
        let mut tmp = vec![];
        for i in 0..basis.nrows() {
            let bi = basis.row(i).to_owned();
            let bi_conj = bi.mapv(|x| x.conj());
            let mut zi = z.dot(&bi_conj) / bi.dot(&bi_conj);
            zi.im = 0.;
            tmp.push(zi);
        }
        Array1::from(tmp)
    }

    fn coordinates_wise_rrounding(coordinates: Array1<c64>) -> Array1<c64> {
        use rand::distributions::weighted::WeightedIndex;
        use rand::prelude::*;
        let decimals = &coordinates - &coordinates.mapv(|x| c64::new(x.re.floor(), x.im));
        /*
           The closer the integer, the higher the probability of rounding.
           For example, if c=4.6, the probability of rounding to 4 is 0.6 and that of rounding to 5 is 0.4.
        */
        let subtract_decimals = decimals.mapv(|c| {
            let choices = [c, c - 1.];
            let weights = [1. - c.re, c.re];
            let mut rng = rand::thread_rng();
            let dist = WeightedIndex::new(&weights).unwrap();
            choices[dist.sample(&mut rng)]
        });
        /*
           coordinates rounded to close integer.
           For example, coordinate = [1.2,1.7,2.6] and suppose that subtract_decimals = [-0.8,0.7,-0.4].
           You can perform coordinate-wise subtraction.
           coordinate - subtract_decimals = [2.0,1.0,3.0].
        */
        coordinates - subtract_decimals
    }
}

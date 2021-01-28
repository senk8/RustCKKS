
struct CKKSEncoder{
    m: usize,
    xi: Compex<f64>
}


impl CKKSContext {
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

    fn vandermonde(root:Complex<f64>)->Vec<Vec<Complex<f64>>>{
        let N = self.m / 2;
        let mut mat = vec![];

        for i in 1..N {
            let root = self.xi.powi(2 * i + 1);
            let mut row = vec![];

            for j in 1..N {
                row.push(root.powi(j));
            }

            mat.push(row)
        }
    }

    pub fn encode(self,z:Vec<Complex<f64>>)->Polynomial
    {
        let A = self.vandermonde(self.xi);
        let coeffs = linalg.solve(A,z);
                
        Polynomial::new(coeffs)
    }

    pub fn decode(self,poly: Polynomial)->Vec<Complex<f64>>
    {
        let n = self.m / 2;
        let mut res = vec![];

        for i in 0..n {
            let root = self.xi.powi(2 * i + 1); 
            let res = poly.eval(root);
            res.push(res);
        }

        res
    }
}

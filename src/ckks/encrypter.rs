use super::plaintxt::Plaintxt;
use super::cihpertxt::Ciphertxt;

struct CKKSEncrypter {
    publick_key:(Plaintxt,Plaintxt),
    secret_key:Plaintxt
}


impl CKKSEncrypter {
   
    /// a: choose on uniform with random
    /// s: small secret polynomial
    /// e: small noisy polynomial
    pub fn new(mu:f64,sigma:f64)->CKKSEncrypter{
        use rand::distributions::Distribution;
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let gaussian = Normal::new(mu,sigma.sqrt()).unwrap();

        let a = rng.gen_range(0..10000);
        let noise = gaussian.sample(&mut rng);

        let publick_key = (-a+secret*noise,a);
        let secret_key = secret;
        CKKSEncrypter { publick_key,secret_key }
    }

    pub fn encrypt(&self,ptxt:Plaintxt)->Ciphertxt{
        Ciphertxt::new(ptxt+publick_key.0,publick_key.1)
    }

    pub fn decrypt(&self,ctxt:Ciphertxt)->Plaintxt{
        Plaintxt::new(ctxt.0+ctxt.1*secret_key)
    }
}
use bls12_381::G1Affine;
use bls12_381::G2Affine;
use bls12_381::Scalar as Fr;
use bls12_381::Bls12 as S;
use rand::Rng;
use rand::RngCore;
use rand::thread_rng;

trait Field {
    fn random(rng: impl RngCore) -> Self;
}

impl Field for Fr {
    fn random(mut rng: impl RngCore) -> Self {
        let mut buf = [0; 64];
        rng.fill_bytes(&mut buf);
        Self::from_bytes_wide(&buf)
    }
}

#[derive(Debug)]
pub struct Polynomial{
    coef: Vec<Fr>,
}

impl Polynomial{
    pub fn new(coef: Vec<Fr>) -> Self {
        Self {coef}
    }
    
    pub fn random(length: usize) -> Self {
        let mut rng = thread_rng();
        // Use the random function from Field trait
        let coef: Vec<Fr> = (0..length)
            .map(|_| Field::random(&mut rng)) 
            .collect();

        Self { coef }
    }

}




#[cfg(test)]
mod test{

    use super::*;

    #[test]
    pub fn random(){

        let rand_poly = Polynomial::random(10);
        print!("{:?}", rand_poly);
    
    }
    
}
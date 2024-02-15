use std::cmp::max;
use std::cmp::min;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::SubAssign;
use std::vec;

use bls12_381::G1Affine;
use bls12_381::G1Projective;
use bls12_381::G2Affine;
use bls12_381::G2Projective;
use bls12_381::Scalar as Fr;
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


pub struct trustedSetup {
    pub g1: Vec<G1Projective>,
    pub g2: Vec<G2Projective>
}


pub fn generatorTS( nDegree: usize) -> trustedSetup {
    let generator_g1 = G1Affine::generator();
    let generator_g2 = G2Affine::generator();

    let randomSecret: Fr = Field::random(&mut thread_rng());
    println!("randomsecret:{}",randomSecret);
    
    let mut srsG1: Vec<G1Projective> = Vec::new(); 
    let mut srsG2: Vec<G2Projective> = Vec::new(); 

    let mut exponent = Fr::one();

    for _ in 0..nDegree{
        srsG1.push(generator_g1.mul(exponent));
        srsG2.push(generator_g2.mul(exponent));
        exponent *= randomSecret;
    }

    trustedSetup{g1: srsG1, g2:srsG2}
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    pub fn ts(){
        let ts = generatorTS(3);
        println!("srsG1:{:?}",ts.g1);
        println!("srsG2:{:?}",ts.g2);
    }
}
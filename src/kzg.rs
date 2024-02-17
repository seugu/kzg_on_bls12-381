use crate::polynomials;
use crate::polynomials::Polynomial;
use crate::ts::trustedSetup;
use bls12_381::Scalar as Fr;
use core::num;
use std::ops::Mul;

use bls12_381::G1Affine;
use bls12_381::G1Projective;
use bls12_381::G2Affine;
use bls12_381::G2Projective;
use rand::RngCore;
use rand::thread_rng;

#[derive(Debug)]
pub struct Proof{
    pub poly_commitment: G1Projective,
    pub quon_commitment: G1Projective,
    pub random_challange: Fr
}

pub fn prove(poly: Polynomial, challenge: Fr, ts: &trustedSetup) -> Proof {
    let evaluation = poly.eval(&challenge);
    let mut numerator = poly.clone();
    numerator.coef[0] -= challenge;
    let denominator = Polynomial::new(vec![challenge.neg(), Fr::one()]);
    let quotient_polynomial = Polynomial::div(&numerator, &denominator);

    let pcommitment = Polynomial::commitG1(&poly, &ts.g1);
    let qcommitment = Polynomial::commitG1(&quotient_polynomial, &ts.g1);

    Proof { 
        poly_commitment: pcommitment, 
        quon_commitment: qcommitment, 
        random_challange: challenge
    }

}

/* pub fn verify(proof: Proof, challenge: Fr, ts: &trustedSetup) -> bool {
    let generator_g2 = G2Projective::generator();
    let challange_g2 = generator_g2 * challenge;

    let s_sub_challage = ts.g2[1] - challange_g2;

    //left pair
    let left_pair = proof.c


} */

#[cfg(test)]
mod test{

    use crate::ts;

    use super::*;

    #[test]
    pub fn prove_test(){
        let ts = ts::generatorTS(8);
        let randPoly = Polynomial::random(5);

        let challenge = Fr::from(3);
        
        let proof = prove(randPoly, challenge, &ts);
        println!("proof:{:?}",proof);

    }
}


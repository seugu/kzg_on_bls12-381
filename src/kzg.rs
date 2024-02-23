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
use bls12_381::Bls12;



#[derive(Debug)]
pub struct Proof{
    pub poly_commitment: G1Affine,
    pub quon_commitment: G1Affine,
    pub random_challenge: Fr
}

pub fn prove(poly: Polynomial, challenge: Fr, ts: &trustedSetup) -> Proof {
    let evaluation = poly.eval(&challenge);
    let mut numerator = poly.clone();
    numerator.coef[0] -= evaluation;
    let denominator = Polynomial::new(vec![challenge.neg(), Fr::one()]);
    let quotient_polynomial = Polynomial::div(&numerator, &denominator);

    let pcommitment = Polynomial::commitG1(&poly, &ts.g1);
    let qcommitment = Polynomial::commitG1(&quotient_polynomial, &ts.g1);


    Proof { 
        poly_commitment: pcommitment, 
        quon_commitment: qcommitment, 
        random_challenge: evaluation
    }

}

pub fn verify(proof: Proof, challenge: Fr, ts: &trustedSetup) -> bool {
    let generator_g2 = G2Projective::generator();
    let challange_g2 = generator_g2 * challenge;

    let s_sub_challenge = ts.g2[1] - challange_g2;

    //left pair
    let left_pair = bls12_381::pairing(&proof.quon_commitment.into(), &s_sub_challenge.into()); 
    
    let generator_g1 = G1Projective::generator();
    let evaluation_g1 = generator_g1 * proof.random_challenge;
    let poly_commit_sub_y = proof.poly_commitment - evaluation_g1;


    let right_pair = bls12_381::pairing(&poly_commit_sub_y.into(), &generator_g2.into());
    println!("{}\n",left_pair);
    println!("{}",right_pair);


    left_pair == right_pair

}

#[cfg(test)]
mod test{

    use crate::ts;
    use super::*;

    #[test]
    pub fn prove_test(){
        let ts = ts::generatorTS(20);
        let rand_poly = Polynomial::random(3);

        let challenge = Fr::from(3);
        
        let proof = prove(rand_poly, challenge, &ts);
        //println!("proof:{:?}",proof);

        let verify = verify(proof, challenge, &ts);
        println!("{}",verify);
    }
}


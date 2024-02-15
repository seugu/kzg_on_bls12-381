use std::cmp::max;
use std::cmp::min;
use std::ops::Add;
use std::ops::AddAssign;
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

#[derive(Debug,PartialEq)]
pub struct Polynomial{
    coef: Vec<Fr>,
}

impl Polynomial{
    // p(x) = [a_0,a_1,a_2,...,a_n ] -> a_0 * x^0 + a_1 * x^1 + a_2 * x^2 + ... + a_n * x^n
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

    pub fn eval(&self, input: &Fr) -> Fr {
        let mut eval = Fr::zero();
    
        for (i, coef) in self.coef.iter().enumerate() {
            let ith_step = coef * input.pow(&[i as u64; 4]);
            eval.add_assign(&ith_step);
        }

        eval
    }

    pub fn add(poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {

        let final_degree = max(poly1.coef.len(), poly2.coef.len());
        let mut final_coef: Vec<Fr> = Vec::new();

        for i in 0..final_degree{
            let mut coef1 = if i < poly1.coef.len() {poly1.coef[i]} else {Fr::zero()};
            let coef2 = if i < poly2.coef.len() {poly2.coef[i]} else {Fr::zero()};
            coef1.add_assign(coef2);
            final_coef.push(coef1);
        }

        Polynomial::new(final_coef)
    }

    pub fn sub(poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {

        let final_degree = max(poly1.coef.len(), poly2.coef.len());
        let mut final_coef: Vec<Fr> = Vec::new();

        for i in 0..final_degree{
            let mut coef1 = if i < poly1.coef.len() {poly1.coef[i]} else {Fr::zero()};
            let coef2 = if i < poly2.coef.len() {poly2.coef[i]} else {Fr::zero()};
            coef1.sub_assign(coef2);
            final_coef.push(coef1);
        }

        Polynomial::new(final_coef)
    }

    pub fn mul(poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let final_degree = poly1.coef.len() + poly2.coef.len()-1;
        let mut final_coef: Vec<Fr> = vec![Fr::zero(); final_degree];

        for i in 0..poly1.coef.len(){
            for j in 0..poly2.coef.len(){
                final_coef[i+j] += poly1.coef[i] * poly2.coef[j];
            }
        }

        Polynomial::new(final_coef)

    }

    // calculates poly1 = poly2 * quotient + residue
    pub fn div(poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let poly1coef: Vec<Fr> = poly1.coef.iter().rev().cloned().collect();

        let mut final_coef: Vec<Fr> = vec![poly1coef[0]];

        for i in 0..poly1.coef.len()-2 {
            final_coef.push(poly1coef[i+1] - final_coef[i] * poly2.coef[0]);
        }

        final_coef.reverse();
        Polynomial::new(final_coef)
    }

    pub fn commitG1(poly: &Polynomial, srsG1: &Vec<G1Projective>) -> G1Projective {
        let mut commitment = G1Projective::default(); 
        for i in 0..poly.coef.len(){
            commitment += srsG1[i] * poly.coef[i];
        } 
        commitment
    }

    pub fn commitG2(poly: &Polynomial, srsG1: &Vec<G2Projective>) -> G2Projective {
        let mut commitment = G2Projective::default(); 
        for i in 0..poly.coef.len(){
            commitment += srsG2[i] * poly.coef[i];
        } 
        commitment
    }
}




#[cfg(test)]
mod test{

    use std::{vec, ops::SubAssign};

    use super::*;

    #[test]
    pub fn random(){
        let rand_poly = Polynomial::random(2);
        print!("{:?}", rand_poly);
    
    }

    #[test]
    pub fn eval_1(){
        let a0 = Fr::from(2);
        let a1 = Fr::from(1);

        let vec_coef = vec![a0,a1];

        let poly = Polynomial::new(vec_coef);

        assert_eq!(poly.eval(&Fr::from(1)), Fr::from(3));
    }
    #[test]
    pub fn add_1(){
        let a0 = Fr::from(2);
        let a1 = Fr::from(1);

        let vec_coef = vec![a0,a1];
        // pol1(x) = 2 * x^0 + 1 * x^1  
        let pol1 = Polynomial::new(vec_coef);

        let b0 = Fr::from(3);
        let b1 = Fr::from(12);
        let b2 = Fr::from(10);

        let vec_coef2 = vec![b0,b1,b2];
        // pol2(x) = 3 * x^0 + 12 * x^1 + 10 * x^2
        let pol2 = Polynomial::new(vec_coef2);

        let c0 = Fr::from(5);
        let c1 = Fr::from(13);
        let c2 = Fr::from(10);

        let vec_coef3 = vec![c0,c1,c2];
        // Pol1(x) + Pol2(x) = 
        // result_pol(x)     = 5 * x^0 + 13 * x^1 + 10 * x^2
        let result_pol = Polynomial::new(vec_coef3);


        assert_eq!(result_pol, Polynomial::add(&pol1, &pol2));

    }

    #[test]
    pub fn sub_1(){
        let a0 = Fr::from(2);
        let a1 = Fr::from(1);

        let vec_coef = vec![a0,a1];
        // pol1(x) = 2 * x^0 + 1 * x^1  
        let pol1 = Polynomial::new(vec_coef);

        let b0 = Fr::from(3);
        let b1 = Fr::from(12);
        let b2 = Fr::from(10);

        let vec_coef2 = vec![b0,b1,b2];
        // pol2(x) = 3 * x^0 + 12 * x^1 + 10 * x^2
        let pol2 = Polynomial::new(vec_coef2);

        let c0 = Fr::from(1);
        let c1 = Fr::from(11);
        let c2 = Fr::from(10);

        let vec_coef3 = vec![c0,c1,c2];
        // Pol2(x) -  Pol1(x) = 
        // result_pol(x)     = 1 * x^0 + 11 * x^1 + 10 * x^2
        let result_pol = Polynomial::new(vec_coef3);


        assert_eq!(result_pol, Polynomial::sub(&pol2, &pol1));
    }
    
    #[test]
    pub fn mul(){
        let a0 = Fr::from(2);
        let a1 = Fr::from(1);

        let vec_coef = vec![a0,a1];
        // pol1(x) = 2 * x^0 + 1 * x^1  
        let pol1 = Polynomial::new(vec_coef);

        let b0 = Fr::from(3);
        let b1 = Fr::from(12);
        let b2 = Fr::from(10);

        let vec_coef2 = vec![b0,b1,b2];
        // pol2(x) = 3 * x^0 + 12 * x^1 + 10 * x^2
        let pol2 = Polynomial::new(vec_coef2);

        let c0 = Fr::from(6);
        let c1 = Fr::from(27);
        let c2 = Fr::from(32);
        let c3 = Fr::from(10);

        let vec_coef3 = vec![c0,c1,c2,c3];
        // Pol2(x) -  Pol1(x) = 
        // result_pol(x)     = 6 * x^0 + 27 * x^1 + 32 * x^2 + 10 * x^3 
        let result_pol = Polynomial::new(vec_coef3);


        assert_eq!(result_pol, Polynomial::mul(&pol2, &pol1));
    }

     #[test]
    pub fn div(){
        let a0 = Fr::from(2);
        let a1 = Fr::from(1);

        let vec_coef = vec![a0,a1];
        // pol1(x) = x + 2 
        let pol1 = Polynomial::new(vec_coef);

        let b0 = Fr::from(3);
        let b1 = Fr::from(12);
        let b2 = Fr::from(6);

        let vec_coef2 = vec![b0,b1,b2];
        // pol2(x) = 6x2 + 12x + 3 
        let pol2 = Polynomial::new(vec_coef2);

        let c0 = Fr::zero();
        let c1 = Fr::from(6);

        let vec_coef3 = vec![c0,c1];
        // Pol2(x) -  Pol1(x) = 
        // result_pol(x)     = 6 * x^0 + 27 * x^1 + 32 * x^2 + 10 * x^3 
        let result_pol = Polynomial::new(vec_coef3);


        assert_eq!(result_pol, Polynomial::div(&pol2, &pol1));
    }
    
    
}
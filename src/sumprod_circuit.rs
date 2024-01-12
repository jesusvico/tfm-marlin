use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable, LinearCombination},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct SumProdCircuit<F: PrimeField> {
    pub x: F,
    pub t: usize,
}

// Constructor for SumProdCircuit
impl<F: PrimeField> BenchCircuit<F> for SumProdCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        SumProdCircuit { 
            x: <F>::rand(rng),  
            t: rounds + 3 
        }
    }

    fn get_result(&self) -> F {
        let mut v: Vec<F> = Vec::new();

        v.push(self.x);
        for _ in 0..(self.t - 1) {
            let mut r: F = self.x - self.x; // This is 0
            for value in &v {
                r = r + value;
            }
            v.push(r * r);
        }
        v.last().unwrap().clone()
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for SumProdCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        /*
            w = [x, x^2, ..., x^t]
            out = x^t
            The minimum t is 3 because of this issue:
            https://github.com/arkworks-rs/marlin/issues/79
        */

        // Allocate witness x
        let x_val = self.x;
        let x = cs.new_witness_variable(|| Ok(x_val))?;

        let mut v_val: Vec<F> = Vec::new();
        let mut v: Vec<Variable> = Vec::new();

        v_val.push(x_val);
        v.push(x);
        for _ in 0..(self.t - 2) {
            let mut r_val = x_val - x_val; // This is 0
            for value in &v_val {
                r_val = r_val + value;
            }
            r_val = r_val * r_val;
            v_val.push(r_val);

            let r = cs.new_witness_variable(|| Ok(r_val))?;
            let mut lc: LinearCombination<F> = lc!();
            for value in &v {
                lc = lc + value;
            }
            v.push(r);

            cs.enforce_constraint(lc.clone(), lc, lc!() + r)?;
        }

        let mut out_val = x_val - x_val; // This is 0
        for value in &v_val {
            out_val = out_val + value;
        }
        out_val = out_val * out_val;

        let out = cs.new_input_variable(|| Ok(out_val))?;
        let mut lc: LinearCombination<F> = lc!();
        for value in &v {
            lc = lc + value;
        }
        cs.enforce_constraint(lc.clone(), lc, lc!() + out)?;

        Ok(())
    }
}
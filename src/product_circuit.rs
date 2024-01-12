use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct ProductCircuit<F: PrimeField> {
    pub x: F,
    pub t: usize,
}

// Constructor for ProductCircuit
impl<F: PrimeField> BenchCircuit<F> for ProductCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        ProductCircuit { 
            x: <F>::rand(rng),  
            t: rounds + 3 
        }
    }

    fn get_result(&self) -> F {
        let mut r = self.x;
        for _ in 0..(self.t - 1) {
            r = r * self.x;
        }
        r
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for ProductCircuit<F> {
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


        let mut new_val;
        let mut new;
        let mut old_val = x_val;
        let mut old = x;

        for _ in 0..(self.t - 2) {
            new_val = old_val * x_val;
            new = cs.new_witness_variable(|| Ok(new_val))?;
            cs.enforce_constraint(lc!() + old, lc!() + x, lc!() + new)?;

            old_val = new_val;
            old = new;
        }

        new_val = old_val * x_val;
        let out = cs.new_input_variable(|| Ok(new_val))?;
        cs.enforce_constraint(lc!() + old, lc!() + x, lc!() + out)?;

        Ok(())
    }
}
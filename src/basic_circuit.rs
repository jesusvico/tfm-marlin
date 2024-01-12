use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct BasicCircuit<F: PrimeField> {
    pub a: F,
    pub b: F,
    pub num_constraints: usize,
}

// Constructor for BasicCircuit
impl<F: PrimeField> BenchCircuit<F> for BasicCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        BasicCircuit { 
            a: <F>::rand(rng), 
            b: <F>::rand(rng), 
            num_constraints: rounds + 2 
        }
    }

    fn get_result(&self) -> F {
        self.a * self.b
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for BasicCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| Ok(self.a))?;
        let b = cs.new_witness_variable(|| Ok(self.b))?;
        let c = cs.new_input_variable(|| {
            Ok(self.a * self.b)
        })?;

        for _ in 0..self.num_constraints {
            cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        }

        Ok(())
    }
}
use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct DenseCircuit<F: PrimeField> {
    pub a: F,
    pub b: F,
    pub num_constraints: usize,
}

// Constructor for DenseCircuit
impl<F: PrimeField> BenchCircuit<F> for DenseCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        DenseCircuit { 
            a: <F>::rand(rng), 
            b: <F>::rand(rng), 
            num_constraints: rounds + 2 
        }
    }

    fn get_result(&self) -> F {
        (self.a + self.b) * (self.a + self.b)
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for DenseCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| Ok(self.a))?;
        let b = cs.new_witness_variable(|| Ok(self.b))?;
        let c = cs.new_input_variable(|| {
            Ok((self.a + self.b) * (self.a + self.b))
        })?;

        for _ in 0..self.num_constraints {
            cs.enforce_constraint(lc!() + a + b, lc!() + a + b, lc!() + c)?;
        }

        Ok(())
    }
}
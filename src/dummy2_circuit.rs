use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct Dummy2Circuit<F: PrimeField> {
    pub a: F,
    pub b: F,
    pub s: F,
    pub num_constraints: usize,
}

// Constructor for DummyCircuit
impl<F: PrimeField> BenchCircuit<F> for Dummy2Circuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        Dummy2Circuit { 
            a: <F>::rand(rng), 
            b: <F>::rand(rng), 
            s: <F>::rand(rng), 
            num_constraints: rounds + 2 
        }
    }

    fn get_result(&self) -> F {
        (self.a + self.s) * self.b
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for Dummy2Circuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| Ok(self.a))?;
        let b = cs.new_witness_variable(|| Ok(self.b))?;
        let s = cs.new_witness_variable(|| Ok(self.s))?;
        let c = cs.new_input_variable(|| {
            Ok((self.a + self.s) * self.b)
        })?;

        for _ in 0..self.num_constraints {
            cs.enforce_constraint(lc!() + a + s, lc!() + b, lc!() + c)?;
        }

        Ok(())
    }
}
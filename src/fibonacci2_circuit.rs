use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;

#[derive(Copy, Clone)]
pub struct Fibonacci2Circuit<F> {
    pub x: F,
    pub t: usize
}

// Constructor for Fibonacci2Circuit
impl<F: PrimeField> BenchCircuit<F> for Fibonacci2Circuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
        Fibonacci2Circuit {   
            x: <F>::from(0 as u32),
            t: rounds + 3
        }
    }

    fn get_result(&self) -> F {
        let mut a = <F>::from(0 as u32);
        let mut b = <F>::from(1 as u32);

        for _ in 0..(self.t - 1) {
            let c = b;
            b = a + b;
            a = c;
        }

        b
    }
} 

impl<F: PrimeField> ConstraintSynthesizer<F> for Fibonacci2Circuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        /*
            w = [x, x^2, ..., x^t]
            out = x^t
            The minimum t is 3 because of this issue:
            https://github.com/arkworks-rs/marlin/issues/79
        */

        let mut v_val: Vec<F> = Vec::new();
        let mut v: Vec<Variable> = Vec::new();

        // Add a 0 to the witness
        v_val.push(<F>::from(0 as u32));
        v.push(cs.new_witness_variable(|| Ok(v_val.last().unwrap().clone()))?);

        // Add a 1 to the witness
        let one_val = <F>::from(1 as u32);
        let one = cs.new_witness_variable(|| Ok(one_val))?;
        v_val.push(one_val);
        v.push(one);

        for i in 0..(self.t - 2) {
            // The next value is the sum of the two lasts
            v_val.push(v_val[v_val.len() - 1] + v_val[v_val.len() - 2]);
            v.push(cs.new_witness_variable(|| Ok(v_val.last().unwrap().clone()))?);


            cs.enforce_constraint(lc!() + v[1], lc!() + v[v.len() - 2] + v[v.len() - 3], lc!() + v.last().unwrap())?;
        }

        let out_val = v_val[v_val.len() - 1] + v_val[v_val.len() - 2];
        let out =  cs.new_input_variable(|| Ok(out_val))?;

        cs.enforce_constraint(lc!() + one, lc!() + v[v.len() - 1] + v[v.len() - 2], lc!() + out);

        Ok(())
    }
}
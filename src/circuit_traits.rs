use ark_ff::PrimeField;
use ark_std::rand::RngCore;

pub trait BenchCircuit<F: PrimeField> {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self;
    fn get_result(&self) -> F;
}
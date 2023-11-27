use ark_std::rand::RngCore;

pub trait NewRandomCircuit {
    fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self;
}
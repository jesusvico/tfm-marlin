// For benchmark, run:
//     cargo bench -- --nocapture
// where N is the number of threads you want to use (N = 1 for single-thread).

use ark_ff::PrimeField;
use ark_marlin::Marlin;
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_mnt4_298::{Fr as MNT4Fr, MNT4_298};
use ark_mnt4_753::{Fr as MNT4BigFr, MNT4_753};
use ark_mnt6_298::{Fr as MNT6Fr, MNT6_298};
use ark_mnt6_753::{Fr as MNT6BigFr, MNT6_753};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{sonic_pc::SonicKZG10, marlin_pc::MarlinKZG10};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::UniformRand;
use blake2::Blake2s;

const NUM_PROVE_REPETITIONS: usize = 1;

#[derive(Copy)]
struct DummyCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub num_variables: usize,
    pub num_constraints: usize,
}

impl<F: PrimeField> Clone for DummyCircuit<F> {
    fn clone(&self) -> Self {
        DummyCircuit {
            a: self.a.clone(),
            b: self.b.clone(),
            num_variables: self.num_variables.clone(),
            num_constraints: self.num_constraints.clone(),
        }
    }
}

impl<F: PrimeField> ConstraintSynthesizer<F> for DummyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

            Ok(a * b)
        })?;

        for _ in 0..(self.num_variables - 3) {
            let _ = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        }

        for _ in 0..self.num_constraints - 1 {
            cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        }

        cs.enforce_constraint(lc!(), lc!(), lc!())?;

        Ok(())
    }
}

macro_rules! marlin_prove_bench {
    (
        $bench_name:ident, 
        $bench_field:ty, 
        $bench_pairing_engine:ty,
        $bench_arithmetization:ty,
        $bench_constraints:expr
    ) => {
        let num_variables: usize = 10;
        let num_constraints: usize = $bench_constraints; //65536;

        let rng = &mut ark_std::test_rng();
        let c = DummyCircuit::<$bench_field> {
            a: Some(<$bench_field>::rand(rng)),
            b: Some(<$bench_field>::rand(rng)),
            num_variables,
            num_constraints,
        };

        let srs = Marlin::<
            $bench_field,
            MarlinKZG10<$bench_pairing_engine, DensePolynomial<$bench_field>>,
            Blake2s,
        >::universal_setup(num_constraints, num_variables, 3 * num_constraints, rng)
        .unwrap();
        let (pk, _) = Marlin::<
            $bench_field,
            MarlinKZG10<$bench_pairing_engine, DensePolynomial<$bench_field>>,
            Blake2s,
        >::index(&srs, c)
        .unwrap();

        let start = std::time::Instant::now();

        for _ in 0..NUM_PROVE_REPETITIONS {
            let _ = Marlin::<
                $bench_field,
                MarlinKZG10<$bench_pairing_engine, DensePolynomial<$bench_field>>,
                Blake2s,
            >::prove(&pk, c.clone(), rng)
            .unwrap();
        }

        println!(
            "per-constraint proving time for {}: {} ns/constraint",
            stringify!($bench_pairing_engine),
            start.elapsed().as_nanos() / NUM_PROVE_REPETITIONS as u128 / 65536u128
        );
    };
}

fn main() {
    marlin_prove_bench!(bls, BlsFr, Bls12_381, SonicKZG10, 100);
    marlin_prove_bench!(mnt4, MNT4Fr, MNT4_298, SonicKZG10, 100);
    marlin_prove_bench!(mnt6, MNT6Fr, MNT6_298, SonicKZG10, 100);
    marlin_prove_bench!(mnt4big, MNT4BigFr, MNT4_753, SonicKZG10, 100);
    marlin_prove_bench!(mnt6big, MNT6BigFr, MNT6_753, SonicKZG10, 100);
}

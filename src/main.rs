use clap::{Parser, arg, command};

mod printers;
mod dummy_circuit;
mod circuit_traits;

use printers::*;

use ark_relations::r1cs::{ConstraintSystem, ConstraintSynthesizer};
use dummy_circuit::DummyCircuit;
use circuit_traits::NewRandomCircuit;

use ark_marlin::Marlin;

use ark_bls12_381::{Fr as BlsFr, Bls12_381};
use ark_mnt4_298::{Fr as MNT4Fr, MNT4_298};
use ark_mnt4_753::{Fr as MNT4BigFr, MNT4_753};
use ark_mnt6_298::{Fr as MNT6Fr, MNT6_298};
use ark_mnt6_753::{Fr as MNT6BigFr, MNT6_753};

use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{marlin_pc::MarlinKZG10, sonic_pc::SonicKZG10};

use blake2::Blake2s;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Circuit to test
    #[arg(short, long, default_value = "dummy")]
    circuit: String,

    /// Number of rounds
    #[arg(short, long, default_value_t = 1)]
    rounds: usize,

    // Field used by the system
    #[arg(short, long, default_value = "BlsFr")]
    field: String,

    // Arithmetization used by the system
    #[arg(short, long, default_value = "MarlinKZG10")]
    arithmetization: String,
}

enum Circuits {
    Dummy,
    Hash
}

enum Fields {
    BlsFr,
    MNT4Fr,
    MNT4BigFr,
    MNT6Fr,
    MNT6BigFr
}

enum Arithmetizations {
    MarlinKZG10,
    SonicKZG10
}

macro_rules! bench_dummy {
    (
        $bench_circuit:ident,
        $bench_field:ty, 
        $bench_pairing_engine:ty,
        $bench_arithmetization:ident,
        $bench_rounds:expr
    ) => {

        print_info!(
            "Benchmarking system {} {} {} {}",
            stringify!($bench_circuit),
            stringify!($bench_field),
            stringify!($bench_pairing_engine),
            stringify!($bench_arithmetization)
        );

        let num_variables = 3;
        let num_constraints = $bench_rounds + 2;

        let rng = &mut ark_std::test_rng();
        let c = $bench_circuit::<$bench_field>::new_random(rng, $bench_rounds);

        // First show the number of constraints
        let cs = ConstraintSystem::<$bench_field>::new_ref();
        let _ = c.generate_constraints(cs.clone());
        print_info!(
            "Number of constraints: {}", 
            cs.num_constraints()
        );

        let srs = Marlin::<
            $bench_field,
            $bench_arithmetization<$bench_pairing_engine, DensePolynomial<$bench_field>>,
            Blake2s,
        >::universal_setup(num_constraints, num_variables, 3 * num_constraints, rng)
        .unwrap();

        let (pk, _) = Marlin::<
            $bench_field,
            $bench_arithmetization<$bench_pairing_engine, DensePolynomial<$bench_field>>,
            Blake2s,
        >::index(&srs, c)
        .unwrap();

        let start = std::time::Instant::now();

        let _prove = Marlin::<
            $bench_field,
            $bench_arithmetization<$bench_pairing_engine, DensePolynomial<$bench_field>>,
            Blake2s,
        >::prove(&pk, c.clone(), rng)
        .unwrap();

        print_info!(
            "Proving time: {}ms", 
            start.elapsed().as_millis()
        );
    }
}

fn main() {
    // Accessing command-line arguments
    let args = Args::parse();

    // Get the circuit
    let circuit = match args.circuit.as_str() {
        "dummy" => Circuits::Dummy,
        "hash" => Circuits::Hash,
        _ => print_panic!("Invalid circuit {}", args.circuit.as_str())
    };

    // Get the number of rounds
    let rounds = args.rounds;
    if rounds == 0 {
        print_panic("0 is not a valid number of rounds")
    }

    // Get the field
    let field = match args.field.as_str() {
        "BlsFr" => Fields::BlsFr,
        "MNT4Fr" => Fields::MNT4Fr,
        "MNT4BigFr" => Fields::MNT4BigFr,
        "MNT6Fr" => Fields::MNT6Fr,
        "MNT6BigFr" => Fields::MNT6BigFr,
        _ => print_panic!("Invalid field {}", args.field.as_str())
    };

    // Get the arithmetization
    let arithmetization = match args.arithmetization.as_str() { 
        "MarlinKZG10" => Arithmetizations::MarlinKZG10,
        "SonicKZG10" => Arithmetizations::SonicKZG10,
        _ => print_panic!("Invalid field {}", args.arithmetization.as_str())
    };

    // Execute the correct macro
    match (circuit, field, arithmetization) {
        (Circuits::Dummy, Fields::BlsFr, Arithmetizations::MarlinKZG10) => {
            bench_dummy!(DummyCircuit, BlsFr, Bls12_381, MarlinKZG10, rounds); 
        }
        (Circuits::Dummy, Fields::MNT4Fr, Arithmetizations::MarlinKZG10) => {
            bench_dummy!(DummyCircuit, MNT4Fr, MNT4_298, MarlinKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT4BigFr, Arithmetizations::MarlinKZG10) => {
            bench_dummy!(DummyCircuit, MNT4BigFr, MNT4_753, MarlinKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT6Fr, Arithmetizations::MarlinKZG10) => {
            bench_dummy!(DummyCircuit, MNT6Fr, MNT6_298, MarlinKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT6BigFr, Arithmetizations::MarlinKZG10) => {
            bench_dummy!(DummyCircuit, MNT6BigFr, MNT6_753, MarlinKZG10, rounds);
        }

        (Circuits::Dummy, Fields::BlsFr, Arithmetizations::SonicKZG10) => {
            bench_dummy!(DummyCircuit, BlsFr, Bls12_381, SonicKZG10, rounds); 
        }
        (Circuits::Dummy, Fields::MNT4Fr, Arithmetizations::SonicKZG10) => {
            bench_dummy!(DummyCircuit, MNT4Fr, MNT4_298, SonicKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT4BigFr, Arithmetizations::SonicKZG10) => {
            bench_dummy!(DummyCircuit, MNT4BigFr, MNT4_753, SonicKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT6Fr, Arithmetizations::SonicKZG10) => {
            bench_dummy!(DummyCircuit, MNT6Fr, MNT6_298, SonicKZG10, rounds);
        }
        (Circuits::Dummy, Fields::MNT6BigFr, Arithmetizations::SonicKZG10) => {
            bench_dummy!(DummyCircuit, MNT6BigFr, MNT6_753, SonicKZG10, rounds);
        }
        _ => print_panic("Invalid")
    }
}
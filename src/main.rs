use ark_ff::PrimeField;
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
use ark_poly_commit::{marlin_pc::MarlinKZG10, sonic_pc::SonicKZG10, PolynomialCommitment};

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

#[derive(Debug)]
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

fn bench
    <
        F: PrimeField,
        C: ConstraintSynthesizer<F> + NewRandomCircuit + Copy,
        A: PolynomialCommitment<F, DensePolynomial<F>>,
    >(rounds: usize) {

        print_info!("Benchmarking");

        let rng = &mut ark_std::test_rng();
        let c = C::new_random(rng, rounds);

        // First show the number of constraints
        let cs = ConstraintSystem::<F>::new_ref();
        let _ = c.clone().generate_constraints(cs.clone());
        print_info!(
            "Number of constraints: {}", 
            cs.num_constraints()
        );

        let srs = Marlin::<F, A, Blake2s,>
            ::universal_setup(1000, 10, 3 * 1000, rng)
            .unwrap();

        let (pk, _) = Marlin::<F, A, Blake2s,>
            ::index(&srs, c)
            .unwrap();

        let start = std::time::Instant::now();

        let _prove = Marlin::<F, A, Blake2s,>
            ::prove(&pk, c.clone(), rng)
            .unwrap();

        print_info!(
            "Proving time: {}ms", 
            start.elapsed().as_millis()
        );
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
            bench::<
                BlsFr, 
                DummyCircuit<BlsFr>, 
                MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>
            >(rounds); 
        }
        (Circuits::Dummy, Fields::MNT4Fr, Arithmetizations::MarlinKZG10) => {
            bench::<
                MNT4Fr, 
                DummyCircuit<MNT4Fr>, 
                MarlinKZG10<MNT4_298, DensePolynomial<MNT4Fr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT4BigFr, Arithmetizations::MarlinKZG10) => {
            bench::<
                MNT4BigFr, 
                DummyCircuit<MNT4BigFr>, 
                MarlinKZG10<MNT4_753, DensePolynomial<MNT4BigFr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT6Fr, Arithmetizations::MarlinKZG10) => {
            bench::<
                MNT6Fr, 
                DummyCircuit<MNT6Fr>, 
                MarlinKZG10<MNT6_298, DensePolynomial<MNT6Fr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT6BigFr, Arithmetizations::MarlinKZG10) => {
            bench::<
                MNT6BigFr, 
                DummyCircuit<MNT6BigFr>, 
                MarlinKZG10<MNT6_753, DensePolynomial<MNT6BigFr>>
            >(rounds);
        }

        (Circuits::Dummy, Fields::BlsFr, Arithmetizations::SonicKZG10) => {
            bench::<
                BlsFr, 
                DummyCircuit<BlsFr>, 
                SonicKZG10<Bls12_381, DensePolynomial<BlsFr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT4Fr, Arithmetizations::SonicKZG10) => {
            bench::<
                MNT4Fr, 
                DummyCircuit<MNT4Fr>, 
                SonicKZG10<MNT4_298, DensePolynomial<MNT4Fr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT4BigFr, Arithmetizations::SonicKZG10) => {
            bench::<
                MNT4BigFr, 
                DummyCircuit<MNT4BigFr>, 
                SonicKZG10<MNT4_753, DensePolynomial<MNT4BigFr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT6Fr, Arithmetizations::SonicKZG10) => {
            bench::<
                MNT6Fr, 
                DummyCircuit<MNT6Fr>, 
                SonicKZG10<MNT6_298, DensePolynomial<MNT6Fr>>
            >(rounds);
        }
        (Circuits::Dummy, Fields::MNT6BigFr, Arithmetizations::SonicKZG10) => {
            bench::<
                MNT6BigFr, 
                DummyCircuit<MNT6BigFr>, 
                SonicKZG10<MNT6_753, DensePolynomial<MNT6BigFr>>
            >(rounds);
        }
        _ => print_panic("Invalid")
    }
}
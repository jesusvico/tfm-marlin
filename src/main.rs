use clap::{Parser, arg, command};

mod printers;
mod circuit_traits;
mod dummy_circuit;
mod product_circuit;

use printers::*;

use ark_relations::r1cs::{ConstraintSystem, ConstraintSynthesizer, OptimizationGoal};
use dummy_circuit::DummyCircuit;
use product_circuit::ProductCircuit;
use circuit_traits::BenchCircuit;

use ark_marlin::Marlin;

use ark_poly_commit::marlin_pc::MarlinKZG10;
use ark_poly::univariate::DensePolynomial;
use blake2::Blake2s;

use ark_bls12_377::{Fr as Bls377Fr, Bls12_377};
use ark_bls12_381::{Fr as Bls381Fr, Bls12_381};
use ark_mnt4_298::{Fr as MNT4Fr, MNT4_298};
use ark_mnt4_753::{Fr as MNT4BigFr, MNT4_753};
use ark_mnt6_298::{Fr as MNT6Fr, MNT6_298};
use ark_mnt6_753::{Fr as MNT6BigFr, MNT6_753};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Circuit to test
    #[arg(short, long, default_value = "dummy")]
    system: String,

    /// Number of rounds
    #[arg(short, long, default_value_t = 1)]
    rounds: usize,

    // Field used by the system
    #[arg(short, long, default_value = "bls12_381")]
    curve: String,
}

macro_rules! bench {
    ($circuit:ident, $field:ty, $pairing_engine:ty, $rounds:expr) => {

        print_info!(
            "Benchmarking {} {}, rounds: {}",
            stringify!($circuit),
            stringify!($pairing_engine),
            $rounds
        );
        
        let rng = &mut ark_std::test_rng();
        let c = $circuit::<$field>::new_random(rng, $rounds);

        // Generate the constraint system without optimizations
        let cs = ConstraintSystem::<$field>::new_ref();
        cs.set_optimization_goal(OptimizationGoal::None);

        // Show the number of constraints
        cs.finalize();
        println!("{}", cs.is_satisfied().unwrap());
        let _ = c.clone().generate_constraints(cs.clone());
        print_info!(
            "Number of constraints: {}", 
            cs.num_constraints()
        );

        // Get the matrices
        let matrices = cs.to_matrices().unwrap();
        print_info!(
            "Non-zeros -  A: {:?}, B: {}, C: {}", 
            matrices.a_num_non_zero,
            matrices.b_num_non_zero,
            matrices.c_num_non_zero,
        );


        let mut x = 0;
        for outer_vec in &matrices.a {
            let mut y = 0;
            for inner_tuple in outer_vec {
                println!("({},{}) {} {}", x, y, inner_tuple.0, inner_tuple.1);
                y += 1;
            }
            x += 1;
        }


        // Generate the SRS
        let srs = Marlin::<$field, MarlinKZG10<$pairing_engine, DensePolynomial<$field>>, Blake2s>
            ::universal_setup(1000, 10, 3 * 1000, rng)
            .unwrap();

        // Generate the setup 
        let (pk, vk) = Marlin::<$field, MarlinKZG10<$pairing_engine, DensePolynomial<$field>>, Blake2s>
            ::index(&srs, c.clone())
            .unwrap();

        // Generate the proof
        let start = std::time::Instant::now();

        let proof = Marlin::<$field, MarlinKZG10<$pairing_engine, DensePolynomial<$field>>, Blake2s>
            ::prove(&pk, c.clone(), rng)
            .unwrap();

        print_info!(
            "Proving time: {}ms", 
            start.elapsed().as_millis()
        );

        // Check the proof
        let start = std::time::Instant::now();

        let res = Marlin::<$field, MarlinKZG10<$pairing_engine, DensePolynomial<$field>>, Blake2s>
            ::verify(&vk, &[c.get_result()], &proof, rng)
            .unwrap();
        print_info!("Verification: {}", res);

        print_info!(
            "Verifying time: {}ms", 
            start.elapsed().as_millis()
        );
    };
}

fn main() {
    // Accessing command-line arguments
    let args = Args::parse();

    // Get the circuit
    let circuit_name = args.system.as_str();

    // Get the number of rounds
    let rounds = args.rounds;
    if rounds == 0 {
        print_panic("0 is not a valid number of rounds")
    }

    // Get the curve
    let curve_name = args.curve.as_str();

    match (circuit_name, curve_name) {
        ("dummy", "bls12_377") => {bench!(DummyCircuit, Bls377Fr, Bls12_377, rounds);},
        ("dummy", "bls12_381") => {bench!(DummyCircuit, Bls381Fr, Bls12_381, rounds);},
        ("dummy", "mnt4_298") => {bench!(DummyCircuit, MNT4Fr, MNT4_298, rounds);},
        ("dummy", "mnt4_753") => {bench!(DummyCircuit, MNT4BigFr, MNT4_753, rounds);},
        ("dummy", "mnt6_298") => {bench!(DummyCircuit, MNT6Fr, MNT6_298, rounds);},
        ("dummy", "mnt6_753") => {bench!(DummyCircuit, MNT6BigFr, MNT6_753, rounds);},

        ("product", "bls12_381") => {bench!(ProductCircuit, Bls381Fr, Bls12_381, rounds);},

        _ => print_panic!("Invalid circuit {} or curve {}", circuit_name, curve_name)
    }


    /*let c = ProductCircuit {
        x: Bls381Fr::from(1),
        t: 3
    };

    // Generate the constraint system without optimizations
    let cs = ConstraintSystem::<Bls381Fr>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::None);

    // Show the number of constraints
    //cs.finalize();
    let _ = c.generate_constraints(cs.clone());
    println!("{}", cs.num_constraints());

    // Get the matrices
    let matrices = cs.to_matrices().unwrap();
    let mut x = 0;
    let mut y = 0;
    for outer_vec in &matrices.a {
        for inner_tuple in outer_vec {
            println!("({},{}) {} {}", x, y, inner_tuple.0, inner_tuple.1);
            x += 1;
        }
        y += 1;
    }

    let mut x = 0;
    let mut y = 0;
    for outer_vec in &matrices.b {
        for inner_tuple in outer_vec {
            println!("({},{}) {} {}", x, y, inner_tuple.0, inner_tuple.1);
            x += 1;
        }
        y += 1;
    }

    let mut x = 0;
    let mut y = 0;
    for outer_vec in &matrices.c {
        for inner_tuple in outer_vec {
            println!("({},{}) {} {}", x, y, inner_tuple.0, inner_tuple.1);
            x += 1;
        }
        y += 1;
    }*/

}
use ark_ff::PrimeField;
use arkworks_r1cs_gadgets::poseidon::FieldHasherGadget;

use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};

use arkworks_utils::{
    bytes_matrix_to_f, bytes_vec_to_f, poseidon_params::setup_poseidon_params, Curve,
};
use arkworks_native_gadgets::poseidon::{sbox::PoseidonSbox, PoseidonParameters};

// This cirtcuit executes H(a)=c
#[derive(Copy, Clone)]
pub struct PoseidonCircuit<F: PrimeField, HG: FieldHasherGadget<F>> {
	pub a: F,
	pub c: F,
	hasher: HG::Native,
}

/// Constructor for PoseidonCircuit
#[allow(dead_code)]
impl<F: PrimeField, HG: FieldHasherGadget<F>> PoseidonCircuit<F, HG> {
	pub fn new(a: F, c: F, hasher: HG::Native) -> Self {
		Self { a, c, hasher }
	}

	pub fn setup_params(curve: Curve, exp: i8, width: u8) -> PoseidonParameters<F> {
		let pos_data = setup_poseidon_params(curve, exp, width).unwrap();

		let mds_f = bytes_matrix_to_f(&pos_data.mds);
		let rounds_f = bytes_vec_to_f(&pos_data.rounds);

		let pos = PoseidonParameters {
			mds_matrix: mds_f,
			round_keys: rounds_f,
			full_rounds: pos_data.full_rounds,
			partial_rounds: pos_data.partial_rounds,
			sbox: PoseidonSbox(pos_data.exp),
			width: pos_data.width,
		};

		pos
	}
}

/*impl<F: PrimeField, HG: FieldHasherGadget<F>> NewRandomCircuit for PoseidonCircuit<F, HG> {
	fn new_random<R: RngCore>(rng: &mut R, rounds: usize) -> Self {
		let parameters = PoseidonC::setup_params(Curve::Bls381, 5, 3);
		let hasher = Poseidon::<F>::new(parameters);

		PoseidonCircuit {
			a: Some(<F>::rand(rng)), 
			a: Some(<F>::rand(rng)), 
		}
	}
}*/

impl<F: PrimeField, HG: FieldHasherGadget<F>> ConstraintSynthesizer<F> for PoseidonCircuit<F, HG> {
	fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = FpVar::new_witness(cs.clone(), || Ok(self.a))?;
		let res_target = FpVar::<F>::new_input(cs.clone(), || Ok(&self.c))?;
		let hasher_gadget: HG = FieldHasherGadget::<F>::from_native(&mut cs.clone(), self.hasher)?;

		let mut res_var = hasher_gadget.hash(&[a])?;
		res_var = hasher_gadget.hash(&[res_var])?;
		res_var = hasher_gadget.hash(&[res_var])?;

		res_var.enforce_equal(&res_target)?;

		Ok(())
	}
}
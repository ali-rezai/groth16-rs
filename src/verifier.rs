use bls12_381::{pairing, G1Affine, G1Projective, G2Affine};

use crate::{setup::TrustedSetup, utils::witness_to_scalar};

pub struct Verifier {
    setup: TrustedSetup,
}

impl Verifier {
    pub fn new(setup: TrustedSetup) -> Self {
        Verifier { setup }
    }

    pub fn verify(&self, lg1: G1Affine, rg2: G2Affine, og1: G1Affine, public_inputs: &[i64]) {
        if public_inputs.len() != self.setup.psi_verifier.len() {
            panic!("Bad public input length");
        }

        let left = pairing(&lg1, &rg2);
        let setup = &self.setup;

        let mut verifier_og1 = G1Projective::identity();
        let public_inputs = witness_to_scalar(public_inputs);
        public_inputs
            .iter()
            .enumerate()
            .for_each(|(i, input)| verifier_og1 += input * setup.psi_verifier[i]);
        let prover_og1 = pairing(&og1, &setup.delta_2);
        let verifier_og1 = pairing(&G1Affine::from(verifier_og1), &setup.gamma);
        let og1 = prover_og1 + verifier_og1;

        let right = pairing(&setup.alpha, &setup.beta_2) + og1;
        if left != right {
            panic!("Bad proof");
        }
    }
}

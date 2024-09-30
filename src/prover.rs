use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective, Scalar};
use rand::RngCore;

use crate::{poly::Poly, program::Program, setup::TrustedSetup, utils::witness_to_scalar};

pub struct Prover {
    setup: TrustedSetup,
    program: Program,
}

impl Prover {
    pub fn new(program: Program, setup: TrustedSetup) -> Self {
        Prover { program, setup }
    }

    pub fn prove(&self, witness: &[i64]) -> (G1Affine, G2Affine, G1Affine) {
        if witness.len() != self.program.cols {
            panic!("Witness size mismatch");
        }
        let witness = witness_to_scalar(witness);

        let program = &self.program;
        let setup = &self.setup;

        let mut left = vec![Scalar::zero(); program.rows];
        let mut right = vec![Scalar::zero(); program.rows];
        let mut output = vec![Scalar::zero(); program.rows];
        let mut og1 = G1Projective::identity();

        witness.iter().enumerate().for_each(|(i, val)| {
            program.left[i].0.iter().enumerate().for_each(|(j, p)| {
                left[j] += p * val;
            });
            program.right[i].0.iter().enumerate().for_each(|(j, p)| {
                right[j] += p * val;
            });
            program.output[i].0.iter().enumerate().for_each(|(j, p)| {
                output[j] += p * val;
            });

            if i >= program.public {
                og1 += setup.psi_prover[i - program.public] * val;
            }
        });

        let left_p = Poly::new(left.clone());
        let right_p = Poly::new(right.clone());
        let output_p = Poly::new(output.clone());
        let ht = Poly::sub(Poly::mul(left_p, right_p), output_p);
        let h = Poly::div(ht, program.t.clone());
        if h.is_err() {
            panic!("Bad witness");
        }
        let h = h.unwrap();

        let mut rng = rand::thread_rng();
        let r = Scalar::from(rng.next_u64());
        let s = Scalar::from(rng.next_u64());

        let mut lg1 = G1Projective::from(setup.alpha) + r * setup.delta_1;
        let mut rg1 = G1Projective::from(setup.beta_1) + s * setup.delta_1;
        let mut rg2 = G2Projective::from(setup.beta_2) + s * setup.delta_2;

        for i in 0..program.rows {
            lg1 += setup.tau_g1[i] * left[i];
            rg1 += setup.tau_g1[i] * right[i];
            rg2 += setup.tau_g2[i] * right[i];
            if h.0.len() > i {
                og1 += h.0[i] * setup.t_tau_g1[i];
            }
        }
        og1 += lg1 * s + rg1 * r - r * s * setup.delta_1;

        (
            G1Affine::from(lg1),
            G2Affine::from(rg2),
            G1Affine::from(og1),
        )
    }
}

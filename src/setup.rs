use crate::program::Program;
use bls12_381::{G1Affine, G2Affine, Scalar};
use rand::RngCore;

#[derive(Clone)]
pub struct TrustedSetup {
    pub alpha: G1Affine,
    pub beta_1: G1Affine,
    pub beta_2: G2Affine,
    pub gamma: G2Affine,
    pub delta_1: G1Affine,
    pub delta_2: G2Affine,
    pub tau_g1: Vec<G1Affine>,
    pub tau_g2: Vec<G2Affine>,
    pub t_tau_g1: Vec<G1Affine>,
    pub psi_verifier: Vec<G1Affine>,
    pub psi_prover: Vec<G1Affine>,
}

impl TrustedSetup {
    pub fn new(c: &Program) -> Self {
        let g1 = G1Affine::generator();
        let g2 = G2Affine::generator();

        let mut rng = rand::thread_rng();
        let tau = Scalar::from(rng.next_u64());
        let alpha = Scalar::from(rng.next_u64());
        let beta = Scalar::from(rng.next_u64());
        let gamma = Scalar::from(rng.next_u64());
        let delta = Scalar::from(rng.next_u64());

        let gamma_inv = gamma.invert().unwrap();
        let delta_inv = delta.invert().unwrap();

        let tau_g1 = powers_g1(tau, c.rows, Scalar::one());
        let tau_g2 = powers_g2(tau, c.rows, Scalar::one());
        let t_tau_g1 = powers_g1(tau, c.rows, c.t.eval(tau) * delta_inv);

        let mut psi_verifier = vec![];
        let mut psi_prover = vec![];
        for i in 0..c.cols {
            let is_public = i < c.public;
            if is_public {
                psi_verifier.push(G1Affine::from(
                    (alpha * c.right[i].eval(tau)
                        + beta * c.left[i].eval(tau)
                        + c.output[i].eval(tau))
                        * gamma_inv
                        * g1,
                ));
            } else {
                psi_prover.push(G1Affine::from(
                    (alpha * c.right[i].eval(tau)
                        + beta * c.left[i].eval(tau)
                        + c.output[i].eval(tau))
                        * delta_inv
                        * g1,
                ));
            };
        }

        TrustedSetup {
            alpha: G1Affine::from(alpha * g1),
            beta_1: G1Affine::from(beta * g1),
            beta_2: G2Affine::from(beta * g2),
            gamma: G2Affine::from(gamma * g2),
            delta_1: G1Affine::from(delta * g1),
            delta_2: G2Affine::from(delta * g2),
            tau_g1,
            tau_g2,
            t_tau_g1,
            psi_verifier,
            psi_prover,
        }
    }
}

fn powers_g1(val: Scalar, len: usize, mul: Scalar) -> Vec<G1Affine> {
    let mut acc = Scalar::one();
    let g1 = G1Affine::generator();
    (0..len)
        .map(|_| {
            let res = G1Affine::from(acc * mul * g1);
            acc = acc * val;
            res
        })
        .collect()
}

fn powers_g2(val: Scalar, len: usize, mul: Scalar) -> Vec<G2Affine> {
    let mut acc = Scalar::one();
    let g2 = G2Affine::generator();
    (0..len)
        .map(|_| {
            let res = G2Affine::from(acc * mul * g2);
            acc = acc * val;
            res
        })
        .collect()
}

mod poly;
pub mod program;
pub mod prover;
pub mod setup;
mod utils;
pub mod verifier;

#[cfg(test)]
mod tests {
    use crate::program::Program;
    use crate::prover::Prover;
    use crate::setup::TrustedSetup;
    use crate::verifier::Verifier;
    use bls12_381::{G1Affine, G1Projective};

    fn get_prover_and_verifier() -> (Prover, Verifier) {
        // y^2 = 4x^3 + 2z + 9
        //
        // Constraints:
        // v1 = y*y
        // v2 = x*x
        // v1 - 2z - 9 = v2*4x
        //
        // Witness:
        // [1, y, x, z, v1, v2]
        //
        // Example witnesses:
        // [1, 5, 1, 6, 25, 1]
        // [1, 7, 1, 18, 49, 1]
        //
        let public = 2;
        let c = Program::new(
            &[
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1],
            ],
            &[
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 4, 0, 0, 0],
            ],
            &[
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1],
                vec![-9, 0, 0, -2, 1, 0],
            ],
            public,
        );
        let s = TrustedSetup::new(&c);
        let verifier = Verifier::new(s.clone());
        let prover = Prover::new(c, s);
        (prover, verifier)
    }

    #[test]
    #[should_panic(expected = "LRO mismatch")]
    fn fail_lro_mismatch_empty_rows() {
        Program::new(&[], &[], &[], 0);
    }

    #[test]
    #[should_panic(expected = "LRO mismatch")]
    fn fail_lro_mismatch_empty_columns() {
        Program::new(&[vec![]], &[vec![]], &[vec![]], 0);
    }

    #[test]
    #[should_panic(expected = "LRO mismatch")]
    fn fail_lro_mismatch_rows() {
        Program::new(&[vec![1], vec![2]], &[vec![1]], &[vec![1]], 0);
    }

    #[test]
    #[should_panic(expected = "LRO mismatch")]
    fn fail_lro_mismatch_columns() {
        Program::new(&[vec![1, 2]], &[vec![1]], &[vec![1]], 0);
    }

    #[test]
    #[should_panic(expected = "Witness size mismatch")]
    fn fail_witness_lro_mismatch() {
        let witness = [1, 5, 1, 6, 25, 1, 2];
        let (prover, _) = get_prover_and_verifier();
        prover.prove(&witness);
    }

    #[test]
    #[should_panic(expected = "Bad witness")]
    fn fail_bad_witness() {
        let witness = [1, 6, 2, 6, 36, 4];
        let (prover, _) = get_prover_and_verifier();
        prover.prove(&witness);
    }

    #[test]
    #[should_panic(expected = "Bad public input length")]
    fn fail_public_input_small() {
        let (prover, verifier) = get_prover_and_verifier();
        let witness = [1, 5, 1, 6, 25, 1];
        let (mut lg1, rg2, og1) = prover.prove(&witness);
        lg1 = G1Affine::from(lg1 + G1Projective::generator());
        verifier.verify(lg1, rg2, og1, &witness[0..1]);
    }

    #[test]
    #[should_panic(expected = "Bad public input length")]
    fn fail_public_input_big() {
        let (prover, verifier) = get_prover_and_verifier();
        let witness = [1, 5, 1, 6, 25, 1];
        let (mut lg1, rg2, og1) = prover.prove(&witness);
        lg1 = G1Affine::from(lg1 + G1Projective::generator());
        verifier.verify(lg1, rg2, og1, &witness[0..3]);
    }

    #[test]
    #[should_panic(expected = "Bad proof")]
    fn fail_wrong_public_input() {
        let (prover, verifier) = get_prover_and_verifier();
        let witness = [1, 5, 1, 6, 25, 1];
        let (lg1, rg2, og1) = prover.prove(&witness);
        verifier.verify(lg1, rg2, og1, &witness[1..3]);
    }

    #[test]
    #[should_panic(expected = "Bad proof")]
    fn fail_bad_proof() {
        let (prover, verifier) = get_prover_and_verifier();
        let witness = [1, 5, 1, 6, 25, 1];
        let (mut lg1, rg2, og1) = prover.prove(&witness);
        lg1 = G1Affine::from(lg1 + G1Projective::generator());
        verifier.verify(lg1, rg2, og1, &witness[0..2]);
    }

    #[test]
    fn pass_1() {
        let (prover, verifier) = get_prover_and_verifier();

        let witness = [1, 5, 1, 6, 25, 1];
        let (lg1, rg2, og1) = prover.prove(&witness);
        verifier.verify(lg1, rg2, og1, &witness[0..2]);

        let witness = [1, 7, 1, 18, 49, 1];
        let (lg1, rg2, og1) = prover.prove(&witness);
        verifier.verify(lg1, rg2, og1, &witness[0..2]);
    }

    #[test]
    fn pass_2() {
        // 529 = x^3 + 4x^2 - yz + 4
        //
        // Constraints:
        // v1 = x*x
        // v2 = x*v1
        // 529 - v2 - 4v1 - 4 = -y*z
        //
        // Witness:
        // [1, y, x, z, v1, v2]
        //
        // Example witness:
        // [1, 7, 7, 2, 49, 343]
        //
        let public = 0;
        let c = Program::new(
            &[
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, -1, 0, 0, 0, 0],
            ],
            &[
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 0, 0],
            ],
            &[
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1],
                vec![525, 0, 0, 0, -4, -1],
            ],
            public,
        );
        let s = TrustedSetup::new(&c);
        let verifier = Verifier::new(s.clone());
        let prover = Prover::new(c, s);

        let witness = [1, 7, 7, 2, 49, 343];
        let (lg1, rg2, og1) = prover.prove(&witness);
        verifier.verify(lg1, rg2, og1, &[]);
    }

    #[test]
    #[should_panic(expected = "Bad proof")]
    fn fail_proof_from_other() {
        let (prover, _) = get_prover_and_verifier();
        let witness = [1, 5, 1, 6, 25, 1];
        let (lg1, rg2, og1) = prover.prove(&witness);
        let public = 2;
        let c = Program::new(
            &[
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, -1, 0, 0, 0, 0],
            ],
            &[
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 0, 0],
            ],
            &[
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1],
                vec![525, 0, 0, 0, -4, -1],
            ],
            public,
        );
        let s = TrustedSetup::new(&c);
        let verifier = Verifier::new(s);
        verifier.verify(lg1, rg2, og1, &witness[0..2]);
    }
}

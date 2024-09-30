use groth16_rs::program::Program;
use groth16_rs::prover::Prover;
use groth16_rs::setup::TrustedSetup;
use groth16_rs::verifier::Verifier;

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
fn main() {
    let public = 2;
    let program = Program::new(
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
    let setup = TrustedSetup::new(&program);

    let verifier = Verifier::new(setup.clone());
    let prover = Prover::new(program, setup);

    let witness = [1, 5, 1, 6, 25, 1];
    let (lg1, rg2, og1) = prover.prove(&witness);
    verifier.verify(lg1, rg2, og1, &witness[0..public]);

    let witness = [1, 7, 1, 18, 49, 1];
    let (lg1, rg2, og1) = prover.prove(&witness);
    verifier.verify(lg1, rg2, og1, &witness[0..public]);
}

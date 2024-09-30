use crate::{
    poly::Poly,
    utils::{to_poly, to_scalar},
};
use bls12_381::Scalar;

pub struct Program {
    pub rows: usize,
    pub cols: usize,
    pub left: Vec<Poly>,
    pub right: Vec<Poly>,
    pub output: Vec<Poly>,
    pub t: Poly,
    pub public: usize,
}

impl Program {
    pub fn new(left: &[Vec<i64>], right: &[Vec<i64>], output: &[Vec<i64>], public: usize) -> Self {
        if left.len() == 0
            || left[0].len() == 0
            || left.len() != right.len()
            || right.len() != output.len()
            || left
                .iter()
                .zip(right.iter())
                .enumerate()
                .any(|(index, (l, r))| l.len() != r.len() || r.len() != output[index].len())
        {
            panic!("LRO mismatch");
        }

        let rows = left.len();
        let cols = left[0].len();

        let (left, right, output) =
            to_poly(&to_scalar(left), &to_scalar(right), &to_scalar(output));

        // Create t(x) = (x-1)(x-2)(x-3)...(x-n)
        let mut t = Poly::new(vec![Scalar::one()]);
        for i in 0..rows {
            t = Poly::mul(
                t,
                Poly::new(vec![-Scalar::from(i as u64 + 1), Scalar::one()]),
            );
        }

        Program {
            rows,
            cols,
            left,
            right,
            output,
            t,
            public,
        }
    }
}

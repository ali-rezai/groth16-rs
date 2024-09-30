use crate::poly::Poly;
use bls12_381::Scalar;

pub fn to_scalar(input: &[Vec<i64>]) -> Vec<Vec<Scalar>> {
    input
        .iter()
        .map(|row| {
            row.iter()
                .map(|col| {
                    let abs = col.abs();
                    if abs > *col {
                        Scalar::from(abs as u64).neg()
                    } else {
                        Scalar::from(abs as u64)
                    }
                })
                .collect()
        })
        .collect()
}

pub fn witness_to_scalar(witness: &[i64]) -> Vec<Scalar> {
    witness
        .iter()
        .map(|col| {
            let abs = col.abs();
            if abs > *col {
                Scalar::from(abs as u64).neg()
            } else {
                Scalar::from(abs as u64)
            }
        })
        .collect()
}

pub fn to_poly(
    left: &[Vec<Scalar>],
    right: &[Vec<Scalar>],
    output: &[Vec<Scalar>],
) -> (Vec<Poly>, Vec<Poly>, Vec<Poly>) {
    let domain: Vec<Scalar> = (0..left.len())
        .map(|value| Scalar::from(value as u64 + 1))
        .collect();

    let mut poly_left = vec![];
    let mut poly_right = vec![];
    let mut poly_output = vec![];

    for column in 0..left[0].len() {
        poly_left.push(Poly::interpolate(
            domain.clone(),
            left.iter().map(|row| row[column]).collect(),
        ));
        poly_right.push(Poly::interpolate(
            domain.clone(),
            right.iter().map(|row| row[column]).collect(),
        ));
        poly_output.push(Poly::interpolate(
            domain.clone(),
            output.iter().map(|row| row[column]).collect(),
        ));
    }

    (poly_left, poly_right, poly_output)
}

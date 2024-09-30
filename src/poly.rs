use bls12_381::Scalar;

#[derive(Clone, Debug, PartialEq)]
pub struct Poly(pub Vec<Scalar>);

#[cfg(test)]
fn to_scalar(values: Vec<i64>) -> Vec<Scalar> {
    values
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

impl Poly {
    pub fn new(coeffs: Vec<Scalar>) -> Self {
        let mut coeffs: Vec<Scalar> = coeffs
            .into_iter()
            .rev()
            .skip_while(|&x| x == Scalar::zero())
            .collect();
        coeffs.reverse();
        Self(coeffs)
    }

    #[cfg(test)]
    fn new_from_i64(coeffs: Vec<i64>) -> Self {
        let mut coeffs: Vec<i64> = coeffs.into_iter().rev().skip_while(|&x| x == 0).collect();
        coeffs.reverse();
        Self(to_scalar(coeffs))
    }

    pub fn degree(&self) -> i32 {
        let len = self.0.len();
        if len == 0 {
            return -1;
        }
        let mut max_index = 0;
        let mut zeros = 0;
        self.0.iter().enumerate().for_each(|(index, e)| {
            if *e != Scalar::zero() {
                max_index = index;
            } else {
                zeros += 1;
            }
        });
        if zeros == len {
            return -1;
        }
        return max_index.try_into().unwrap();
    }

    pub fn eval(&self, x: Scalar) -> Scalar {
        let mut curr = Scalar::one();
        let mut res = Scalar::zero();
        for i in 0..self.0.len() {
            res += self.0[i] * curr;
            curr = curr * x;
        }
        res
    }

    pub fn leading_coefficient(&self) -> Scalar {
        let index: usize = self.degree().try_into().unwrap();
        self.0[index]
    }

    pub fn interpolate(domain: Vec<Scalar>, evaluations: Vec<Scalar>) -> Self {
        assert!(domain.len() == evaluations.len());
        assert!(domain.len() > 0);

        let x = Poly::new(vec![Scalar::zero(), Scalar::one()]);
        let mut acc = Poly::new(vec![]);
        for i in 0..domain.len() {
            let mut prod = Poly::new(vec![evaluations[i]]);
            for j in 0..domain.len() {
                if j == i {
                    continue;
                }
                prod = Poly::mul(
                    Poly::mul(
                        prod,
                        Poly::add(x.clone(), Poly::neg(Poly::new(vec![domain[j]]))),
                    ),
                    Poly::new(vec![(domain[i] - domain[j]).invert().unwrap()]),
                );
            }
            acc = Poly::add(acc, prod);
        }
        acc
    }

    pub fn add(lhs: Poly, rhs: Poly) -> Poly {
        if lhs.degree() == -1 {
            return rhs.clone();
        } else if rhs.degree() == -1 {
            return lhs.clone();
        }

        let size = if lhs.0.len() > rhs.0.len() {
            lhs.0.len()
        } else {
            rhs.0.len()
        };

        let mut new_coeffs = vec![Scalar::zero(); size];
        lhs.0.iter().enumerate().for_each(|(index, e)| {
            new_coeffs[index] = new_coeffs[index] + e;
        });
        rhs.0.iter().enumerate().for_each(|(index, e)| {
            new_coeffs[index] = new_coeffs[index] + e;
        });
        Poly::new(new_coeffs)
    }

    pub fn sub(lhs: Poly, rhs: Poly) -> Poly {
        Poly::add(lhs, Poly::neg(rhs))
    }

    pub fn mul(lhs: Poly, rhs: Poly) -> Poly {
        if lhs.0.len() == 0 || rhs.0.len() == 0 {
            return Poly::new(vec![]);
        }
        let zero = Scalar::zero();
        let size = rhs.0.len() + lhs.0.len() - 1;
        let mut new_coeffs = vec![zero; size];
        lhs.0.iter().enumerate().for_each(|(i, e)| {
            if *e != zero {
                rhs.0.iter().enumerate().for_each(|(j, er)| {
                    new_coeffs[i + j] = new_coeffs[i + j] + (e * er);
                });
            }
        });
        Poly::new(new_coeffs)
    }

    pub fn div(lhs: Poly, rhs: Poly) -> Result<Poly, ()> {
        if rhs.degree() == -1 && lhs.degree() < rhs.degree() {
            return Err(());
        }

        let degree = lhs.degree() - rhs.degree() + 1;

        let mut remainder = lhs.clone();
        let mut quotient_coefficients = vec![Scalar::zero(); degree.try_into().unwrap()];

        for _ in 0..degree {
            if remainder.degree() < rhs.degree() {
                break;
            }
            let coefficient =
                remainder.leading_coefficient() * rhs.leading_coefficient().invert().unwrap();
            let shift: usize = (remainder.degree() - rhs.degree()).try_into().unwrap();

            let mut coeffs = vec![Scalar::zero(); shift];
            coeffs.push(coefficient.clone());

            let subtractee = Poly::mul(Poly::new(coeffs), rhs.clone());

            quotient_coefficients[shift] = coefficient;
            remainder = Poly::sub(remainder, subtractee);
        }
        let quotient = Poly::new(quotient_coefficients);

        if remainder.degree() != -1 {
            return Err(());
        }

        Ok(quotient)
    }

    pub fn neg(lhs: Poly) -> Poly {
        let new_coeffs = lhs.0.iter().map(|e| -*e).collect();
        Poly::new(new_coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poly_add() {
        let a = Poly::new_from_i64(vec![1, 2, 3]);
        let b = Poly::new_from_i64(vec![2, 3]);
        assert_eq!(Poly::add(a, b), Poly::new_from_i64(vec![3, 5, 3]));
    }

    #[test]
    fn poly_sub() {
        let a = Poly::new_from_i64(vec![1, 2, 3]);
        let b = Poly::new_from_i64(vec![2, 3]);
        assert_eq!(Poly::sub(a, b), Poly::new_from_i64(vec![-1, -1, 3]));
    }

    #[test]
    fn poly_mul() {
        let a = Poly::new_from_i64(vec![1, 2, 3]);
        let b = Poly::new_from_i64(vec![2, 3]);
        assert_eq!(Poly::mul(a, b), Poly::new_from_i64(vec![2, 7, 12, 9]));
    }

    #[test]
    fn poly_interpolate() {
        let domain = to_scalar(vec![1, 2, 3, 4]);
        let evaluations = to_scalar(vec![1, 2, 3, 4]);

        let poly = Poly::interpolate(domain, evaluations);
        assert_eq!(poly, Poly::new_from_i64(vec![0, 1]));

        let domain = to_scalar(vec![1, 2, 3, 4]);
        let evaluations = to_scalar(vec![3, 7, 13, 21]);

        let poly = Poly::interpolate(domain, evaluations);
        assert_eq!(poly, Poly::new_from_i64(vec![1, 1, 1]));
    }
}

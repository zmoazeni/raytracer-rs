use super::*;
use crate::iterator::*;
use crate::util;

use std::ops::{Index,IndexMut,Mul};
use std::cmp::{PartialEq,Eq};

impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        let row = vec![0.0; width];
        let mut v2 = Vec::with_capacity(height);
        for _ in 0..height {
            v2.push(row.clone());
        }

        Matrix { height, width, values: v2 }
    }

    pub fn with_values(values: Vec<Vec<f32>>) -> Matrix {
        let height = values.len();
        let width = values.first().expect("matrix requires at least one row").len();
        let mut m = Self::new(height, width);

        for (y, row) in values.iter().enumerate() {
            if row.len() != width {
                panic!("matrix is not uniform {}x{}. Row {} has {} column((s).", height, width, y, row.len());
            }

            for (x, value) in row.iter().enumerate() {
                m[(y, x)] = *value;
            }
        }
        m
    }

    pub fn iter(&self) -> DimensionalIterator {
        DimensionalIterator::matrix(self.dimensions())
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn identity(&self) -> Matrix {
        let mut id = Matrix::new(self.height, self.width);
        let mut x = 0;
        for y in 0..self.height {
            id[(y, x)] = 1.0;
            x += 1;
        }
        id
    }

    pub fn new_identity(y: usize, x: usize) -> Matrix {
        let m = Matrix::new(y, x);
        m.identity()
    }

    pub fn transpose(&self) -> Matrix {
        let mut m = Matrix::new(self.width, self.height);
        for (y, x) in self.iter() {
            m[(x, y)] = self[(y, x)];
        }
        m
    }

    pub fn determinate(&self) -> Result<f32, String> {
        if self.height != self.width {
            return Err(format!("Matrix must be square: {}x{}", self.height, self.width))
        }

        if self.height == 2 && self.width == 2 {
            let a = self[(0, 0)];
            let b = self[(0, 1)];
            let c = self[(1, 0)];
            let d = self[(1, 1)];
            return Ok((a * d) - (b * c))
        }

        let mut calculated = Ok(0.0);
        let mut calculated_something = false;
        for x in 0..self.width {
            calculated = calculated.and_then(|sum| {
                self.cofactor(0, x).map(|cofactor| {
                    calculated_something = true;
                    let value = self[(0, x)];
                    cofactor * value
                }).map(|v| v + sum)
            });
        }

        match calculated {
            Ok(x) => {
                if calculated_something {
                    Ok(x)
                } else {
                    Err(String::from("Matrix is not invertable and can't calculate determinate"))
                }
            }
            err@_ => err
        }
    }

    pub fn submatrix(&self, skip_y: usize, skip_x: usize) -> Result<Matrix, String> {
        if skip_y >= self.height || skip_x >= self.width {
            return Err(format!("y:{} or x:{} is outside the matrix dimensions {}x{}", skip_y, skip_x, self.height, self.width))
        }
        let mut m = Matrix::new(self.height - 1, self.width - 1);
        for (y, x) in self.iter() {
            if y != skip_y && x != skip_x {
                let y2 = if y < skip_y {
                    y
                } else {
                    y - 1
                };
                let x2 = if x < skip_x {
                    x
                } else {
                    x - 1
                };

                m[(y2, x2)] = self[(y, x)];
            }
        }
        Ok(m)
    }

    pub fn minor(&self, y: usize, x: usize) -> Result<f32, String> {
        return self.submatrix(y, x).and_then(|m| m.determinate())
    }

    pub fn cofactor(&self, y: usize, x: usize) -> Result<f32, String> {
        let sign = if (y + x) % 2 == 1 { -1.0 } else { 1.0 };
        self.minor(y, x).map(|v| v * sign)
    }

    pub fn is_invertable(&self) -> bool {
        match self.determinate() {
            Ok(x) => {
                if util::feq(x, 0.0) {
                    false
                } else {
                    true
                }
            }
            _ => false
        }
    }

    pub fn inverse(&self) -> Result<Matrix, String> {
        match self.determinate() {
            Ok(determinate) => {
                let mut inverse = Self::new(self.height, self.width);
                for (y, x) in self.iter() {
                    match self.cofactor(x, y) {
                        Ok(cofactor) => { inverse[(y, x)] = cofactor / determinate; }
                        Err(e) => return Err(e)
                    }
                }
                Ok(inverse)
            }
            Err(e) => Err(e)
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, (y, x): (usize, usize)) -> &f32 {
        &self.values[y][x]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut f32 {
        &mut self.values[y][x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        if self.dimensions() != rhs.dimensions() {
            return false;
        }
        self.iter().all(|dimensions| {
            util::feq(self[dimensions], rhs[dimensions])
        })
    }
}
impl Eq for Matrix {}

fn matrix_mul_helper(lhs: &Matrix, rhs: &Matrix) -> Result<Matrix, String> {
    if lhs.width != rhs.height {
        return Err(format!("width of left ({}x{}) does not match height of rhs ({}x{})", lhs.height, lhs.width, rhs.height, rhs.width));
    }

    let mut result = Matrix::new(lhs.height, rhs.width);
    for (y, x) in result.iter() {
        let mut sum = 0.0;
        for i in 0..lhs.width {
            sum += lhs[(y, i)] * rhs[(i, x)];
        }
        result[(y, x)] = sum;
    }
    Ok(result)
}

impl Mul<Matrix> for Matrix {
    type Output = Result<Self, String>;
    fn mul(self, rhs: Self) -> Result<Self, String> {
        matrix_mul_helper(&self, &rhs)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Result<Matrix, String>;
    fn mul(self, rhs: &Matrix) -> Result<Matrix, String> {
        matrix_mul_helper(self, rhs)
    }
}

impl Mul<Result<Matrix, String>> for Matrix {
    type Output = Result<Matrix, String>;
    fn mul(self, rhs: Result<Matrix, String>) -> Result<Matrix, String> {
        rhs.and_then(|rhs| self * rhs)
    }
}

impl Mul<Result<&Matrix, String>> for &Matrix {
    type Output = Result<Matrix, String>;
    fn mul(self, rhs: Result<&Matrix, String>) -> Result<Matrix, String> {
        rhs.and_then(|rhs| self * rhs)
    }
}

impl Mul<Matrix> for Result<Matrix, String> {
    type Output = Result<Matrix, String>;
    fn mul(self, rhs: Matrix) -> Result<Matrix, String> {
        self.and_then(|lhs| lhs * rhs)
    }
}

impl Mul<&Matrix> for Result<&Matrix, String> {
    type Output = Result<Matrix, String>;
    fn mul(self, rhs: &Matrix) -> Result<Matrix, String> {
        self.and_then(|lhs| lhs * rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_matrix() {
        let m = Matrix::new(4, 3);
        assert_eq!(m.height, 4);
        assert_eq!(m.width, 3);

        let mut count = 0;
        for (y, x) in m.iter() {
            count += 1;
            let value = m[(y, x)];
            assert_eq!(value, 0.0);
        }
        assert_eq!(count, 12);
    }

    #[test]
    fn indexing1() {
        let m = matrix![
            1.0, 2.0, 3.0, 4.0;
            5.5, 6.5, 7.5, 8.5;
            9.0, 10.0, 11.0, 12.0;
            13.5, 14.5, 15.5, 16.5;
        ];

        assert_eq!(m[(0,0)], 1.0);
        assert_eq!(m[(0,3)], 4.0);
        assert_eq!(m[(1,0)], 5.5);
        assert_eq!(m[(1,2)], 7.5);
        assert_eq!(m[(2,2)], 11.0);
        assert_eq!(m[(3,0)], 13.5);
        assert_eq!(m[(3,2)], 15.5);
    }

    #[test]
    fn indexing2() {
        let m = matrix![
            -3.0, 5.0;
            1.0, -2.0;
        ];

        assert_eq!(m[(0,0)], -3.0);
        assert_eq!(m[(0,1)], 5.0);
        assert_eq!(m[(1,0)], 1.0);
        assert_eq!(m[(1,1)], -2.0);
    }

    #[test]
    fn indexing3() {
        let m = matrix![
            -3.0, 5.0, 0.0;
            1.0, -2.0, -7.0;
            0.0, 1.0, 1.0;
        ];

        assert_eq!(m[(0,0)], -3.0);
        assert_eq!(m[(1,1)], -2.0);
        assert_eq!(m[(2,2)], 1.0);
    }

    #[test]
    fn equivalence() {
        let m = matrix![
            1.0, 2.0;
            3.0, 4.0;
        ];
        assert_eq!(m, m);

        let m2 = matrix![
            1.0, 2.0;
            3.0, 4.0;
        ];
        assert_eq!(m, m2);

        let m3 = matrix![
            1.0;
            2.0
        ];
        assert_ne!(m, m3);

        let m4 = matrix![
            1.0, 2.0;
            3.0, 10.0;
        ];
        assert_ne!(m, m4);
    }

    #[test]
    fn multiplication4x4() {
        let m1 = matrix![
            1.0, 2.0, 3.0, 4.0;
            5.0, 6.0, 7.0, 8.0;
            9.0, 8.0, 7.0, 6.0;
            5.0, 4.0, 3.0, 2.0
        ];
        let m2 = matrix![
            -2.0, 1.0, 2.0, 3.0;
            3.0, 2.0, 1.0, -1.0;
            4.0, 3.0, 6.0, 5.0;
            1.0, 2.0, 7.0, 8.0
        ];
        let expected = matrix![
            20.0, 22.0, 50.0, 48.0;
            44.0, 54.0, 114.0, 108.0;
            40.0, 58.0, 110.0, 102.0;
            16.0, 26.0, 46.0, 42.0
        ];
        assert_eq!(Ok(expected.clone()), m1.clone() * m2.clone());
        assert_eq!(Ok(expected.clone()), Ok(m1.clone()) * m2.clone());
        assert_eq!(Ok(expected.clone()), m1.clone() * Ok(m2.clone()));

        let r = &m1 * Ok(&m2);
        assert_eq!(Ok(expected.clone()), r);

        let r = Ok(&m1) * &m2;
        assert_eq!(Ok(expected.clone()), r);
    }

    #[test]
    fn multiplication_wrong_sizes() {
        let m1 = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];
        let m2 = matrix![
            1.0, 2.0, 3.0;
        ];
        assert!((m1 * m2).is_err());
    }

    #[test]
    fn multiplication_tuple() {
        let m1 = matrix![
            1.0, 2.0, 3.0, 4.0;
            2.0, 4.0, 4.0, 2.0;
            8.0, 6.0, 4.0, 1.0;
            0.0, 0.0, 0.0, 1.0
        ];
        let tuple = tuple![
            1.0, 2.0, 3.0, 1.0
        ];
        assert_eq!(4, tuple.height);
        assert_eq!(1, tuple.width);

        let expected = matrix![
            18.0;
            24.0;
            33.0;
            1.0
        ];
        assert_eq!(Ok(expected), m1 * tuple);
    }

    #[test]
    fn identity() {
        let m = matrix![
            1.0, 2.0, 3.0, 4.0;
            2.0, 4.0, 4.0, 2.0;
            8.0, 6.0, 4.0, 1.0;
            0.0, 0.0, 0.0, 1.0
        ];
        let expected = matrix![
            1.0, 0.0, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0;
            0.0, 0.0, 0.0, 1.0
        ];
        assert_eq!(expected, m.identity());
    }

    #[test]
    fn identity_inverse() {
        let m = Matrix::new_identity(4, 4);
        assert_eq!(Ok(1.0), m.determinate());
        let inverse = m.inverse();
        assert_eq!(Ok(m), inverse);
    }

    #[test]
    fn transpose() {
        let m = matrix![
            0.0, 9.0, 3.0, 0.0;
            9.0, 8.0, 0.0, 8.0;
            1.0, 8.0, 5.0, 3.0;
            0.0, 0.0, 5.0, 8.0
        ];
        let expected = matrix![
            0.0, 9.0, 1.0, 0.0;
            9.0, 8.0, 8.0, 0.0;
            3.0, 0.0, 5.0, 5.0;
            0.0, 8.0, 3.0, 8.0
        ];
        assert_eq!(expected, m.transpose());
    }

    #[test]
    fn determinate2x2() {
        let m = matrix![
            1.0, 5.0;
            -3.0, 2.0
        ];
        assert_eq!(Ok(17.0), m.determinate());
    }

    #[test]
    fn determinate3x3() {
        let a = matrix![
            1.0, 2.0, 6.0;
            -5.0, 8.0, -4.0;
            2.0, 6.0, 4.0
        ];
        assert_eq!(Ok(56.0), a.cofactor(0, 0));
        assert_eq!(Ok(12.0), a.cofactor(0, 1));
        assert_eq!(Ok(-46.0), a.cofactor(0, 2));
        assert_eq!(Ok(-196.0), a.determinate());
    }

    #[test]
    fn determinate4x4() {
        let a = matrix![
            -2.0, -8.0, 3.0, 5.0;
            -3.0, 1.0, 7.0, 3.0;
            1.0, 2.0, -9.0, 6.0;
            -6.0, 7.0, 7.0, -9.0
        ];
        assert_eq!(Ok(690.0), a.cofactor(0, 0));
        assert_eq!(Ok(447.0), a.cofactor(0, 1));
        assert_eq!(Ok(210.0), a.cofactor(0, 2));
        assert_eq!(Ok(51.0), a.cofactor(0, 3));
        assert_eq!(Ok(-4071.0), a.determinate());
    }

    #[test]
    fn determinate_not_square() {
        let a = matrix![
            -2.0, -8.0, 5.0;
            -3.0, 1.0, 3.0;
        ];
        assert!(a.determinate().is_err());
    }

    #[test]
    fn determinate_unknown() {
        assert!(matrix![
            1.0, 2.0, 3.0
        ].determinate().is_err());
    }

    #[test]
    fn submatrix_pass() {
        let m = matrix![
            1.0, 5.0, 0.0;
            -3.0, 2.0, 7.0;
            0.0, 6.0, -3.0
        ];
        assert_eq!(matrix![
            -3.0, 2.0;
            0.0, 6.0
        ], m.submatrix(0, 2).unwrap());

        let m = matrix![
            -6.0, 1.0, 1.0, 6.0;
            -8.0, 5.0, 8.0, 6.0;
            -1.0, 0.0, 8.0, 2.0;
            -7.0, 1.0, -1.0, 1.0
        ];
        assert_eq!(matrix![
            -6.0, 1.0, 6.0;
            -8.0, 8.0, 6.0;
            -7.0, -1.0, 1.0
        ], m.submatrix(2, 1).unwrap());
    }

    #[test]
    fn submatrix_beyond_size() {
        let m = matrix![
            1.0, 5.0, 0.0;
            -3.0, 2.0, 7.0;
            0.0, 6.0, -3.0
        ];
        assert!(m.submatrix(3, 2).is_err());
        assert!(m.submatrix(0, 3).is_err());
    }

    #[test]
    fn minor_3x3() {
        let a = matrix![
            3.0, 5.0, 0.0;
            2.0, -1.0, -7.0;
            6.0, -1.0, 5.0
        ];
        let b = a.submatrix(1, 0).unwrap();
        assert_eq!(Ok(25.0), b.determinate());
        assert_eq!(Ok(25.0), a.minor(1, 0));
    }

    #[test]
    fn minor_invalid_size() {
        assert!(matrix![1.0, 2.0].minor(0, 0).is_err());
    }

    #[test]
    fn cofactor_3x3() {
        let a = matrix![
            3.0, 5.0, 0.0;
            2.0, -1.0, -7.0;
            6.0, -1.0, 5.0
        ];
        assert_eq!(Ok(-12.0), a.minor(0, 0));
        assert_eq!(Ok(-12.0), a.cofactor(0, 0));
        assert_eq!(Ok(25.0), a.minor(1, 0));
        assert_eq!(Ok(-25.0), a.cofactor(1, 0));
    }

    #[test]
    fn inversion1() {
        let a = matrix![
            6.0,  4.0,  4.0,  4.0;
            5.0,  5.0,  7.0,  6.0;
            4.0,  -9.0,  3.0,  -7.0;
            9.0,  1.0,  7.0,  -6.0
        ];
        assert_eq!(Ok(-2120.0), a.determinate());
        assert!(a.is_invertable());
    }

    #[test]
    fn inversion2() {
        let a = matrix![
            -4.0,  2.0,  -2.0,  -3.0;
            9.0,  6.0,  2.0,  6.0;
            0.0,  -5.0,  1.0,  -5.0;
            0.0,  0.0,  0.0,  0.0
        ];
        assert_eq!(Ok(0.0), a.determinate());
        assert!(!a.is_invertable());
    }

    #[test]
    fn inversion_calculation() {
        let a = matrix![
            -5.0, 2.0, 6.0, -8.0;
            1.0, -5.0, 1.0, 8.0;
            7.0, 7.0, -6.0, -7.0;
            1.0, -3.0, 7.0, 4.0
        ];
        let b = a.inverse().unwrap();
        assert_eq!(Ok(532.0), a.determinate());
        assert_eq!(Ok(-160.0), a.cofactor(2, 3));
        assert_feq!(-160.0/532.0, b[(3, 2)]);

        assert_eq!(Ok(105.0), a.cofactor(3, 2));
        assert_feq!(105.0/532.0, b[(2, 3)]);
        assert_eq!(matrix![
            0.21805, 0.45113, 0.24060, -0.04511;
            -0.80827, -1.45677, -0.44361, 0.52068;
            -0.07895, -0.22368, -0.05263, 0.19737;
            -0.52256, -0.81391, -0.30075, 0.30639
        ], b);
    }

    #[test]
    fn inverse_scenarios1() {
        let a = matrix![
            8.0, -5.0, 9.0, 2.0;
            7.0, 5.0, 6.0, 1.0;
            -6.0, 0.0, 9.0, 6.0;
            -3.0, 0.0, -9.0, -4.0
        ];

        assert_eq!(matrix![
            -0.15385, -0.15385, -0.28205, -0.53846;
            -0.07692, 0.12308, 0.02564, 0.03077;
            0.35897, 0.35897, 0.43590, 0.92308;
            -0.69231, -0.69231, -0.76923, -1.92308
        ], a.inverse().unwrap());
    }

    #[test]
    fn inverse_scenarios2() {
        let a = matrix![
            9.0, 3.0, 0.0, 9.0;
            -5.0, -2.0, -6.0, -3.0;
            -4.0, 9.0, 6.0, 4.0;
            -7.0, 6.0, 6.0, 2.0
        ];

        assert_eq!(matrix![
            -0.04074, -0.07778, 0.14444, -0.22222;
            -0.07778, 0.03333, 0.36667, -0.33333;
            -0.02901, -0.14630, -0.10926, 0.12963;
            0.17778, 0.06667, -0.26667, 0.33333
        ], a.inverse().unwrap());
    }

    #[test]
    fn inversing_multiplication() {
        let a = matrix![
            3.0, -9.0, 7.0, 3.0;
            3.0, -8.0, 2.0, -9.0;
            -4.0, 4.0, 4.0, 1.0;
            -6.0, 5.0, -1.0, 1.0
        ];

        let b = matrix![
            8.0, 2.0, 2.0, 2.0;
            3.0, -1.0, 7.0, 0.0;
            7.0, 0.0, 5.0, 4.0;
            6.0, -2.0, 0.0, 5.0
        ];

        let c = &a * &b;
        assert_eq!(Ok(a), c.and_then(|c| c * b.inverse()));
    }
}

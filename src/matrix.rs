use super::iterator::*;
use super::util::*;
use std::ops::{Index,IndexMut,Mul};
use std::cmp::{PartialEq,Eq};

#[macro_export]
macro_rules! matrix {
    (
        $(
            $($x:expr),+
        );* $(;)?
    ) => (
        Matrix::with_values(
            vec![$(
                vec![$($x),*],
            )*]
        )
    )
}

#[macro_export]
macro_rules! tuple {
    ( $($x:expr),* ) => {
        matrix![
            $($x);*
        ]
    };
}


#[derive(Debug,Clone)]
pub struct Matrix {
    height: usize,
    width: usize,
    values: Vec<Vec<f32>>,
}

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

    pub fn transpose(&self) -> Matrix {
        let mut m = Matrix::new(self.width, self.height);
        for (y, x) in self.iter() {
            m[(x, y)] = self[(y, x)];
        }
        m
    }

    pub fn determinate(&self) -> Option<f32> {
        if self.height != self.width {
            return None
        }

        if self.height == 2 && self.width == 2 {
            let a = self[(0, 0)];
            let b = self[(0, 1)];
            let c = self[(1, 0)];
            let d = self[(1, 1)];
            return Some((a * d) - (b * c))
        }

        let mut calculated = Some(0.0);
        for x in 0..self.width {
            calculated = calculated.and_then(|sum| {
                self.cofactor(0, x).map(|cofactor| {
                    let value = self[(0, x)];
                    cofactor * value
                }).map(|v| v + sum)
            });
        }
        calculated
    }

    pub fn submatrix(&self, skip_y: usize, skip_x: usize) -> Option<Matrix> {
        if skip_y >= self.height || skip_x >= self.width {
            return None
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
        Some(m)
    }

    pub fn minor(&self, y: usize, x: usize) -> Option<f32> {
        return self.submatrix(y, x).and_then(|m| m.determinate())
    }

    pub fn cofactor(&self, y: usize, x: usize) -> Option<f32> {
        let sign = if (y + x) % 2 == 1 { -1.0 } else { 1.0 };
        self.minor(y, x).map(|v| v * sign)
    }

    pub fn is_invertable(&self) -> bool {
        match self.determinate() {
            None => false,
            Some(v) => !feq(v, 0.0)
        }
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if !self.is_invertable() {
            return None
        }

        match self.determinate() {
            Some(determinate) => {
                let mut inverse = Self::new(self.height, self.width);
                for (y, x) in self.iter() {
                    if let Some(cofactor) = self.cofactor(x, y) {
                        inverse[(y, x)] = cofactor / determinate;
                    } else {
                        return None
                    }
                }
                Some(inverse)
            }
            _ => None
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
            feq(self[dimensions], rhs[dimensions])
        })
    }
}
impl Eq for Matrix {}

impl Mul<Matrix> for Matrix {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Option<Self> {
        if self.width != rhs.height {
            return None;
        }

        let mut result = Matrix::new(self.height, rhs.width);
        for (y, x) in result.iter() {
            let mut sum = 0.0;
            for i in 0..self.width {
                sum += self[(y, i)] * rhs[(i, x)];
            }
            result[(y, x)] = sum;
        }
        Some(result)
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
        assert_eq!(Some(expected), m1 * m2);
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
        println!("{:?} {:?}", m1, m2);
        assert_eq!(None, m1 * m2);
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
        assert_eq!(Some(expected), m1 * tuple);
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
        assert_eq!(17.0, m.determinate().unwrap());
    }

    #[test]
    fn determinate3x3() {
        let a = matrix![
            1.0, 2.0, 6.0;
            -5.0, 8.0, -4.0;
            2.0, 6.0, 4.0
        ];
        assert_eq!(Some(56.0), a.cofactor(0, 0));
        assert_eq!(Some(12.0), a.cofactor(0, 1));
        assert_eq!(Some(-46.0), a.cofactor(0, 2));
        assert_eq!(Some(-196.0), a.determinate());
    }

    #[test]
    fn determinate4x4() {
        let a = matrix![
            -2.0, -8.0, 3.0, 5.0;
            -3.0, 1.0, 7.0, 3.0;
            1.0, 2.0, -9.0, 6.0;
            -6.0, 7.0, 7.0, -9.0
        ];
        assert_eq!(Some(690.0), a.cofactor(0, 0));
        assert_eq!(Some(447.0), a.cofactor(0, 1));
        assert_eq!(Some(210.0), a.cofactor(0, 2));
        assert_eq!(Some(51.0), a.cofactor(0, 3));
        assert_eq!(Some(-4071.0), a.determinate());
    }

    #[test]
    fn determinate_not_square() {
        let a = matrix![
            -2.0, -8.0, 5.0;
            -3.0, 1.0, 3.0;
        ];
        assert_eq!(None, a.determinate());
    }

    #[test]
    fn determinate_unknown() {
        assert_eq!(None, matrix![
            1.0, 2.0, 3.0
        ].determinate());
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
        assert_eq!(None, m.submatrix(3, 2));
        assert_eq!(None, m.submatrix(0, 3));
    }

    #[test]
    fn minor_3x3() {
        let a = matrix![
            3.0, 5.0, 0.0;
            2.0, -1.0, -7.0;
            6.0, -1.0, 5.0
        ];
        let b = a.submatrix(1, 0).unwrap();
        assert_eq!(25.0, b.determinate().unwrap());
        assert_eq!(25.0, a.minor(1, 0).unwrap());
    }

    #[test]
    fn minor_invalid_size() {
        assert_eq!(None, matrix![1.0, 2.0].minor(0, 0));
    }

    #[test]
    fn cofactor_3x3() {
        let a = matrix![
            3.0, 5.0, 0.0;
            2.0, -1.0, -7.0;
            6.0, -1.0, 5.0
        ];
        assert_eq!(-12.0, a.minor(0, 0).unwrap());
        assert_eq!(-12.0, a.cofactor(0, 0).unwrap());
        assert_eq!(25.0, a.minor(1, 0).unwrap());
        assert_eq!(-25.0, a.cofactor(1, 0).unwrap());
    }

    #[test]
    fn inversion1() {
        let a = matrix![
            6.0,  4.0,  4.0,  4.0;
            5.0,  5.0,  7.0,  6.0;
            4.0,  -9.0,  3.0,  -7.0;
            9.0,  1.0,  7.0,  -6.0
        ];
        assert_eq!(Some(-2120.0), a.determinate());
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
        assert_eq!(Some(0.0), a.determinate());
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
        assert_eq!(Some(532.0), a.determinate());
        assert_eq!(Some(-160.0), a.cofactor(2, 3));
        assert_feq!(-160.0/532.0, b[(3, 2)]);

        assert_eq!(Some(105.0), a.cofactor(3, 2));
        assert_feq!(105.0/532.0, b[(2, 3)]);
        assert_eq!(matrix![
            0.21805, 0.45113, 0.24060, -0.04511;
            -0.80827, -1.45677, -0.44361, 0.52068;
            -0.07895, -0.22368, -0.05263, 0.19737;
            -0.52256, -0.81391, -0.30075, 0.30639
        ], b);
    }
}

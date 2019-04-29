use super::iterator::*;
use super::util::*;
use std::ops::{Index,IndexMut,Mul};
use std::cmp::{PartialEq,Eq};

#[allow(unused_macros)]
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

#[allow(unused_macros)]
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
        if self.height != 2 && self.width != 2 {
            return None
        }
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];
        return Some((a * d) - (b * c))
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
        return Some(m)
    }

    pub fn minor(&self, y: usize, x: usize) -> Option<f32> {
        if self.height == 3 && self.width == 3 {
            return self.submatrix(y, x).and_then(|m| m.determinate())
        }
        return None
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
}

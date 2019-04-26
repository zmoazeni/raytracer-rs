use super::iterator::*;
use std::ops::{Index,IndexMut};

// Requires trailing ; character. Don't know if there's a way to make that last one optional
#[allow(unused_macros)]
macro_rules! matrix {
    ( $( $($x:expr),* ;)* ) => {
        {
            Matrix::with_values(vec![
                $(
                    vec![$($x),*]
                ),*
            ])
        }
    };
}

#[derive(Debug,Clone)]
pub struct Matrix {
    pub dimensions: (usize, usize),
    values: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        let row = vec![0.0; width];
        let mut v2 = Vec::with_capacity(height);
        for _ in 0..height {
            v2.push(row.clone());
        }

        Matrix { dimensions: (width, height), values: v2 }
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
        DimensionalIterator::matrix(self.dimensions)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_matrix() {
        let m = Matrix::new(4, 3);
        let (width, height) = m.dimensions;
        assert_eq!(height, 4);
        assert_eq!(width, 3);

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
}

use super::iterator::*;
use std::ops::{Index,IndexMut};

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
}

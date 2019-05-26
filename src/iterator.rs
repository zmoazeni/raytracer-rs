pub struct DimensionalIterator {
    dimensions: (usize, usize),
    position: (usize, usize),
    first_pass: bool,
    dimensions_reversed: bool,
}

impl DimensionalIterator {
    pub fn canvas(dimensions: (usize, usize)) -> DimensionalIterator {
        DimensionalIterator {
            dimensions: dimensions,
            position: (0, 0),
            first_pass: true,
            dimensions_reversed: false,
        }
    }

    pub fn matrix(dimensions: (usize, usize)) -> DimensionalIterator {
        DimensionalIterator {
            dimensions: dimensions,
            position: (0, 0),
            first_pass: true,
            dimensions_reversed: true,
        }
    }
}

fn reverse_tuple(tuple: (usize, usize), reverse: bool) -> (usize, usize) {
    if reverse {
        return (tuple.1, tuple.0);
    } else {
        return tuple;
    }
}

impl Iterator for DimensionalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let (width, height) = reverse_tuple(self.dimensions, self.dimensions_reversed);

        let (x, y) = self.position;

        if self.first_pass && x < width {
            self.first_pass = false;
            return Some(reverse_tuple(self.position, self.dimensions_reversed));
        }

        if x + 1 < width {
            self.position = (x + 1, y);
        } else if y + 1 < height {
            self.position = (0, y + 1);
        } else {
            return None;
        }

        return Some(reverse_tuple(self.position, self.dimensions_reversed));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterate_canvas() {
        let iter = DimensionalIterator::canvas((4, 2));
        let mut v = vec![];
        for coordinate in iter {
            v.push(coordinate);
        }
        assert_eq!(
            v,
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1)
            ]
        );
    }

    #[test]
    fn iterate_matrix() {
        let iter = DimensionalIterator::matrix((4, 2));
        let mut v = vec![];
        for coordinate in iter {
            v.push(coordinate);
        }
        assert_eq!(
            v,
            vec![
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 0),
                (2, 1),
                (3, 0),
                (3, 1),
            ]
        );
    }
}

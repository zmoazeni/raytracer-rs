mod ops;
mod transformations;

pub use ops::*;
pub use transformations::*;

#[derive(Debug, Clone)]
pub struct Matrix {
    height: usize,
    width: usize,
    values: Vec<Vec<f32>>,
}

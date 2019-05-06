#[macro_use] mod macros;

mod ops;
mod transformations;

pub use macros::*;
pub use ops::*;
pub use transformations::*;

#[derive(Debug,Clone)]
pub struct Matrix {
    height: usize,
    width: usize,
    values: Vec<Vec<f32>>,
}

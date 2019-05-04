#[macro_use] mod macros;

mod ops;
pub use macros::*;
pub use ops::*;

#[derive(Debug,Clone)]
pub struct Matrix {
    height: usize,
    width: usize,
    values: Vec<Vec<f32>>,
}

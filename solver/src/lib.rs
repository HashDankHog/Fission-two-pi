pub mod matrix;
pub mod parameter;
pub mod parse;
pub mod geometry;
pub mod vec;

pub trait Value {
    fn value(&self) -> f64;
}


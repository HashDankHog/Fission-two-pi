pub mod matrix;
pub mod parameter;
pub mod parse;
pub mod function;
pub mod geometry;
pub mod vec;
//TODO: remove and replace with From
/// # THIS WILL BE REMOVED SHORTLY
/// This trait is just a way to convert different generic numeric types into a common type,
/// which can be used by the matrix
/// With the exception of the parameter type, value is defined as:
/// ```
/// # pub trait Value: Into<f64> {
/// #   fn value(self) -> f64;
/// # }
/// impl<T: Into<f64>> Value for T {
///    fn value(self) -> f64 {
///        self.into()
///    }
/// }
/// ```
pub trait Value: Into<f64> {
    fn value(self) -> f64;
}

impl<T: Into<f64>> Value for T {
    fn value(self) -> f64 {
        self.into()
    }
}


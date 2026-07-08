use crate::parse::{interpret, simplify};
use dyn_clone::DynClone; //TODO: create my own hand rolled version once I understand why it works
use std::ops::{Add, Sub, Mul, Div, Neg};


/*TODO: traits
<T: From<bool> + 
 + AddAssign + 
 + SubAssign +
 + MulAssign +
 + DivAssign + Neg<Output = T> + Value + PartialOrd>
*/

//TODO: add parameterSet

dyn_clone::clone_trait_object!(Expression);
//from here to line 27 was written by claude
//TODO: replace with my own implementation, if possible
pub trait Expression: Fn(Vec<f64>) -> f64 + DynClone {}


impl<T> Expression for T
where
    T: 'static + Fn(Vec<f64>) -> f64 + Clone,
{}
// TODO: rewrite this doc segment to take advantage of code hiding
/// Arguably the core of the entirity of Parametrox
/// Internally, Parameter is just a unit struct that wraps around a trait object for a closure of Fn(Vec<f64>) -> f64
/// The power of this is that by getting around rusts orpahns rules and using a parser, we can evaluate arbitrary expressions at runtime
/// # Examples
/// ```
/// use solver::parameter::Parameter;
/// 
/// // here we define our set of inputs
/// let mut p = vec![0.0,1.0,2.0];
/// 
/// //box around the closure is nesscesary due to how rust handles trait objects
/// let x0 = Parameter(Box::new(|p| p[0] + 1.0));
/// 
/// // lets define another output parameter
/// let x1 = Parameter(Box::new(|p| p[[1] + 1.0));
/// 
/// assert_eq!(x0(p), 1.0);
/// assert_eq!(x1(p),2.0);
/// assert_eq!((x0/x1)(p), 0.5);
/// ```
pub struct Parameter(Box<dyn Expression>);

impl From<bool> for Parameter {
    fn from(value: bool) -> Self {
        match value {
            true =>  Parameter(Box::new(|_p| 1.0)),
            false => Parameter(Box::new(|_p| 0.0))
        }
    }
}
//this is going to be hard to implement
//since it might force me to add a value item to the parameter struct and I dont want to do that
impl Into<f64> for Parameter{
    fn into(self) -> f64 {
        self.value
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter(Box::new(move |_p| 0.0))
    }
}

impl Add for Parameter {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { 
        Parameter(Box::new(move |p| self.0(p.clone()) + rhs.0(p)))
    }
}

impl Sub for Parameter {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { 
        Parameter(Box::new(move |p| self.0(p.clone()) - rhs.0(p)))
    }
}

impl Mul for Parameter {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Parameter(Box::new(move |p| self.0(p.clone()) * rhs.0(p)))
    }
}

impl Div for Parameter {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Parameter(Box::new(move |p| self.0(p.clone()) / rhs.0(p)))
    }
}

impl Neg for Parameter {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Parameter(Box::new(move |p| self.0(p.clone()) * -1.0))
    }
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
    fn lt(&self, other: &Self) -> bool {
        if self.value < other.value { return true; }
        false
    }
    fn le(&self, other: &Self) -> bool {
        if self.value <= other.value { return true; }
        false
    }
    fn gt(&self, other: &Self) -> bool {
        if self.value > other.value { return true; }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        if self.value >= other.value { return true; }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_add_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 + parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("+")], value: 3.0 })
    }

    #[test]
    fn parameter_sub_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 - parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("-")], value: -1.0 })
    }

    #[test]
    fn parameter_mul_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 * parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("*")], value: 2.0 })
    }

    #[test]
    fn parameter_div_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 / parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("/")], value: 0.5 })
    }

}
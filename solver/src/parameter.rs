use std::sync::{LazyLock, Mutex};
use dyn_clone::DynClone; //TODO: create my own hand rolled version once I understand why it works
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::function::*;

static STEP: f64 = 0.00005;


static VALUES: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone)]
pub struct Parameters(pub Vec<Parameter>);
impl Parameters {
    pub fn update(&self) {
        let mut old_values = VALUES.lock().unwrap();
        let mut new_values = vec![0.0; old_values.len()];
        for index in 0..self.0.len() {
            new_values[index] = self.0[index].0(&old_values);
        }
        *old_values = new_values.clone();
    }
}

dyn_clone::clone_trait_object!(Expression);
//from here to line 27 was written by claude
//TODO: replace with my own implementation, if possible
pub trait Expression: Fn(&Vec<f64>) -> f64 + DynClone {}


impl<T> Expression for T
where
    T: 'static + Fn(&Vec<f64>) -> f64 + Clone,
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
/// let x1 = Parameter(Box::new(|p| p[1] + 1.0));
/// 
/// assert_eq!(x0.0(&p), 1.0);
/// assert_eq!(x1.0(&p),2.0);
/// 
/// //we can also change p
/// p[0] = 1.0;
/// assert_eq!(x0.0(&p), 2.0);
/// p[0] = 0.0;
/// 
/// assert_eq!((x0/x1).0(&p), 0.5);
/// ```
#[derive(Clone)]
pub struct Parameter(pub Box<dyn Expression>);

impl From<bool> for Parameter {
    fn from(value: bool) -> Self {
        match value {
            true =>  Parameter(Box::new(|_p| 1.0)),
            false => Parameter(Box::new(|_p| 0.0))
        }
    }
}

impl Into<f64> for Parameter{
    fn into(self) -> f64 {
        self.0(&*VALUES.lock().unwrap())
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
        Parameter(Box::new(move |p| self.0(p) + rhs.0(p)))
    }
}

impl Sub for Parameter {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { 
        Parameter(Box::new(move |p| self.0(p) - rhs.0(p)))
    }
}

impl Mul for Parameter {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Parameter(Box::new(move |p| self.0(p) * rhs.0(p)))
    }
}

impl Div for Parameter {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Parameter(Box::new(move |p| self.0(p) / rhs.0(p)))
    }
}

impl Neg for Parameter {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Parameter(Box::new(move |p| self.0(p) * -1.0))
    }
}

impl Sin for Parameter {
    fn sin(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).sin()))
    }
}

impl Cos for Parameter {
    fn cos(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).cos()))
    }
}

impl Tan for Parameter {
    fn tan(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).tan()))
    }
}

impl ArcSin for Parameter {
    fn arc_sin(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).asin()))
    }
}

impl ArcCos for Parameter {
    fn arc_cos(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).acos()))
    }
}

impl ArcTan for Parameter {
    fn arc_tan(self) -> Self {
        Parameter(Box::new(move |p| self.0(p).atan()))
    }
}

impl Pow for Parameter {
    fn pow(self, other: Self) -> Self {
        Parameter(Box::new(move |p| self.0(p).powf(other.0(p))))
    }
}

use crate::vec::add;
impl Diff for Parameter {
    /// Returns a Parameter that approximates the partial deriviatiave at any point
    /// While differentiation is closed, the anonomyous nature of closures makes it, at least to my knowledge, impossible to symbollically/automatically compute
    /// derivitaves.
    /// # Examples
    /// ```
    /// use solver::parameter::Parameter;
    /// use solver::function::Diff;
    /// // here we define our set of inputs
    /// let mut p = vec![1.0];
    /// 
    /// // p0 + 1
    /// let x0 = Parameter(Box::new(|p| p[0] + 1.0));
    /// 
    /// // dx0/dp0
    /// let x1 = x0.diff(0);
    /// 
    /// //since we cant have exact computation, we instead check for the error of the derivitave
    /// assert!(x1.0(&p)-1.0 < 0.00001);
    /// ```
    fn diff(self, of: usize) -> Parameter {
        //I HATE ITERATORS
        Parameter(Box::new(move |p| (
            self.0(&add(p,&(p.into_iter().enumerate().map(|(index, _val )| if index == of { STEP } else { 0.0 }).collect())).unwrap()) - self.0(p)) / STEP))
    }
}



impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        self.0(&val) == other.0(&val)
    }
    fn ne(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        self.0(&val) != other.0(&val)
    }
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let val = VALUES.lock().unwrap();
        self.0(&val).partial_cmp(&other.0(&val))
    }
    fn lt(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        if self.0(&val) < other.0(&val) { return true; }
        false
    }
    fn le(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        self.0(&val) <= other.0(&val)
    }
    fn gt(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        self.0(&val) > other.0(&val)
    }
    fn ge(&self, other: &Self) -> bool {
        let val = VALUES.lock().unwrap();
        self.0(&val) >= other.0(&val)
    }
}


//! # Vec
//! Adds just a couple of simple vector operations for numeric vectors

use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::Value;

#[derive(PartialEq, Debug, Clone)]
/// Is a wrapper for a vector of numeric type.
/// # Examples
/// ```
/// use solver::vec::NumVec;
/// let A = NumVec(vec![0,1,4]);
/// let B = NumVec(vec![1,1,-1]);
/// let C = A.add(&B).unwrap();
/// assert_eq!(C.dot_prod(&B), Some(0));
/// ```
pub struct NumVec<T: From<bool> + 
Add<Output = T>  + 
Sub<Output = T>  +
Mul<Output = T>  +
Div<Output = T>  + Neg<Output = T> + Clone + Value + PartialOrd>(pub Vec<T>);

impl <T: From<bool> + 
Add<Output = T>  + 
Sub<Output = T>  +
Mul<Output = T>  +
Div<Output = T>  + Neg<Output = T> + Clone + Value + PartialOrd> NumVec<T> {
    /// Calculates the dot product of two numeric vectors
    /// # Examples
    /// ```
    /// use solver::vec::NumVec;
    /// let A = NumVec(vec![1.0,1.0]);
    /// assert_eq!(A.dot_prod(&A), Some(2.0));
    /// ```
    /// # Errors
    /// If one of the vecs is a different length than the other, the function will return None
    pub fn dot_prod(&self, other: &Self) -> Option<T> {
        if other.0.len() != self.0.len() { 
            return None; 
        }

        let mut result = self.0[0].clone() * other.0[0].clone();

        for index in 1..self.0.len() {
            result = result + (self.0[index].clone() * other.0[index].clone());
        }

        Some(result)
    }
    /// Calculates the cross product of two numeric vectors
    /// # Examples
    /// ```
    /// use solver::vec::NumVec;
    /// 
    /// let A = NumVec(vec![1.0,0.0,0.0]);
    /// let B = NumVec(vec![0.0,1.0,0.0]);
    ///
    /// assert_eq!(A.cross_prod(&B), Some(NumVec(vec![0.0,0.0,1.0])));
    /// ```
    /// # Errors
    /// If either one of the vectors are not of length 3, the function will return None
    pub fn cross_prod(&self, other: &Self) -> Option<Self> {
        if self.0.len() == 3 && other.0.len() == 3 {
            Some(Self(vec![
                self.0[1].clone() * other.0[2].clone() - self.0[2].clone() * other.0[1].clone(),
                self.0[2].clone() * other.0[0].clone() - self.0[0].clone() * other.0[2].clone(), 
                self.0[0].clone() * other.0[1].clone() - self.0[1].clone() * other.0[0].clone()]))
        } else {
            None
        }
    }

    /// finds the index of the largest item in a slice of a numeric vector
    /// If there are multiple largest items, it will return the first index of that maximum
    /// # Examples
    /// ```
    /// use solver::vec::NumVec;
    /// let A = NumVec(vec![1,2,3]);
    /// assert_eq!(A.arg_max(), 2);
    /// ```
    // TODO: swap back to vec: &Vec<T>
    pub fn arg_max(&self) -> usize {
        let mut max: (T, usize) = (self.0[0].clone(), 0);
        for element in 0..self.0.len() {
            if self.0[element] > max.0 {
                max = (self.0[element].clone(), element);
            }
        }
        max.1
    }

    /// finds the index of the largest item in a slice of a numeric vector
    /// If there are multiple largest items, it will return the first index of that maximum
    /// # Examples
    /// ```
    /// use solver::vec::NumVec;
    /// let A = NumVec(vec![1,2,1]);
    /// assert_eq!(A.arg_min(), 0);
    /// ```
    // TODO: swap back to vec: &Vec<T>
    pub fn arg_min(&self) -> usize {
        let mut min: (T, usize) = (self.0[0].clone(), 0);
        for element in 0..self.0.len() {
            if self.0[element] < min.0 {
                min = (self.0[element].clone(), element);
            }
        }
        min.1
    }

    /// adds vectors twin
    pub fn add(&self, other: &Self) -> Option<Self> {
        if self.0.len() != other.0.len() { return None; }
        let mut result = self.0.clone();
        for element in 0..other.0.len() {
            result[element] = result[element].clone() + other.0[element].clone();
        }
        Some(Self(result))
    } 
}

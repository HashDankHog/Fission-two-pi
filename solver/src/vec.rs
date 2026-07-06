//! # Vec
//! Adds just a couple of simple vector operations for numeric vectors

use std::ops::{Add, Sub, Mul};

/// Calculates the dot product of two numeric vectors
/// # Examples
/// ```
/// use solver::vec::dot_prod;
/// let A = vec![1.0,1.0];
/// assert_eq!(dot_prod(&A, &A), Some(2.0));
/// ```
/// # Errors
/// If one of the vecs is a different length than the other, the function will return None
pub fn dot_prod<T: Mul<Output = T> + Add<Output = T> + Clone>(vec_1: &Vec<T>, vec_2: &Vec<T>) -> Option<T> {
    match vec_2.len() {
        l if l == vec_1.len() => Some({
            //TODO: replace with an iterator
            let mut prod = vec_1[0].clone()*vec_2[0].clone();            
            for i in 1..(l) {
                prod = prod + vec_1[i].clone()*vec_2[i].clone();
            }
            prod
        }),
        _ => None
    }
}


/// Calculates the cross product of two numeric vectors
/// # Examples
/// ```
/// use solver::vec::cross_prod;
/// 
/// let A = vec![1.0,0.0,0.0];
/// let B = vec![0.0,1.0,0.0];
///
/// assert_eq!(cross_prod(&A,&B), Some(vec![0.0,0.0,1.0]));
/// ```
/// # Errors
/// If either one of the vectors are not of length 3, the function will return None
pub fn cross_prod<T: Mul<Output = T> + Sub<Output = T> + Clone>(vec_1: &Vec<T>, vec_2: &Vec<T>) -> Option<Vec<T>> {
    match vec_1.len() {
        3 if vec_2.len() == 3 => { Some(vec![
                vec_1[1].clone() * vec_2[2].clone() - vec_1[2].clone() * vec_2[1].clone(),
                vec_1[2].clone() * vec_2[0].clone() - vec_1[0].clone() * vec_2[2].clone(), 
                vec_1[0].clone() * vec_2[1].clone() - vec_1[1].clone() * vec_2[0].clone()])},
        _ => None
    }
}

/// finds the index of the largest item in a slice of a numeric vector
/// If there are multiple largest items, it will return the first index of that maximum
/// # Examples
/// ```
/// use solver::vec::arg_max;
/// let A = vec![1,2,3];
/// assert_eq!(arg_max(&A[0..]), 2);
/// ```
// TODO: swap back to vec: &Vec<T>
pub fn arg_max<T: std::cmp::PartialOrd + Clone>(vec: &[T]) -> usize {
    let mut max: (T, usize) = (vec[0].clone(), 0);
    for element in 0..vec.len() {
        if vec[element] > max.0 {
            max = (vec[element].clone(), element);
        }
    }
    max.1
}

/// finds the index of the largest item in a slice of a numeric vector
/// If there are multiple largest items, it will return the first index of that maximum
/// # Examples
/// ```
/// use solver::vec::arg_min;
/// let A = vec![1,2,1];
/// assert_eq!(arg_min(&A[0..]), 0);
/// ```
// TODO: swap back to vec: &Vec<T>
pub fn arg_min<T: std::cmp::PartialOrd + Clone>(vec: &[T]) -> usize {
    let mut min: (T, usize) = (vec[0].clone(), 0);
    for element in 0..vec.len() {
        if vec[element] < min.0 {
            min = (vec[element].clone(), element);
        }
    }
    min.1
}

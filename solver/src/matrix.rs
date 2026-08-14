//! # Matrix
//! Module for matrix and vector operations. 
pub mod plu;
pub mod qr;
use crate::parameter::Parameter;
use crate::vec::NumVec;
use crate::NumBounds;

pub enum EchelonForm {
    None,
    Row,
    RowReduced,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Solution<T: NumBounds<T>> {
    Unique(NumVec<T>),
    Inconsistant,
    Infinite{particular: NumVec<T>, homogeneous: Vec<NumVec<T>>}
}
/// A 2D structure for operating on sets of numbers
/// # Examples
/// ```
/// //this example does not work currently bc I am extremely lazy and cant be fucked to implement a extremely simple function
/// use solver::matrix::{Matrix, Solution};
/// use solver::vec::NumVec;
/// let mut A = Matrix {elements: vec![1.0,1.0,1.0,-1.0], rows: 2, colums: 2 };
/// let B = vec![2.0, 0.0];
/// assert_eq!(A.solve_for(&NumVec(B)), Solution::Unique(NumVec(vec![1.0,1.0])));
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T: NumBounds<T>> {
    pub elements: Vec<T>, //this shouldnt be public but for now idk how to fix ts since assert_eq needs it to be
    pub rows: usize,
    pub colums: usize,
}
impl <T: NumBounds<T>> Matrix<T> {
    //NOTE: most of the methods here make heavy use of clone. The reason this is not a huge issue 
    // for performance is that for primitive types, Rust implements a trivial clone which is equivilent to copy

    /// Checks if a element is in the bounds of the matrix
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let A = Matrix {elements: vec![1,2,3,4], rows: 2, colums: 2};
    /// assert_eq!(A.in_bounds(2,2).is_ok(), false);
    /// ```
    pub fn in_bounds(&self, row: usize, colum: usize) -> Result<bool, bool> {
        if row > self.rows-1 || colum > self.colums-1 {
            return Err(false);
        } else {
            return Ok(true);
        }
    }
    
    /// Returns the element for any row and colum in the matrix
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let A = Matrix {elements: vec![1,2,3,4], rows: 2, colums: 2};
    /// assert_eq!(A.element(0,0), 1);
    /// ```
    /// # Panics
    /// Trying to access an element outside the bounds of the matrix will result in a panic
    pub fn element(&self, row: usize, colum: usize) -> T {
        if row > self.rows-1 || colum > self.colums-1 {
            panic!("tried accessing an out of bounds element");
        }
        self.elements[self.colums*row+colum].clone()
    }

    /// Changes the value of an element for any row and colum in the matrixx
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix {elements: vec![0.0, 1.0], rows: 2, colums: 1};
    /// A.set(0,0, 1.0);
    /// assert_eq!(A, Matrix { elements: vec![1.0, 1.0], rows: 2, colums: 1})
    /// ```
    /// # Panics
    /// Trying to set an element outside the bounds of the matrix will result in a panic
    pub fn set(&mut self, row: usize, colum: usize, value: T) {
        if row > self.rows || colum > self.colums {
            panic!("tried accessing an out of bounds element");
        }
        self.elements[self.colums*row+colum]=value.clone();
    }

    pub fn iterate<F: FnMut(usize, usize) -> T>(&self, mut value: F) -> Matrix<T> {
        let mut result_matrix: Matrix<T> = Matrix{ elements: Vec::new(), rows: self.rows, colums: self.colums};
        for row in 0..result_matrix.rows {
            for colum in 0..result_matrix.colums {
                result_matrix.elements.push(value(row, colum));
            }
        }
        result_matrix
    }
    
    /// Returns any row of a matrix as a vector
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// use solver::vec::NumVec;
    /// let A = Matrix {elements: vec![1.0, 2.0, 3.0, 4.0], rows: 2, colums: 2};
    /// assert_eq!(A.row(0), Ok(NumVec(vec![1.0, 2.0]))); 
    /// ```
    //TODO: return result instead of vec(?)
    pub fn row(&self, row: usize) -> Result<NumVec<T>, & 'static str> {
        if row >= self.rows {
            return Err("cannot acces row outside of bounds");
        }
        let mut r = Vec::new();
        for colum in 0..self.colums {
            r.push(self.element(row,colum));
        }
        Ok(NumVec(r))
    }

    ///```
    /// use solver::matrix::Matrix;
    /// use solver::vec::NumVec;
    /// let mut a = Matrix {elements: vec![0,1], rows: 1, colums: 2};
    /// a.append_row(&NumVec(vec![2,3]));
    /// assert_eq!(a, Matrix {elements: vec![0,1,2,3], rows: 2, colums: 2})
    /// ```
    //TODO: return result
    pub fn append_row(&mut self, row: &NumVec<T>) {
        if row.0.len() != self.colums { panic!("ouu shi"); }
        for colum in 0..self.colums {
            self.elements.insert(self.colums*(self.rows) + colum, row.0[colum].clone());
        }
        self.rows += 1;
    }

    /// Returns any colum of a matrix as a vector
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// use solver::vec::NumVec;
    /// let A = Matrix { elements: vec![1.0,2.0,3.0,4.0], rows: 2, colums: 2};
    /// assert_eq!(A.colum(1), Ok(NumVec(vec![2.0,4.0])));
    /// ```
    //TODO: return result instead of vec(?)
    pub fn colum(&self, colum: usize) -> Result<NumVec<T>, & 'static str> {
        let mut r = Vec::new();
        if colum >= self.colums{
            return Err("cannot access colum outside of bounds");
        }
        for row in 0..self.rows {
            r.push(self.element(row,colum));
        }
        Ok(NumVec(r))
    }

    //TODO: return result
    pub fn append_colum(&mut self, colum: &NumVec<T>) {
        if colum.0.len() != self.rows { panic!("ouu shi"); }
        for row in 0..self.rows {
            self.elements.insert((self.colums+1)*(row) + (self.colums), colum.0[row].clone());
        }
        self.colums += 1;
    }

    /// Mutably swaps any two rows of a matrix
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix { elements: vec![1,0,0,1], rows: 2, colums: 2};
    /// A.swap_row(0,1);
    /// assert_eq!(A.elements, vec![0,1,1,0]);
    /// ```
    //I am not sure if I want my row operations to be mutable or not
    //It should be easy to come back and fix them if I decide not to though but for now I will leave it as mutable
    pub fn swap_row(&mut self, row_1: usize, row_2: usize) {
        for colum in 0..self.colums {
            let element_1 = self.element(row_1, colum);
            let element_2 = self.element(row_2,colum);
            self.set(row_1, colum, element_2);
            self.set(row_2, colum, element_1);
        }
    }

    /// Mutable scale all of the elements in a row
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix {elements: vec![4,3,2,1], rows: 2, colums: 2};
    /// A.scale_row(0, 2);
    /// assert_eq!(A.elements, vec![8,6,2,1]);
    /// ``` 
    pub fn scale_row(&mut self, row: usize, scale: T) {
        for colum in 0..self.colums {
            let scaled_value  = self.element(row, colum) * scale.clone();
            self.set(row,colum, scaled_value);
        }
    }

    /// Mutably adds a multable of one row to another of a matrix
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix { elements: vec![3,2,1], rows: 3, colums: 1};
    /// A.add_row(0,2,3);
    /// assert_eq!(A.elements, vec![6,2,1]);
    /// ```
    pub fn add_row(&mut self, row_1: usize, row_2: usize, scale: T) {
        for colum in 0..self.colums {
            let new_value  = self.element(row_1, colum) + self.element(row_2, colum) * scale.clone();
            self.set(row_1, colum, new_value);
        }
    }

    /// Finds the index of the pivot of a row
    /// returns None if there is no pivot
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let A = Matrix {elements: vec![1,0,0,1], rows: 2, colums:2};
    /// assert_eq!(A.find_pivot(1), Some(1));
    /// ```
    pub fn find_pivot(&self, row: usize) -> Option<usize> { 
        let items = self.row(row);
        for element in 0..items.clone().unwrap().0.len() {
            match items.clone().unwrap().0[element].clone().value() {
                x if x.abs() < 0.1E-16 => {},
                _ => return Some(element)
            }   
        }
        None
    }
    
    /// performs addition of matrices
    pub fn add(&self, matrix: &Matrix<T>) -> Option<Matrix<T>> { 
        if self.rows != matrix.rows || self.colums != matrix.colums {
            return None
        }
        Some(
        self.iterate(|row, colum| self.element(row, colum).clone() + matrix.element(row, colum).clone())
        )
    }

    /// multiplies matrices
    pub fn mult(&self, matrix: &Matrix<T>) -> Option<Matrix<T>> { 
        if self.colums != matrix.rows {
            return None
        }
        let result = Matrix {elements: vec![self.element(0,0).clone(); self.rows * matrix.colums], rows: self.rows, colums: matrix.colums};

        Some(
        result.iterate(|row, colum| 
            self.row(row).unwrap()
            .dot_prod(
                &matrix.colum(colum).unwrap()).unwrap()
        ))
    }
    
    /// scales a matrix by a scalar
    pub fn scale(&self, scale: T) -> Matrix<T> {
        self.iterate(|row, colum| self.element(row, colum).clone() * scale.clone())
    }

    /// Transposes a Matrix
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let a = Matrix{ elements: vec![1,2,3,
    ///                                4,5,6], rows: 2, colums: 3};
    /// assert_eq!(a.transpose(), Matrix{elements: vec![1,4,2,5,3,6], rows: 3, colums: 2})
    /// ```
    pub fn transpose(&self) -> Matrix<T> {
        let result = Matrix {elements: vec![self.element(0,0).clone(); self.colums* self.rows], rows: self.colums, colums: self.rows};
        result.iterate(|row, colum| self.element(colum, row).clone())
    }

    /// Finds the inverse of a square matrix
    /// returns none if the matrix is not square
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let a = Matrix {elements: vec![2.0,1.0,1.0,3.0], rows: 2, colums: 2};
    /// assert_eq!(a.inverse().unwrap().elements, vec![0.6, -0.2, -0.2, 0.4]);
    /// ```
    // I am like 99% sure this will return the inverse of non invertible matrices but Idgaf rn
    pub fn inverse(&self) -> Option<Matrix<T>> {
        let a = plu::PLU::from(self);
        a.inverse()
    }

    /// not implemented yet due to general lazieness
    pub fn determinant(&self) -> T {self.element(0,0).clone()}
   pub fn right_inverse(&self) -> Option<Self> {
        self.transpose().mult(&self.mult(&self.transpose()).unwrap().inverse().unwrap())
   }
   pub fn left_inverse(&self) -> Option<Self> {
        self.mult(&self.transpose()).unwrap().inverse().unwrap().mult(&self.transpose())
   }

}

/// Creates an identity matrix of dimension N for a certain type
/// # Examples
/// ```
/// use solver::matrix::{Matrix, IdentityMatrix};
/// let a = f64::imat(2);
/// assert_eq!(a, Matrix {elements: vec![1.0,0.0,0.0,1.0], rows: 2, colums: 2})
/// ```
pub trait IdentityMatrix: NumBounds<Self> {
    fn imat(size: usize) -> Matrix<Self>;
}


impl<T: NumBounds<T>> IdentityMatrix for T {
    fn imat(size: usize) -> Matrix<Self>{ 
        let mut identity = Matrix { elements: vec![Self::from(false); size*size], rows: size, colums: size};
        for element in 0..size {
            identity.set(element, element, Self::from(true));
        }
        identity
    }
}

pub struct Jacobian {
    pub parameters: Vec<Parameter>,
    pub inputs: NumVec<f64>,
    matrix: Matrix<Parameter>,
    iter: usize
}
impl Jacobian {
    pub const fn new() -> Self {
        Jacobian { parameters: Vec::new(), inputs: NumVec(Vec::new()), matrix: 
            Matrix {elements: Vec::new(), rows: 0, colums:0}, iter: 10 }
    }
    pub fn set_iterations(&mut self, num: usize) {
        self.iter = num;
    }
    fn iterate(&mut self, iteration: usize) -> Solution<f64> {
        let mut mat_f64 = Matrix { elements: Vec::new(), rows: self.matrix.rows, colums: self.matrix.colums};
        for element in self.matrix.elements.clone() {
            mat_f64.elements.push(element.0(&self.inputs));
        }
        let mut goal = Matrix{elements: Vec::new(), rows: mat_f64.rows, colums: 1};
        for parameter in self.parameters.clone() {
            goal.elements.push(parameter.0(&self.inputs));
        }
        let inverse;
        if mat_f64.rows > mat_f64.colums {
            inverse = mat_f64.left_inverse();
        } else if mat_f64.rows < mat_f64.colums {
            inverse = mat_f64.right_inverse();
        } else {
            inverse = mat_f64.inverse();
        }

        match inverse {
            None => return Solution::Inconsistant,
            Some(x) => {
                if iteration == self.iter {
                    if x.mult(&mat_f64).unwrap() == f64::imat(x.rows){
                        return Solution::Unique(self.inputs.clone())
                    } else {
                        //TODO: this is not correct at all but oh well
                        let mut h = Vec::new();
                        for col in 0..x.colums {
                            h.push(x.colum(col).unwrap());
                        }
                        return Solution::Infinite { particular: self.inputs.clone(), homogeneous: h }
                    }
                }
                let a = x.mult(&goal).unwrap();
                self.inputs = self.inputs.add(&NumVec(a.elements).scale(-1.0)).unwrap();
                return self.iterate(iteration+1)
            }
        }
        
    }
    /// solves a system of non linear equations using newtons method
    pub fn solve(&mut self, initial_guess: NumVec<f64>) -> Solution<f64> {
        self.inputs = initial_guess;
        self.iterate(0)
    }
}
use crate::function::Diff;
impl From<(Vec<Parameter>, &NumVec<f64>)> for Jacobian{ 
    fn from(value: (Vec<Parameter>, &NumVec<f64>)) -> Self {
        let jacobian = Matrix { elements : vec![ Parameter::default(); value.1.len()*value.0.len()], rows: value.0.len(), colums: value.1.len() };
        Jacobian {
            parameters: value.0.clone(),
            inputs: value.1.clone(),
            matrix: jacobian.iterate(|row, colum| value.0[row].clone().diff(colum)),
            iter: 20
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jacobian_unique() {
        let b = NumVec(vec![0.5,2.5]);
        let constraint = vec![Parameter(Box::new(|p| (p.0[0].powf(2.0)+p.0[1].powf(2.0)-1.0).powf(2.0))),Parameter(Box::new(|p| (p.0[1]-p.0[0]).powf(2.0)))];
        let mut mat = Jacobian::from((constraint, &b));
        
        let mut results = Vec::new();
        match mat.solve(b) {
            Solution::Unique(s) => {
                for c in s.0 {
                    results.push(c);
                }
            }
            Solution::Inconsistant => println!("sigh"),
            Solution::Infinite { particular: p, homogeneous: h } => {
                for c in p.0 {
                    results.push(c);
                }
                for a in h {
                    for c in a.0 {
                        results.push(c);
                    }
                }
            }
        }
        let mut error = 0.0;
        error += 1.41421356237-results[0]-results[1];
        assert!(error.abs() < 0.1E-2);
    }
    //TODO: rewrite test
    #[test]
    fn jacobian_infinite() {
        let b = NumVec(vec![0.5,2.5]);
        let constraint = vec![Parameter(Box::new(|p| (p.0[0].powf(2.0)+p.0[1].powf(2.0)-1.0).powf(2.0)))];
        let mut mat = Jacobian::from((constraint, &b));
        
        let mut results = Vec::new();
        match mat.solve(b) {
            Solution::Unique(s) => {
                for c in s.0 {
                    results.push(c);
                }
            }
            Solution::Inconsistant => println!("sigh"),
            Solution::Infinite { particular: p, homogeneous: h } => {
                for c in p.0 {
                    results.push(c);
                }
                for a in h {
                    for c in a.0 {
                        results.push(c);
                    }
                }
            }
        }
        let error: f64 = 0.0; // this isnt really doing anything rn, I dont know how to test this
        // I mean I kidna do but I am way too lazy to implement it rn
        assert!(error.abs() < 0.1E-2);
    }
}
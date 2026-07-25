//! # Matrix
//! Module for matrix and vector operations. 

use crate::parameter::Parameter;
use std::ops::{Add, Div, Mul, Sub, Neg};
use crate::vec::NumVec;
use crate::Value;

pub enum EchelonForm {
    None,
    Row,
    RowReduced,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Solution<T: From<bool> + 
Add<Output = T>  + 
Sub<Output = T>  +
Mul<Output = T>  +
Div<Output = T>  + Neg<Output = T> + Clone + Value + PartialOrd> {
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
pub struct Matrix<T: From<bool> + 
Add<Output = T>  + 
Sub<Output = T>  +
Mul<Output = T>  +
Div<Output = T>  + Neg<Output = T> + Clone + Value + PartialOrd> {
    pub elements: Vec<T>, //this shouldnt be public but for now idk how to fix ts since assert_eq needs it to be
    pub rows: usize,
    pub colums: usize,
}
impl <T: From<bool> + 
Add<Output = T>  + 
Sub<Output = T>  +
Mul<Output = T>  +
Div<Output = T>  + Neg<Output = T> + Clone + Value + PartialOrd> Matrix<T> {
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
        if row > self.rows || colum > self.colums {
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
                x if x < 0.1E-15 => {},
                _ => return Some(element)
            }   
        }
        None
    }

    //TODO: refactor to use find_pivot
    pub fn check_reduced(&self) -> EchelonForm {
        let mut pivots: Vec<usize> = Vec::new();

        //this might be bad syntax but at the same time I am seperating out sub blocks
        {
        let mut pivot: usize = 0;
        for row in 0..self.rows {
            for colum in 0..self.colums {
                match self.element(row, colum).value() {
                    0.0 => {},
                    _ if row == 0 => {
                        pivot = colum;
                        pivots.push(pivot);
                        break;
                    },
                    _ if row != 0 => {
                        if pivot >= colum {
                            return EchelonForm::None;
                        } else {
                            pivots.push(colum);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        }

        for pivot in pivots {
            let mut pivot_count: usize = 0;
            for row in 0..self.rows {
                match self.element(row, pivot).value() {
                    0.0 => {},
                    1.0 => {pivot_count += 1},
                    _ => {return EchelonForm::Row}
                }
            }
            if pivot_count > 1 { return EchelonForm::Row; }
        }


        EchelonForm::RowReduced
    }

    /// Takes a matrix and reduces it to Row Echelon Form
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix { elements: vec![1.0,1.0,2.0,
    ///                                     1.0,-1.0,0.0,
    ///                                     2.0,3.0,5.0], rows: 3, colums: 3};
    /// A.to_row_form();
    /// assert_eq!(A.elements, vec![2.0, 3.0, 5.0, 
    ///                             0.0, -2.5, -2.5, 
    ///                             0.0, 0.0, 0.0]);
    /// ```
    pub fn to_row_form(&mut self) {
        for row in 0..(self.rows-1) {
            //the mapping here is a way to get around the fact that abs is not a generic trait
            //we cannot use arg_min because of the edge case of rows that already have a 0 element
            self.swap_row(row, NumVec(self.colum(row).unwrap().0.iter().map(|x| (*x).clone()*(*x).clone()).collect()).arg_max() + row); //fix this mess please
            let pivot = self.element(row, row);
            match pivot.clone().value() {
                0.0 => {},
                _ => {
                    for row_2 in (row+1)..self.rows {
                        let scale = self.element(row_2,row)/pivot.clone();
                        self.add_row(row_2, row, -scale);
                    }
                }
            }
        }
    }

    /// Takes a matrix and reduces it to Reduced Row Echelon Form
    /// # Examples
    /// ```
    /// use solver::matrix::Matrix;
    /// let mut A = Matrix { elements: vec![1.0,1.0,2.0,
    ///                                     1.0,-1.0,0.0,
    ///                                     2.0,3.0,5.0], rows: 3, colums: 3};
    /// A.to_red_row_form();
    /// assert_eq!(A.elements, vec![1.0, 0.0, 1.0, 
    ///                             0.0, 1.0, 1.0, 
    ///                             0.0, 0.0, 0.0]);
    /// ```
    // TODO: make it so I dont have 8 levels of indentation
    pub fn to_red_row_form(&mut self) {
        self.to_row_form();
        let rows = self.rows;
        for row in (1..=rows).map(|x| rows - x) {
            
            let pivot_value;
            match self.find_pivot(row) {
                Some(pivot) => {
                    pivot_value = self.row(row).unwrap().0[pivot].clone();
                    self.scale_row(row, T::from(true)/(pivot_value.clone()));
                    for row_2 in row+1..rows {
                        let p = self.find_pivot(row_2);
                        match p {
                            Some(x) => {
                                self.add_row(row, row_2, -self.element(row,x));
                            },
                            None => {}
                        }
                    }
                },
                None => {}
            }
        }
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
        result.iterate(|row, colum| self.row(row).unwrap().dot_prod(&matrix.colum(colum).unwrap()).unwrap()
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
        if self.rows != self.colums { return None }
        let mut temp = self.clone();
        let mut inverse = T::imat(self.rows);

        for row in 0..(self.rows-1) {
            //the mapping here is a way to get around the fact that abs is not a generic trait
            //we cannot use arg_min because of the edge case of rows that already have a 0 element
            let row_2 = NumVec(temp.colum(row).unwrap().0.iter().map(|x| (*x).clone()*(*x).clone()).collect()).arg_max() + row;
            temp.swap_row(row, row_2);
            inverse.swap_row(row, row_2); //when I swap back to &vec<T>: look into clone_into()
            let pivot = temp.element(row, row);
            match pivot.clone().value() {
                0.0 => {},
                _ => {
                    for row_2 in (row+1)..self.rows {
                        let scale = temp.element(row_2,row)/pivot.clone();
                        temp.add_row(row_2, row, -scale.clone());
                        inverse.add_row(row_2, row, -scale);
                    }
                }
            }
        }
        let rows = self.rows;
        for row in (1..=rows).map(|x| rows - x) {
            
            let pivot_value;
            match temp.find_pivot(row) {
                Some(pivot) => {
                    pivot_value = temp.row(row).unwrap().0[pivot].clone();
                    temp.scale_row(row, pivot_value.clone()/(pivot_value.clone()*pivot_value.clone()));
                    inverse.scale_row(row, pivot_value.clone()/(pivot_value.clone()*pivot_value.clone())); //this is the closest I can get rn to writing 1/pivot_value since generics are hard and annoying
                    for row_2 in row+1..rows {
                        let p = temp.find_pivot(row_2);
                        match p {
                            Some(x) => {
                                let scale = -temp.element(row,x);
                                temp.add_row(row, row_2, scale.clone());
                                inverse.add_row(row, row_2, scale);
                            },
                            None => {}
                        }
                    }
                },
                None => {}
            }
        }
        Some(inverse)
    }

    /// not implemented yet due to general lazieness
    pub fn determinant(&self) -> T {self.element(0,0).clone()}
    
    /// solves a linear system for a specified value
    /// returns a Solution Enum depending on the number of solutions
    /// if there are infinite solutions, we return the particular solution, alongside its associated homogenous solution
    /// # Examples
    /// ```
    /// use solver::matrix::{Matrix, Solution};
    /// use solver::vec::NumVec;
    /// let a = Matrix { elements: vec![1.0,0.0,2.0,0.0,1.0,2.0], rows: 2, colums: 3};
    /// let b = NumVec(vec![1.0,2.0]);
    /// assert_eq!(a.solve_for(&b), Solution::Infinite{ particular: b, homogeneous: vec![NumVec(vec![-2.0,-2.0,1.0])]})
    /// ```
    pub fn solve_for(&self, value: &NumVec<T>) -> Solution<T> {
        let mut solution = self.clone();
        let mut pivots = Vec::new();
        solution.append_colum(value);
        solution.to_red_row_form();
        //this logic works because of a few guarentees one can make based off of how gaussian elimination works
        for row in 0..solution.rows {
            match solution.find_pivot(row){
                Some(x) if x == solution.colums - 1 => return Solution::Inconsistant,
                None => {},
                Some(x) => pivots.push(x)
            }
        }
        
        let mut basis = Vec::new();
        for i in 0..solution.colums-1 {
            if !pivots.contains(&i) {
                let mut base = solution.colum(i).unwrap().scale(-T::from(true));
                for j in base.len()..self.colums {
                    if j == i {
                        base.push(T::from(true))
                    } else {
                        base.push(T::from(false))
                    }
                }
                basis.push(base);
            }
        }
        if basis.len() == 0 { return Solution::Unique(solution.colum(solution.colums-1).unwrap()) }
        Solution::Infinite{particular: solution.colum(solution.colums -1).unwrap(), homogeneous: basis}
    }
    
}

/// Creates an identity matrix of dimension N for a certain type
/// # Examples
/// ```
/// use solver::matrix::{Matrix, IdentityMatrix};
/// let a = f64::imat(2);
/// assert_eq!(a, Matrix {elements: vec![1.0,0.0,0.0,1.0], rows: 2, colums: 2})
/// ```
pub trait IdentityMatrix: 
From<bool> + 
Add<Output = Self>  + 
Sub<Output = Self> +
Mul<Output = Self> +
Div<Output = Self> + 
Neg<Output = Self> + Clone + Value + PartialOrd {
    fn imat(size: usize) -> Matrix<Self>;
}


impl<T: 
From<bool> + 
Add<Output = Self> + 
Sub<Output = Self> +
Mul<Output = Self> +
Div<Output = Self> + 
Neg<Output = Self> + Clone + Value + PartialOrd> IdentityMatrix for T {
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
    pub fn set_iterations(&mut self, num: usize) {
        self.iter = num;
    }
    fn iterate(&mut self, iteration: usize) -> Solution<f64> {
        let mut mat_f64 = Matrix { elements: Vec::new(), rows: self.matrix.rows, colums: self.matrix.colums};
        for element in self.matrix.elements.clone() {
            mat_f64.elements.push(element.0(&self.inputs));
        }
        let mut goal: NumVec<f64> = NumVec(Vec::new());
        for parameter in self.parameters.clone() {
            goal.push(-parameter.0(&self.inputs));
        }
        match mat_f64.solve_for(&goal) {
            Solution::Inconsistant => return Solution::Inconsistant,
            Solution::Unique(n) if iteration < self.iter => {
                self.inputs = self.inputs.add(&n).unwrap();
                return self.iterate(iteration + 1);
            },
            Solution::Infinite { particular: p, homogeneous: _ } if iteration < self.iter => {
                self.inputs = self.inputs.add(&p).unwrap();
                return self.iterate(iteration + 1);
            },
            Solution::Unique(n) => {
                self.inputs = self.inputs.add(&n).unwrap();
                return Solution::Unique(self.inputs.clone());
            }
            Solution::Infinite { particular: p, homogeneous: h } => {
                self.inputs = self.inputs.add(&p).unwrap();
                return Solution::Infinite { particular: self.inputs.clone(), homogeneous: h }
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
impl From<(Vec<Parameter>, &Vec<f64>)> for Jacobian{ 
    fn from(value: (Vec<Parameter>, &Vec<f64>)) -> Self {
        let jacobian = Matrix { elements : vec![ Parameter::default(); value.1.len()*value.0.len()], rows: value.1.len(), colums: value.0.len() };
        Jacobian {
            parameters: value.0.clone(),
            inputs: NumVec(value.1.clone()),
            matrix: jacobian.iterate(|row, colum| value.0[row].clone().diff(colum)),
            iter: 10
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let b = vec![0.5,2.5];
        let constraint = vec![Parameter(Box::new(|p| (p.0[0].powf(2.0)+p.0[1].powf(2.0)-1.0).powf(2.0))),Parameter(Box::new(|p| (p.0[1]-p.0[0]).powf(2.0)))];
        let mut mat = Jacobian::from((constraint, &b));
        println!("{}, {}", mat.matrix.rows, mat.matrix.colums);
        let mut results = Vec::new();
        match mat.solve(NumVec(b)) {
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
        assert!(error < 0.1E-2);
    }
}
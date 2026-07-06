//! # Matrix
//! Module for matrix and vector operations. 


//having a dedicated matrix structure rather than just Vec<Vec<f64>> makes it easier to prevent errors when declaring matrices
//it also allows for easy implementation of matrix operations
// NOTE: the rows and colums attributes might be unnesescary, but at the same time I am not sure
//TODO
//  - add, in a different file, the ability to round to a certain number of digits
//  - fix matrix so it is implemented for more than just parameter, ideally parameter and f64, it might be the case that I have to revert all of this code
//  - bc its too slow which would suck ass and balls
/* */
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use crate::vec::{arg_max, dot_prod};
use crate::Value;
pub enum EchelonForm {
    None,
    Row,
    RowReduced,
}

/// A 2D structure for operating on sets of numbers
/// # Examples
/// ```
/// //this example does not work currently bc I am extremely lazy and cant be fucked to implement a extremely simple function
/// let mut A = Matrix {elements: vec![1,1,1,-1], rows: 2, colums: 2 };
/// let B = Matrix { elements: vec![2, 0], rows: 2, colums: 1};
/// assert_eq!(A.solve_for(B), Matrix {vec![1,1], rows:2, colums: 1});
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T: From<bool> + 
Add<Output = T> + AddAssign + 
Sub<Output = T> + SubAssign +
Mul<Output = T> + MulAssign +
Div<Output = T> + DivAssign + Neg<Output = T> + Clone + Value + PartialOrd> {
    pub elements: Vec<T>, //this shouldnt be public but for now idk how to fix ts since assert_eq needs it to be
    pub rows: usize,
    pub colums: usize,
}
impl <T: From<bool> + 
Add<Output = T> + AddAssign + 
Sub<Output = T> + SubAssign +
Mul<Output = T> + MulAssign +
Div<Output = T> + DivAssign + Neg<Output = T> + Clone + Value + PartialOrd> Matrix<T> {
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
    /// let A = Matrix {elements: vec![1.0, 2.0, 3.0, 4.0], rows: 2, colums: 2};
    /// assert_eq!(A.row(0), vec![1.0, 2.0]); 
    /// ```
    //TODO: return result instead of vec(?)
    pub fn row(&self, row: usize) -> Vec<T> {
        let mut r = Vec::new();
        for colum in 0..self.colums {
            r.push(self.element(row,colum));
        }
        r
    }

    /// Returns any colum of a matrix as a vector
    /// # Examples
    /// ```
    /// use solver::matirx::Matrix;
    /// let A = Matrix { elements: vec![1.0,2.0,3.0,4.0], rows: 2, colums: 2};
    /// assert_eq!(A.colum(1), vec![2.0,4.0]);
    /// ```
    //TODO: return result instead of vec(?)
    pub fn colum(&self, colum: usize) -> Vec<T> {
        let mut r = Vec::new();
        for row in 0..self.rows {
            r.push(self.element(row,colum));
        }
        r
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
    /// assert_eq!(A.elments, vec![9,2,1]);
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
        for element in 0..items.len() {
            match items[element].clone().value() {
                0.0 => {},
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
            self.swap_row(row, arg_max(&self.colum(row).iter().map(|x| (*x).clone()*(*x).clone()).collect::<Vec<_>>()[row..])+row); //when I swap back to &vec<T>: look into clone_into()
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
                    pivot_value = self.row(row)[pivot].clone();
                    self.scale_row(row, pivot_value.clone()/(pivot_value.clone()*pivot_value.clone())); //this is the closest I can get rn to writing 1/pivot_value since generics are hard and annoying
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
    pub fn add_matrix(&self, matrix: &Matrix<T>) -> Option<Matrix<T>> { 
        if self.rows != matrix.rows || self.colums != matrix.colums {
            return None
        }
        Some(
        self.iterate(|row, colum| self.element(row, colum).clone() + matrix.element(row, colum).clone())
        )
    }

    /// multiplies matrices
    pub fn mult(&self, matrix: &Matrix<T>) -> Option<Matrix<T>> { 
        if self.rows != matrix.rows || self.colums != matrix.colums {
            return None
        }
        let result = Matrix {elements: vec![self.element(0,0).clone(); self.rows * matrix.colums], rows: self.rows, colums: matrix.colums};

        Some(
        result.iterate(|row, colum| dot_prod(&self.row(row), &matrix.colum(colum)).unwrap()
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
    pub fn inverse(&self) -> Option<Matrix<T>> {
        if self.rows != self.colums { return None }
        let mut temp = self.clone();
        let mut inverse = T::imat(self.rows);

        for row in 0..(self.rows-1) {
            //the mapping here is a way to get around the fact that abs is not a generic trait
            //we cannot use arg_min because of the edge case of rows that already have a 0 element
            let row_2 = arg_max(&temp.colum(row).iter().map(|x| (*x).clone()*(*x).clone()).collect::<Vec<_>>()[row..])+row;
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
                    pivot_value = temp.row(row)[pivot].clone();
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
    pub fn determinant(&self) -> Matrix<T> {self.clone()}
    /// not implemented yet due to general lazieness
    pub fn adjoint(&self) -> Matrix<T> {self.clone()}
    
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
Add<Output = Self> + AddAssign + 
Sub<Output = Self> + SubAssign +
Mul<Output = Self> + MulAssign +
Div<Output = Self> + DivAssign + 
Neg<Output = Self> + Clone + Value + PartialOrd {
    fn imat(size: usize) -> Matrix<Self>;
}


impl<T: 
From<bool> + 
Add<Output = Self> + AddAssign + 
Sub<Output = Self> + SubAssign +
Mul<Output = Self> + MulAssign +
Div<Output = Self> + DivAssign + 
Neg<Output = Self> + Clone + Value + PartialOrd> IdentityMatrix for T {
    fn imat(size: usize) -> Matrix<Self>{ 
        let mut identity = Matrix { elements: vec![Self::from(false); size*size], rows: size, colums: size};
        for element in 0..size {
            identity.set(element, element, Self::from(true));
        }
        identity
    }
}

pub struct Jacobian;
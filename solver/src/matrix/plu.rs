use crate::{NumBounds, matrix::Matrix, matrix::IdentityMatrix};
use crate::vec::NumVec;
use crate::matrix::Solution::{self, Unique};

/// Is a structure for the Permutation, Lower, Upper decomposition for a matrix,
/// it is particuarly useful for solving square systems of matrices, calculating determinants, and finding inverses
pub struct PLU<T: NumBounds<T>>{
    permutation: Matrix<T>,
    lower: Matrix<T>,
    upper: Matrix<T>
}
impl <T: NumBounds<T>> PLU<T> {
    pub fn permutation(&self) -> Matrix<T> {
        self.permutation.clone()
    }
    pub fn lower(&self) -> Matrix<T> {
        self.lower.clone()
    }
    pub fn upper(&self) -> Matrix<T> {
        self.upper.clone()
    }
    fn substitute(&self, forward: bool, matrix: &Matrix<T>, mut other: NumVec<T>) -> Solution<T> {
        let mut mat = matrix.clone();
        let rows = mat.rows;
        if other.len() != mat.rows {
            panic!("issues wee woo wee woo");
        }
        for row in 0..rows {
            if let None = mat.find_pivot(row) {
                continue;
            }
            let scale = mat.element(row,row);
            mat.scale_row(row, T::from(true)/scale.clone());
            other.0[row] = other.0[row].clone()/scale.clone();
            mat.set(row,row,T::from(true));
        }
        for row in (0..(rows-1)).map(|x| {
                if forward {
                    x
                } else {
                    rows-(x+1)
                }
            }){
                let scale;
                if forward {
                    scale = mat.element(row+1,row);
                    mat.add_row(row+1, row, -scale.clone());
                    other.0[row+1] = other.0[row+1].clone() - scale*other.0[row].clone();
                } else {
                    scale = mat.element(row-1, row);
                    mat.add_row(row-1, row, -scale.clone());
                    other.0[row-1] = other.0[row-1].clone() - scale*other.0[row].clone();
                }
         
        }
        let mut homogeneous = Vec::new();
        for row in 0..rows {
            match mat.find_pivot(row) {
                Some(_) => {},
                None => {
                    if other.0[row].clone().value() <= 0.1E-10 {
                        homogeneous.push(mat.row(row).unwrap());
                    } else {
                        return Solution::Inconsistant
                    }
                }
            }
        }
        if homogeneous.len() != 0 {
            return Solution::Infinite { particular: other, homogeneous: homogeneous }
        } else {
            return Solution::Unique(other)
        }
    }
    pub fn solve_for(&self, other: NumVec<T>) ->  Solution<T>{
        let l = other.len();
        let b = Matrix { elements: other.0, rows: l, colums: 1};
        let s = NumVec(self.permutation.mult(&b).unwrap().elements);
        let y = self.substitute(true, &self.lower, s);
        let x;
        let mut homogeneous = Vec::new();
        match y {
            Solution::Inconsistant => return Solution::Inconsistant,
            Solution::Unique(z) => x=z,
            Solution::Infinite { particular: p, homogeneous: h } => {
                x=p;
                for a in h{
                    homogeneous.push(a);
                }
            }
        }
        let sol = self.substitute(false, &self.upper, x);
        match sol {
            Solution::Inconsistant => Solution::Inconsistant,
            Solution::Unique(z) => Solution::Unique(z),
            Solution::Infinite { particular: p, homogeneous: h } => {
                for a in h{
                    homogeneous.push(a);
                }
                Solution::Infinite { particular: p, homogeneous: homogeneous}
            }
        }
    }
    pub fn inverse(&self) -> Option<Matrix<T>> {
        let mut inverse = Matrix { elements: Vec::new(), rows: self.lower.rows, colums: 0};
        for col in 0..self.lower.colums {
            let mut sol = vec![T::from(false); self.lower.rows];
            sol[col] = T::from(true);
            match self.solve_for(NumVec(sol)) {
                Unique(x) => inverse.append_colum(&x),
                _ => return None
            }
        }
        return Some(inverse);
    }
}
impl <T: NumBounds<T>> From<&Matrix<T>> for PLU<T> {
    fn from(value: &Matrix<T>) -> Self {
        let mut permutation = T::imat(value.rows);
        let mut lower = T::imat(value.rows);
        let mut upper = value.clone();
        
        for row in 0..(upper.rows-1) {
            //the mapping here is a way to get around the fact that abs is not a generic trait
            //we cannot use arg_min because of the edge case of rows that already have a 0 element

            let row_swap = NumVec(upper.colum(row).unwrap().0.iter().map(|x| (*x).clone()*(*x).clone())
                .collect::<Vec<T>>()[row..].to_vec()).arg_max() + row;

            upper.swap_row(row, row_swap); //fix this mess please
            permutation.swap_row(row, row_swap);
            let pivot = upper.element(row, row);
            if pivot.clone().value().abs() > 0.1E-16 {
                for row_2 in (row+1)..upper.rows {
                    let scale = upper.element(row_2,row)/pivot.clone();
                    upper.add_row(row_2, row, -scale.clone());
                    upper.set(row_2,row, T::from(false)); //way to reduce error

                    lower.set(row_2,row,scale);
                }
            }
            if row < upper.rows-2 && row_swap < upper.rows-2{
                let temp = lower.element(row+1,row);
                let other = lower.element(row_swap+1, row);
                lower.set(row+1,row,other);
                lower.set(row_swap+1, row, temp);   
            }
        }
        
        PLU { permutation: permutation, lower: lower, upper: upper }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from() {
        let mat = Matrix{
            elements: vec![0.0,5.0,7.3333333333333333,4.0,2.0,1.0,2.0,7.0,9.0], 
            rows: 3, 
            colums: 3
        };
        let a = PLU::from(&mat);
        let permutations = Matrix{elements: vec![0.0,1.0,0.0,0.0,0.0,1.0,1.0,0.0,0.0], rows:3, colums:3};
        let lower = Matrix{elements: vec![1.0,0.0,0.0,0.5,1.0,0.0,0.0,0.8333333333333334,1.0], rows:3, colums:3};
        let upper = Matrix{elements: vec![4.0,2.0,1.0,0.0,6.0,8.5,0.0,0.0,0.2499999999999991], rows:3, colums:3};
        
        assert_eq!(a.permutation(),permutations);
        assert_eq!(a.lower(), lower);
        assert_eq!(a.upper(), upper);
    }
    #[test]
    fn solve_for_unique(){
        let mat = Matrix{
            elements: vec![6.0,3.0,4.0,3.0], 
            rows:2, 
            colums:2
        };
        let a = PLU::from(&mat);
        let s = NumVec(vec![9.0,7.0]);
        assert_eq!(a.solve_for(s),Solution::Unique(NumVec(vec![1.0,1.0])));
    }
    #[test]
    fn inverse_exists(){
        let mat = Matrix {
            elements: vec![-1.0,1.5,1.0,-1.0],
            rows: 2,
            colums: 2
        };
        let a = PLU::from(&mat);
        assert_eq!(a.inverse(),Some(Matrix { elements: vec![2.0,3.0,2.0,2.0], rows: 2, colums: 2 }));
    }
}
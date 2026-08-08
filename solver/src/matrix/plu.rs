use crate::{NumBounds, matrix::Matrix, matrix::IdentityMatrix};
use crate::vec::NumVec;

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
            if pivot.clone().value() > 0.1E-16 {
                for row_2 in (row+1)..upper.rows {
                    let scale = upper.element(row_2,row)/pivot.clone();
                    upper.add_row(row_2, row, -scale.clone());
                    upper.set(row_2,row, T::from(false)); //way to reduce error

                    lower.set(row_2,row,scale);
                }
            }
            if row < upper.rows-2 {
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
}
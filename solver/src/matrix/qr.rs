use crate::{NumBounds, matrix::Matrix};
pub struct QR<T: NumBounds<T>> {
    orthonormal: Matrix<T>,
    upper: Matrix<T>
}


impl <T: NumBounds<T>> QR<T> {
    pub fn orthonormal(&self) -> Matrix<T> {
        self.orthonormal.clone()
    } 
    pub fn upper(&self) -> Matrix<T> {
        self.upper.clone()
    }
}

impl <T: NumBounds<T>> From<&Matrix<T>> for QR<T> {
    fn from(value: &Matrix<T>) -> Self {
        
        let mut orthonormal_vec = Vec::new();
        for colum in 0..value.colums {
            orthonormal_vec.push(value.colum(colum).unwrap());
        }

        //gram schmidtt process
        for colum in 1..orthonormal_vec.len(){
            let mut temp = orthonormal_vec[colum].clone();
            for colum_2 in 0..colum {
                temp = temp.add(&(orthonormal_vec[colum_2]).proj(&orthonormal_vec[colum]).unwrap().scale(-T::from(true)))
                    .unwrap();
            }
            orthonormal_vec[colum]=temp;
        }
        for i in 0..orthonormal_vec.len() {
            orthonormal_vec[i] = orthonormal_vec[i].scale(T::from(true)/orthonormal_vec[i].magnitude());
        }
        orthonormal_vec.reverse();
        let mut orthonormal = Matrix {elements: orthonormal_vec.pop().unwrap().0, rows:value.rows, colums: 1};
        while let Some(colum) = orthonormal_vec.pop(){
            orthonormal.append_colum(&colum);
        }

        let upper = orthonormal.transpose().mult(&value).unwrap();

        Self {
            orthonormal: orthonormal,
            upper: upper
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //this is another test where it works, but it fails due to fp error
    #[test]
    fn from() {
        let mat = Matrix{
            elements: vec![12.0,-51.0,4.0,6.0,167.0,-68.0,-4.0,24.0,-41.0], 
            rows: 3, 
            colums: 3
        };
        let a = QR::from(&mat);
        let orthonormal = Matrix{elements: vec![0.857142857,-0.394285714,-0.331428571,
                                                            0.428571429,0.902857143,0.0342857143,
                                                            -0.285714286,0.171428571,-0.942857143], rows:3, colums:3};
        let upper = Matrix{elements: vec![14.0,21.0,-14.0,0.0,175.0,-70.0,0.0,0.0,35.0], rows:3, colums:3};
        assert_eq!(a.orthonormal(), orthonormal);
        assert_eq!(a.upper(), upper);
    }
}
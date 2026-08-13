use crate::NumBounds;
use std::{ops::{Add, Div, Mul, Neg, Sub}, process::Output};
pub struct Complex<T: NumBounds<T>> {
    pub r: T,
    pub i: T
}

impl <T: NumBounds<T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}
impl <T: NumBounds<T>> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl <T: NumBounds<T>> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.clone()*rhs.r.clone() - self.i.clone()*rhs.i.clone(),
            i: self.r*rhs.i+self.i*rhs.r
        }
    }
}

impl <T: NumBounds<T>> Div for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let b = rhs.r.clone()*rhs.r.clone()+rhs.i.clone()*rhs.i.clone();
        Self {
            r: (self.r.clone()*rhs.r.clone()+self.i.clone()*rhs.i.clone())/b.clone(),
            i: (self.i*rhs.r-self.r*rhs.i)/b
        }
    }
}

impl <T: NumBounds<T>> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i
        }
    }
}

pub struct Fraction<T: NumBounds<T>> {
    pub num: T,
    pub den: T
}

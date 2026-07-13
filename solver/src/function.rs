pub trait Sin {
    fn sin(self) -> Self;
}
pub trait Cos {
    fn cos(self) -> Self;
}
pub trait Tan {
    fn tan(self) -> Self;
}

pub trait ArcSin {
    fn arc_sin(self) -> Self;
}

pub trait ArcCos {
    fn arc_cos(self) -> Self;
}

pub trait ArcTan {
    fn arc_tan(self) -> Self;
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

pub trait Pow {
    fn pow(self, other: Self) -> Self; 
}
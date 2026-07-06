use crate::parse::{interpret, simplify};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Add, Sub, Mul, Div, Neg};

/*TODO: traits
<T: From<bool> + 
 + AddAssign + 
 + SubAssign +
 + MulAssign +
 + DivAssign + Neg<Output = T> + Value + PartialOrd>
*/

//TODO: add parameterSet

#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub expression: Vec<String>,
    pub value: f64,
}
impl Parameter {
    pub fn update_value(&mut self, parameters: &Vec<Rc<RefCell<Parameter>>>) { 
        self.value = interpret(&self.expression, parameters,1)
    }

    pub fn simplify_expression(&mut self) {
        self.expression = simplify(&self.expression);
    }
}

impl Into<f64> for Parameter{
    fn into(self) -> f64 {
        self.value
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter { expression: vec![String::from("0")], value: 0.0 }
    }
}
impl Add for Parameter {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { //will probably change to pointers but thats wierd
        let mut expression = self.expression.clone(); //unnessecary?
        expression.extend(rhs.expression);
        expression.push(String::from("+"));
        Self {
            expression: expression,
            value: self.value + rhs.value,
        }
    }
}
impl Sub for Parameter {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { //will probably change to pointers but thats wierd
        let mut expression = self.expression.clone(); //unnessecary?
        expression.extend(rhs.expression);
        expression.push(String::from("-"));
        Self {
            expression: expression,
            value: self.value - rhs.value,
        }
    }
}
impl Mul for Parameter {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { //will probably change to pointers but thats wierd
        let mut expression = self.expression.clone(); //unnessecary?
        expression.extend(rhs.expression);
        expression.push(String::from("*"));
        Self {
            expression: expression,
            value: self.value * rhs.value,
        }
    }
}
impl Div for Parameter {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { //will probably change to pointers but thats wierd
        let mut expression = self.expression.clone(); //unnessecary?
        expression.extend(rhs.expression);
        expression.push(String::from("/"));
        Self {
            expression: expression,
            value: self.value / rhs.value,
        }
    }
}

impl Neg for Parameter {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut expression = self.expression.clone();
        expression.push(String::from("-1"));
        expression.push(String::from("*"));
        Self {
            expression: expression,
            value: self.value * -1.0
        }
    }
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
    fn lt(&self, other: &Self) -> bool {
        if self.value < other.value { return true; }
        false
    }
    fn le(&self, other: &Self) -> bool {
        if self.value <= other.value { return true; }
        false
    }
    fn gt(&self, other: &Self) -> bool {
        if self.value > other.value { return true; }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        if self.value >= other.value { return true; }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_add_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 + parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("+")], value: 3.0 })
    }

    #[test]
    fn parameter_sub_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 - parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("-")], value: -1.0 })
    }

    #[test]
    fn parameter_mul_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 * parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("*")], value: 2.0 })
    }

    #[test]
    fn parameter_div_test() {
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0 };
        let parameter_2 = Parameter{expression: vec![String::from("2")], value: 2.0 };
        assert_eq!(parameter_1 / parameter_2, Parameter{expression: vec![String::from("1"), String::from("2"), String::from("/")], value: 0.5 })
    }

}
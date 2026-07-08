/*
TODO:
    - improve tokenizer error handling
    - improve test cases
    - add enum for operators("(", ")", operator, and function)?
    - clean code even more(FAHHHH)
    - other performance shi tbh
    - expression simplification(?)
    - find a way to set operator as a static variable or something adjactent to a static variable
*/

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::LazyLock};
use crate::{parameter::Parameter, parse::Token::Operator};

static MNEMONICS: [(&str, &str); 4] = [ ("sin", "s"),
                                        ("cos", "c"),
                                        ("tan", "t"),
                                        ("pi", "3.14159265359879")];
    static OPERATORS: &str = "+-/c*t^s";
    static PRECIDENCE: LazyLock<HashMap<char, i32>> = LazyLock::new(|| HashMap::from([
        ('+', 1),
        ('-', 1),
        ('/', 2),
        ('*', 2),
        ('^', -3),
        ('s', 4),
        ('c', 4),
        ('t', 4),
    ]));
#[derive(Clone)]
enum Token {
    Beginning,
    Number(f64),
    Parameter(&str),
    Operator{op: &str, precidence: u8},
    Parantheses(char),
}
impl Token {
    const MUL: Self::Operator = Self::Operator { op: "*", precidence: 2 };
    fn next(&self, token: &Token) -> Result<Self, &'static str> {
        match self {
            Self::Beginning => Ok(token.clone()),
            Self::Number(_) => {
                match token {
                    Self::Operator { op: _ , precidence: _ } => Ok(token.clone()),
                    Self::Parameter(_) | Self::Parantheses('(') => Ok(MUL),
                    _ => Err("Undefined behavior")
                }
            },
            Self::Number(_) => {
                match token {
                    Self::Parameter(_) | Self::Parantheses('(') => Ok(MUL),
                    Operator{op: _, precidence: prec} if prec == 4 => Ok(MUL), //this is saying that if it is something like 3.0sin then expand it to 3.0 * sin(x)
                    Operator{op: _, precidence: _} | Self::Parantheses(')') => Ok(token.clone()),
                }
            }
            Self::Parameter(_) => {
                match token {
                    Self::Number(_) | Self::Parameter(_) | Self::Parantheses('(') => Ok(MUL),
                    Self::Operator { op: _, precidence: _ } | Self::Parantheses(')') => Ok(token.clone()),
                    Self::Beginning => unreachable!()
                }
            },
            Self::Operator{op: _, precidence: 4} => {
                match token {
                    Self::Parantheses('(') => Ok(token.clone()),
                    _ => Err("Invalid")
                }
            },
            Self::Parantheses('(') => {
                match token {
                    self::Operator { op: _, precidence: p } if p != 4 => Err("Invalid"),
                    _ => Ok(token.clone())
                }
            }
            Self::Parantheses(')') => {
                match token {
                    Self::Parameter(_) | Self::Number(_) => Ok(MUL),
                    _ => Ok(token.clone())
                }
            }
        }
    }
}

pub fn tokenize(raw_expression: &str) -> 

//shunting yard algorithm
//autism reference
//needs to be refactored again because ts buns
pub fn parse(mut tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();

    let mut temp: char = ' ';
    tokens.push(String::from(" "));
    for token in tokens {
        loop {
            let stack_length = operator_stack.len();
            if stack_length == 0 {
                if temp != ' ' {
                    operator_stack.push(temp);
                    temp = ' ';
                }
                break;
            }

            let stack_precidence = PRECIDENCE.get(&operator_stack[stack_length - 1]).copied().unwrap_or(0);
            let temp_precidence = PRECIDENCE.get(&temp).copied().unwrap_or(i32::MAX);
            match temp {
                ')' if operator_stack[stack_length - 1] == '(' => {
                    operator_stack.pop();
                    temp = ' ';
                    break;
                },

                ')' => output.push(String::from(operator_stack.pop().unwrap())),
                _operator if stack_precidence.abs() > temp_precidence.abs() => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                },

                _operator if stack_precidence.abs() == temp_precidence => {
                    output.push(String::from(operator_stack.pop().unwrap()))
                },

                ' ' => break,

                _ => {
                    operator_stack.push(temp);
                    temp = ' ';
                    break;
                },
            }
        }

        match token.chars().next().unwrap_or('0') {
            operator if OPERATORS.contains(operator) => temp = operator,
            parenthenses @ ('(' | ')') => temp = parenthenses,
            _ => output.push(token),
        }
    }

    output.pop();

    operator_stack.reverse();
    for operator in operator_stack {
        output.push(String::from(operator));
    }

    output
}


//TODO: rewrite for parameterSet
/// Takes in a parsed RPN expression an returns a Parameter
pub fn interpret(expression: &Vec<String>) -> Parameter {
    
}

#[cfg(test)]
mod tests {
use crate::parameter::Parameter;

use super::*;
    struct TestExpressions {
        raw_expression_whitespace: String,
        raw_expression_parenthenses: String,
        raw_expression_variable: String,
        raw_expression_prefix: String,

        tokens_whitespace: Vec<String>,
        tokens_parenthenses: Vec<String>,
        tokens_variable: Vec<String>,
        tokens_prefix: Vec<String>,

        expression_whitespace: Vec<String>,
        expression_parenthenses: Vec<String>,
        expression_variable: Vec<String>,
        expression_prefix: Vec<String>,

        result_whitespace: f64,
        result_parenthenses: f64,
        result_variable: f64,
        result_prefix: f64,
 
        
        parameters: Vec<Rc<RefCell<parameter::Parameter>>>, //the worlds ugliest concrete type
    }
    impl Default for TestExpressions {
        fn default() -> TestExpressions {
            TestExpressions {
                raw_expression_whitespace: String::from(" 1.2/7 - 6 + 3 . 0 *   10   "),
                raw_expression_parenthenses: String::from("(1-3)/(2+1)"),
                raw_expression_variable: String::from("2p3+p1/2+3.1(2+1)"),
                raw_expression_prefix: String::from("sin(3.14159265358979) + 1 - p0"),

                tokens_whitespace: vec![
                    String::from("1.2"),
                    String::from("/"),
                    String::from("7"),
                    String::from("-"),
                    String::from("6"),
                    String::from("+"),
                    String::from("3.0"),
                    String::from("*"),
                    String::from("10"),
                ],
                tokens_parenthenses: vec![
                    String::from("("),
                    String::from("1"),
                    String::from("-"),
                    String::from("3"),
                    String::from(")"),
                    String::from("/"),
                    String::from("("),
                    String::from("2"),
                    String::from("+"),
                    String::from("1"),
                    String::from(")"),
                ],
                tokens_variable: vec![
                    String::from("2"),
                    String::from("*"),
                    String::from("p3"),
                    String::from("+"),
                    String::from("p1"),
                    String::from("/"),
                    String::from("2"),
                    String::from("+"),
                    String::from("3.1"),
                    String::from("*"),
                    String::from("("),
                    String::from("2"),
                    String::from("+"),
                    String::from("1"),
                    String::from(")"),
                ],
                tokens_prefix: vec![
                    String::from("s"),
                    String::from("("),
                    String::from("3.14159265358979"),
                    String::from(")"),
                    String::from("+"),
                    String::from("1"),
                    String::from("-"),
                    String::from("p0"),
                ],

                expression_whitespace: vec![
                    String::from("1.2"),
                    String::from("7"),
                    String::from("/"),
                    String::from("6"),
                    String::from("-"),
                    String::from("3.0"),
                    String::from("10"),
                    String::from("*"),
                    String::from("+"),
                ],
                expression_parenthenses: vec![
                    String::from("1"),
                    String::from("3"),
                    String::from("-"),
                    String::from("2"),
                    String::from("1"),
                    String::from("+"),
                    String::from("/"),
                ],
                expression_variable: vec![
                    String::from("2"),
                    String::from("p3"),
                    String::from("*"),
                    String::from("p1"),
                    String::from("2"),
                    String::from("/"),
                    String::from("+"),
                    String::from("3.1"),
                    String::from("2"),
                    String::from("1"),
                    String::from("+"),
                    String::from("*"),
                    String::from("+"),
                ],
                expression_prefix: vec![
                    String::from("3.14159265358979"),
                    String::from("s"),
                    String::from("1"),
                    String::from("+"),
                    String::from("p0"),
                    String::from("-"),
                ],

                result_whitespace: 24.17142857142857,
                result_parenthenses: -0.6666666666666666,
                result_variable: 25.3,
                result_prefix: -6.9999999999999964, //aproaches -7 with more digits of pi

                
                parameters: vec![
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("8")],
                        value: 8.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("0")],
                        value: 0.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("0")],
                        value: 0.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("8")],
                        value: 8.0,
                    })),
                    Rc::new(RefCell::new(Parameter {
                        expression: vec![String::from("5")],
                        value: 5.0,
                    })),
                ],
            }
        }
    }

    #[test]
    fn tokenize_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_whitespace
            ),
            test_expressions.tokens_whitespace
        )
    }

    #[test]
    fn tokenize_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            tokenize(
                &test_expressions.raw_expression_parenthenses
            ),
            test_expressions.tokens_parenthenses
        )
    }

    #[test]
    fn tokenize_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(tokenize(&test_expressions.raw_expression_variable),test_expressions.tokens_variable)
    }
    #[test]
    fn tokenize_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(tokenize(&test_expressions.raw_expression_prefix),test_expressions.tokens_prefix)
    }

    #[test]
    fn parse_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(parse(test_expressions.tokens_whitespace.clone()),test_expressions.expression_whitespace)
    }

    #[test]
    fn parse_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(parse(test_expressions.tokens_parenthenses.clone()),test_expressions.expression_parenthenses)
    }

    #[test]
    fn parse_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(parse(test_expressions.tokens_variable.clone()),test_expressions.expression_variable)
    }

    #[test]
    fn parse_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(parse(test_expressions.tokens_prefix.clone()),test_expressions.expression_prefix)
    }

    #[test]
    fn interpret_whitespace_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_whitespace,                
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_whitespace
        )
    }

    #[test]
    fn interpret_parenthenses_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_parenthenses,                
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_parenthenses
        )
    }

    #[test]
    fn interpret_variable_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_variable,               
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_variable
        )
    }

    #[test]
    fn interpret_prefix_test() {
        let test_expressions: TestExpressions = TestExpressions::default();
        assert_eq!(
            interpret(
                &test_expressions.expression_prefix,
                &test_expressions.parameters,
                0
            ),
            test_expressions.result_prefix
        )
    }

    #[test]
    fn simplify_test() {
        let test_expressions = TestExpressions::default();
        let expected_result = vec![
                    String::from("2"),
                    String::from("p3"),
                    String::from("*"),
                    String::from("p1"),
                    String::from("2"),
                    String::from("/"),
                    String::from("+"),
                    String::from("9.3"),
                    String::from("+"),
                ];
        assert_eq!(expected_result, simplify(&test_expressions.expression_variable))
    }
}

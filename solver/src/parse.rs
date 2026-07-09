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

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::LazyLock, thread::current};
use crate::{parameter::Parameter, };

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
enum Parenthenses {
    Open,
    Close
}

#[derive(Clone)]
enum Token {
    Beginning,
    Number(&'static str),
    Parameter(&'static str),
    Operator{op: &'static str, precidence: i8},
    Parantheses(Parenthenses),
}

pub fn tokenize(raw_expression: &str) -> Option<Vec<Token>> {
    let mut chars = raw_expression.chars();
    let mut current_token = Token::Beginning;
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(char) = chars.next() {
        match current_token {
            Token::Beginning => {
                match char {
                    n @ '0'..='9' => current_token = Token::Number(&n.to_string()),
                    'p' => current_token = Token::Parameter("p"),
                    '(' => current_token = Token::Parantheses(Parenthenses::Open),
                    op @ 'a'..='z' => current_token = Token::Operator { op: &op.to_string(), precidence: 4 },
                    _ => return None
                }
            },
            Token::Number(num) => {
                match char {
                    n @ ('0'..='9' | '.') => { //TODO: clean ts john on up
                        let mut a = num.to_string();
                        a.push(n); 
                        current_token = Token::Number(&a);
                    },
                    'p' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parameter(&"p");
                    },
                    op @ ('+' | '-') => {
                        tokens.push(current_token);
                        current_token = Token::Operator {op: &op.to_string(), precidence: 1};
                    },
                    op @ ('*' | '/' ) => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: &op.to_string(), precidence: 2 };
                    },
                     '^'  => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: "^", precidence: -3 };
                    },
                    '(' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parantheses(Parenthenses::Open);
                    }
                    ')'  => {
                        tokens.push(current_token);
                        current_token = Token::Parantheses(Parenthenses::Close);
                    },
                    op @ 'a'..='z' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Operator { op: &op.to_string(), precidence: 4 }
                    },
                    _ => return None
                }
            },
            Token::Parameter(param) => {
                match char {
                    n @ '0'..='9' => {
                        let mut a = param.to_string();
                        a.push(n); 
                        current_token = Token::Parameter(&a);
                    },
                    'p' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parameter("p");
                    },
                    op @ ('+' | '-') => {
                        tokens.push(current_token);
                        current_token = Token::Operator {op: &op.to_string(), precidence: 1};
                    },
                    op @ ('*' | '/' ) => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: &op.to_string(), precidence: 2 };
                    },
                    '^'  => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: "^", precidence: -3 };
                    },
                    '(' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parantheses(Parenthenses::Open);
                    }
                    ')'  => {
                        tokens.push(current_token);
                        current_token = Token::Parantheses(Parenthenses::Close);
                    },
                    op @ 'a'..='z' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Operator { op: &op.to_string(), precidence: 4 }
                    },
                    _ => return None
                }
            },
            Token::Operator { op: op, precidence: prec } => {
                match char {
                    c @ 'a'..='z' if prec == 4 => {
                        let mut a = op.to_string();
                        a.push(c);
                        current_token = Token::Operator { op: &a, precidence: 4 };
                    },
                    c @ 'a'..='z' if prec != 4 => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: &c.to_string(), precidence: 4};
                    },
                    num @ '0'..='9' if prec != 4 => {
                        tokens.push(current_token);
                        current_token = Token::Number(&num.to_string());
                    },
                    num @ '0'..='9' if prec == 4 => {
                        match op {
                            "pi" => {
                                tokens.push(Token::Number("3.14159265358979"));
                                tokens.push(Token::Operator{op: "*", precidence: 2});
                                current_token = Token::Number(&num.to_string());
                            },
                            "e" => {
                                tokens.push(Token::Number("2.71"));
                                tokens.push(Token::Operator{op: "*", precidence: 2});
                                current_token = Token::Number(&num.to_string());
                            },
                            _ => return None
                        }
                    },
                    'p' if prec != 4 => {
                        tokens.push(current_token);
                        current_token = Token::Parameter("p");
                    },
                    '(' if prec != 4 => {
                        tokens.push(current_token);
                        current_token = Token::Parantheses(Parenthenses::Open);
                    },
                    '(' if prec == 4 => {
                        match op {
                            "sin" | "cos" | "tan" | "arcsin" | "arccos" | "arctan" | "log" | "ln"  => {
                                tokens.push(current_token);
                                current_token = Token::Parantheses(Parenthenses::Open);
                            },
                            "pi" => {
                                tokens.push(Token::Number("3.13159265358979"));
                                tokens.push(Token::Operator{op: "*", precidence: 2});
                                current_token = Token::Parantheses(Parenthenses::Open);
                            },
                            "e" => {
                                tokens.push(Token::Number("2.71"));
                                tokens.push(Token::Operator{op: "*", precidence: 2});
                                current_token = Token::Parantheses(Parenthenses::Open);
                            },
                            _ => return None
                        }
                    },
                    _ => return None
                }
            },
            Token::Parantheses(Parenthenses::Open) => {
                match char {
                    num @ '0'..='9' => {
                        tokens.push(current_token);
                        current_token = Token::Number(&num.to_string());
                    },
                    'p' => {
                        tokens.push(current_token);
                        current_token = Token::Parameter("p");
                    }
                    c @ 'a'..='z' => {
                        tokens.push(current_token);
                        current_token = Token::Operator{op: &c.to_string(), precidence: 4};
                    },
                    '(' => tokens.push(current_token),
                    ')' => {
                        tokens.push(current_token);
                        current_token = Token::Parantheses(Parenthenses::Close);
                    },
                    _ => return None
                }
            },
            Token::Parantheses(Parenthenses::Close) => {
                match char {
                    num@ '0'..='9' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Number(&num.to_string());
                    },
                    'p' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parameter("p");
                    },
                    '(' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Parantheses(Parenthenses::Open);
                    },
                    op @ ('+' | '-') => {
                        tokens.push(current_token);
                        current_token = Token::Operator {op: &op.to_string(), precidence: 1};
                    },
                    op @ ('*' | '/' ) => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: &op.to_string(), precidence: 2 };
                    },
                     '^'  => {
                        tokens.push(current_token);
                        current_token = Token::Operator { op: "^", precidence: -3 };
                    },
                    c @ 'a'..='z' => {
                        tokens.push(current_token);
                        tokens.push(Token::Operator { op: "*", precidence: 2 });
                        current_token = Token::Operator{op: &c.to_string(), precidence: 4};
                    },
                    _ => return None
            }
        }
    }
    }
    Some(tokens)
}

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

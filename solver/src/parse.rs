use crate::parameter::Parameter;
use crate::function::*;

/// Denotes the possible components of any expression a user might input
/// - Beginning: marks the beginning of a expression
/// - Number: any constant value, IE: 12, 17.45, etc
/// - Parameter: any variable, IE: p0, p12, etc
/// - Operator: any non function operators like +, -, /, *, and ^
/// - Function: any function: IE sin, cos, tan, etc
/// - OpenParenthenses: the ( symbol
/// - CloseParentenses: the ) symbol
/// - Unknown: will never show up in a parsed expresion, is used as an intermediate when figuring out the validity of certain strings
#[derive(Clone, PartialEq, Debug)]
pub enum Token<T> {
    Beginning,
    Number(T),
    Parameter(usize),
    Operator(char),
    Function(String),
    OpenParenthenses,
    CloseParenthenses,
    Unknown(T),
}

impl Token<String> {
    pub fn next(&self, token: Self) -> Option<Self> {
        let func = String::from(" sin cos tan arctan arccos arcsin sqrt ");
        match self {
            Self::Beginning | Self::OpenParenthenses | Self::Operator(_) => {
                match token {
                    Self::CloseParenthenses | Self::Operator(_) => None,
                    Self::Unknown(ref str) if &String::from("p") == str => Some(token.clone()),
                    //these can be condensed into a single match using a hashmap 
                    Self::Unknown(str) if String::from("pi") == str => Some(Token::Number(String::from("3.14159265358979"))),
                    Self::Unknown(str) if String::from("e") == str =>  Some(Token::Number(String::from("2.71828182845904"))),
                    Self::Unknown(str) if func.contains(&(" ".to_string() + &str + " ")) => Some(Self::Function(str)), 
                    _ => Some(token.clone())
                }
            },
            Self::Number(_) | Self::Parameter(_) | Self::CloseParenthenses => {
                match token {
                    Self::Operator(_) | Self::CloseParenthenses => Some(token.clone()),
                    _ => Some(Self::Operator('*'))
                }
            },
            Self::Function(_) if token == Self::OpenParenthenses => Some(token.clone()),
            Self::Unknown(str) if *str == String::from("p")  => {
                match token {
                    Self::Number(num) => {
                        match num.parse::<usize>() {
                            Ok(u) => Some(Self::Parameter(u)),
                            _ => None
                        }
                    },
                    _ => None
                }
            }
            Self::Unknown(_) => unreachable!(),
            _ => None
        }
    }
}

/// this function takes in a string literal and returns a Vec<Token<f64>>
/// For instance, the expression "3p0 +  sin(1)" will be parsed as:
/// [Number(3.0), Parameter(0), Operator('+'), Function("sin"), OpenParenthenses, Number(1.0), CloseParenthenses]
/// Since there are so many quirks in how we write math, there is a whole other function dedicated to expanding these tokens,
/// adding in implied multiplications and checking if the sequence of tokens are valid
/// this function just deals with two tasks
/// - take the raw input and match it with tokens
/// - compress those tokens according to a set of rules
/// # Time Complexity
/// I am like 50% sure this is O(n) but I know that it is at most O(n^2)
/// # Examples
/// ```
/// use solver::parse::{tokenize, Token};
/// let a = "0.0+1";
/// assert_eq!(tokenize(&a), vec![Token::Number(0.0), Token::Operator('+'), Token::Number(1.0)].into())
/// ```
pub fn tokenize(expression_raw: &str) -> Option<Vec<Token<f64>>> {
    let mut tokens_raw = Vec::new();
    let mut chars = expression_raw.chars();
    while let Some(char) = chars.next() {
        match char {
            ' ' => {},
            '0'..='9' | '.' => tokens_raw.push(Token::Number(char)),
            'A'..='Z' | 'a'..='z' => tokens_raw.push(Token::Unknown(char.to_ascii_lowercase())),
            '(' => tokens_raw.push(Token::OpenParenthenses),
            ')' => tokens_raw.push(Token::CloseParenthenses),
            '+' | '-' | '*' | '/' | '^' => tokens_raw.push(Token::Operator(char)),
            _ => return None
        }
    }

    let mut tokens_combined: Vec<Token<String>> = Vec::new();
    let mut current_token: Token<String> = Token::Beginning;
    tokens_raw.reverse();

    //TODO: with a little bit of ingenuity, I should be able to get rid of temp all together
    let mut temp = String::new();
    while let Some(token) = tokens_raw.pop() {
        match token {
            //the ideal behavior would be something like Some(tok @ (Token::Number(c) | Token::Unknown(c))) because then I could cut the
            //number of match statements to two
            // actually I could probably achieve this using if statements but I will come back and do that
            Token::Number(c) => {
                match current_token {
                    Token::Beginning | Token::Number(_) => {},
                    _ => {
                        tokens_combined.push(current_token);
                        temp = String::new(); //uncessasarry computation for all cases besides unknown
                    },
                }
                temp.push(c);
                current_token = Token::Number(temp.clone());
            },
            Token::Operator(c) => {
                match current_token {
                    Token::Beginning => {},
                    _ => tokens_combined.push(current_token)
                }
                current_token = Token::Operator(c);
            },
            Token::Unknown(c) => {
                match current_token {
                    Token::Beginning | Token::Unknown(_) => {},
                    _ => {
                        tokens_combined.push(current_token);
                        temp = String::new(); //uncessasarry computation for all cases besides unknown
                    }
                }
                temp.push(c);
                current_token = Token::Unknown(temp.clone());
            },
            Token::OpenParenthenses => {
                tokens_combined.push(current_token);
                current_token = Token::OpenParenthenses;
            },
            Token::CloseParenthenses => {
                tokens_combined.push(current_token);
                current_token = Token::CloseParenthenses;
            },
            _ => unreachable!(),
        }
    }
    tokens_combined.push(current_token);

    let mut tokens_string = Vec::new();
    tokens_combined.reverse();
    while let Some(token_1) = tokens_combined.pop() {
        match tokens_combined.pop() {
            Some(token_2) => {
                if token_1 != Token::Unknown(String::from("p")) { // TODO: clean up this if statement by removing it
                    tokens_string.push(token_1.clone());   
                }
                match token_1.next(token_2.clone()) {
                    Some(Token::Operator('*')) => {
                        tokens_combined.push(token_2);
                        tokens_combined.push(Token::Operator('*'));
                    },
                    Some(tok @ _ ) => {
                        tokens_combined.push(tok);
                    },
                    None => return None
                }
            },
            None => tokens_string.push(token_1)
        }
    }
    
    let mut tokens = Vec::new();
    tokens_string.reverse();
    while let Some(token) = tokens_string.pop() {
        match token {
            Token::Number(num) => tokens.push(Token::Number(num.parse::<f64>().unwrap_or(0.0))),
            Token::OpenParenthenses => tokens.push(Token::OpenParenthenses),
            Token::CloseParenthenses => tokens.push(Token::CloseParenthenses),
            Token::Parameter(n) => tokens.push(Token::Parameter(n)),
            Token::Operator(op) => tokens.push(Token::Operator(op)),
            Token::Function(func) => tokens.push(Token::Function(func)),
            _ => return None    
        }
    }
    Some(tokens)
}

fn precidence(operator: &Token<f64>) -> i8 {
    match operator {
        Token::Operator('+') | Token::Operator('-') => 1,
        Token::Operator('*') | Token::Operator('/') => 2,
        Token::Operator('^') => -3,
        _ => unreachable!()
    }
}

/// Takes in a list of tokens and outputs a Reverse Polish Notation expression
/// # Examples
/// ```
/// use solver::parse::{parse, Token};
/// //1+2*3
/// let expression = vec![Token::Number(1.0), Token::Operator('+'), Token::Number(2.0), Token::Operator('*'), Token::Number(3.0)];
/// let expected = vec![Token::Number(1.0), Token::Number(2.0), Token::Number(3.0), Token::Operator('*'), Token::Operator('+')];
/// assert_eq!(parse(&expression), Some(expected));
/// ```
pub fn parse(expression: &Vec<Token<f64>>) -> Option<Vec<Token<f64>>> {
    let mut tokens = expression.clone();
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();
    tokens.reverse();
    while let Some(token) = tokens.pop() {
        println!("h: {:?}", operator_stack);
        match token {
            Token::Number(_) | Token::Parameter(_) => output.push(token),
            Token::Function(_) | Token::OpenParenthenses => operator_stack.push(token),
            Token::Operator(_) => {
                while let Some(op_2) = operator_stack.pop() {
                    match op_2 {
                        Token::OpenParenthenses => {
                            operator_stack.push(op_2); 
                            break
                        },
                        Token::Operator(_) => {
                            if precidence(&op_2).abs() > precidence(&token).abs() {
                                output.push(op_2);
                            }
                            else if precidence(&op_2) == precidence(&token) && precidence(&op_2) > 0 {
                                output.push(op_2);
                            }
                            else {
                                operator_stack.push(op_2);
                                break
                            }
                        },
                        _ => unreachable!()
                    }
                }
                operator_stack.push(token);
            },
            Token::CloseParenthenses => {
                let mut found = false;
                while let Some(op_2) = operator_stack.pop() {
                    match op_2 {
                        Token::OpenParenthenses => {
                            found = true;
                            break},
                        Token::Operator(_) => output.push(op_2),
                        _ => unreachable!()
                    }
                }
                if !found {
                    return None
                }
            },
            _ => unreachable!()
        }
    }

    while let Some(token) = operator_stack.pop() {
        match token {
            Token::OpenParenthenses => return None,
            Token::Operator(_) | Token::Function(_) => output.push(token),
            _ => unreachable!() 
        }
    }
    Some(output)
}    

/// Takes in a parsed expression and returns a parameter that can actually be used to make computations.
/// # Examples
/// ```
/// use solver::parse::{Token, interpret};
/// // p0 + 1
/// let expression = vec![Token::Parameter(0), Token::Number(1.0), Token::Operator('+')];
/// let params = vec![0.0, 1.0];
/// 
/// assert_eq!(interpret(&expression).unwrap().0(&params), 1.0);
/// ```

pub fn interpret(parsed_expression: &Vec<Token<f64>>) -> Option<Parameter> {
    let mut tokens = parsed_expression.clone();
    tokens.reverse();
    let mut output_queue = Vec::new();
    while let Some(token) = tokens.pop() {
        match token {
            Token::Number(num) => output_queue.push(Parameter(Box::new(move |_p| num))),
            Token::Parameter(index) => output_queue.push(Parameter(Box::new(move |p| p[index]))),
            Token::Operator(op) => {
                let param_1: Parameter;
                let param_2: Parameter;

                if let Some(param) = output_queue.pop() { param_2 = param; } else { return None; }
                if let Some(param) = output_queue.pop() { param_1 = param; } else { return None; }
                
                match op {
                    '+' => output_queue.push(param_1 + param_2),
                    '-' => output_queue.push(param_1 - param_2),
                    '/' => output_queue.push(param_1 / param_2),
                    '*' => output_queue.push(param_1 * param_2),
                    '^' => output_queue.push(param_1.pow(param_2)),
                    _ => return None
                }
            },
            Token::Function(func) => {
                let param_1: Parameter;

                if let Some(param) = output_queue.pop() {param_1 = param; } else { return None; }

                match &func[..] {
                    "sin" => output_queue.push(param_1.sin()),
                    "arcsin" => output_queue.push(param_1.arc_sin()),
                    "cos" => output_queue.push(param_1.cos()),
                    "arccos" => output_queue.push(param_1.arc_cos()),
                    "tan" => output_queue.push(param_1.tan()),
                    "arctan" => output_queue.push(param_1.arc_tan()),
                    _ => return None
                }
            },
            _ => return None
        }
    }
    
    Some(output_queue[0].clone())
}
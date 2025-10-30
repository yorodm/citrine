//! Citrine - A Clojure-like language implemented in Rust
//!
//! This crate provides a lexer and parser for the Citrine language,
//! a Clojure-like Lisp dialect. The parser produces a concrete syntax tree (CST)
//! using the rowan library, which can be used for further processing.

pub mod lexer;
pub mod parser;
pub mod syntax;
pub mod reader;

/// Parses the given input and returns a syntax tree
pub fn parse(input: &str) -> syntax::SyntaxNode {
    let parser = parser::Parser::new(input);
    parser.parse()
}

/// Tokenizes the given input and returns a vector of tokens
pub fn tokenize(input: &str) -> Vec<lexer::Token> {
    let mut lexer = lexer::Lexer::new(input);
    lexer.tokenize()
}

/// Reads the given input and returns a Citrine value
pub fn read_str(input: &str) -> Result<reader::Value, reader::EvalError> {
    let syntax = parse(input);
    reader::read(&syntax)
}

/// Evaluates the given input in the given environment
pub fn eval_str(input: &str, env: &std::rc::Rc<std::cell::RefCell<reader::Environment>>) -> Result<reader::Value, reader::EvalError> {
    let value = read_str(input)?;
    reader::eval(&value, env)
}

/// Creates a new standard environment with built-in functions
pub fn standard_env() -> std::rc::Rc<std::cell::RefCell<reader::Environment>> {
    use std::rc::Rc;
    use std::cell::RefCell;
    use reader::{Value, Function, EvalError};
    
    let env = Rc::new(RefCell::new(reader::Environment::new()));
    
    // Add built-in functions
    
    // Arithmetic operations
    env.borrow_mut().set(
        "+".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            let mut sum = 0.0;
            for arg in args {
                match arg {
                    Value::Number(n) => sum += n,
                    _ => return Err(EvalError::TypeError {
                        expected: "number".to_string(),
                        got: format!("{:?}", arg),
                    }),
                }
            }
            Ok(Value::Number(sum))
        })),
    );
    
    env.borrow_mut().set(
        "-".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.is_empty() {
                return Err(EvalError::ArityMismatch {
                    expected: 1,
                    got: 0,
                });
            }
            
            match &args[0] {
                Value::Number(first) => {
                    if args.len() == 1 {
                        // Unary minus
                        Ok(Value::Number(-first))
                    } else {
                        // Subtraction
                        let mut result = *first;
                        for arg in &args[1..] {
                            match arg {
                                Value::Number(n) => result -= n,
                                _ => return Err(EvalError::TypeError {
                                    expected: "number".to_string(),
                                    got: format!("{:?}", arg),
                                }),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                }
                _ => Err(EvalError::TypeError {
                    expected: "number".to_string(),
                    got: format!("{:?}", args[0]),
                }),
            }
        })),
    );
    
    env.borrow_mut().set(
        "*".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            let mut product = 1.0;
            for arg in args {
                match arg {
                    Value::Number(n) => product *= n,
                    _ => return Err(EvalError::TypeError {
                        expected: "number".to_string(),
                        got: format!("{:?}", arg),
                    }),
                }
            }
            Ok(Value::Number(product))
        })),
    );
    
    env.borrow_mut().set(
        "/".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.is_empty() {
                return Err(EvalError::ArityMismatch {
                    expected: 1,
                    got: 0,
                });
            }
            
            match &args[0] {
                Value::Number(first) => {
                    if args.len() == 1 {
                        // Reciprocal
                        if *first == 0.0 {
                            return Err(EvalError::Other("Division by zero".to_string()));
                        }
                        Ok(Value::Number(1.0 / first))
                    } else {
                        // Division
                        let mut result = *first;
                        for arg in &args[1..] {
                            match arg {
                                Value::Number(n) => {
                                    if *n == 0.0 {
                                        return Err(EvalError::Other("Division by zero".to_string()));
                                    }
                                    result /= n;
                                }
                                _ => return Err(EvalError::TypeError {
                                    expected: "number".to_string(),
                                    got: format!("{:?}", arg),
                                }),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                }
                _ => Err(EvalError::TypeError {
                    expected: "number".to_string(),
                    got: format!("{:?}", args[0]),
                }),
            }
        })),
    );
    
    // Comparison operations
    env.borrow_mut().set(
        "=".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() < 2 {
                return Err(EvalError::ArityMismatch {
                    expected: 2,
                    got: args.len(),
                });
            }
            
            let first = &args[0];
            for arg in &args[1..] {
                if first != arg {
                    return Ok(Value::Boolean(false));
                }
            }
            
            Ok(Value::Boolean(true))
        })),
    );
    
    env.borrow_mut().set(
        "<".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() != 2 {
                return Err(EvalError::ArityMismatch {
                    expected: 2,
                    got: args.len(),
                });
            }
            
            match (&args[0], &args[1]) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
                _ => Err(EvalError::TypeError {
                    expected: "number".to_string(),
                    got: format!("{:?} and {:?}", args[0], args[1]),
                }),
            }
        })),
    );
    
    env.borrow_mut().set(
        ">".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() != 2 {
                return Err(EvalError::ArityMismatch {
                    expected: 2,
                    got: args.len(),
                });
            }
            
            match (&args[0], &args[1]) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
                _ => Err(EvalError::TypeError {
                    expected: "number".to_string(),
                    got: format!("{:?} and {:?}", args[0], args[1]),
                }),
            }
        })),
    );
    
    // Logical operations
    env.borrow_mut().set(
        "not".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() != 1 {
                return Err(EvalError::ArityMismatch {
                    expected: 1,
                    got: args.len(),
                });
            }
            
            match &args[0] {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                Value::Nil => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            }
        })),
    );
    
    // List operations
    env.borrow_mut().set(
        "list".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            Ok(Value::List(args))
        })),
    );
    
    env.borrow_mut().set(
        "first".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() != 1 {
                return Err(EvalError::ArityMismatch {
                    expected: 1,
                    got: args.len(),
                });
            }
            
            match &args[0] {
                Value::List(items) | Value::Vector(items) => {
                    if items.is_empty() {
                        Ok(Value::Nil)
                    } else {
                        Ok(items[0].clone())
                    }
                }
                _ => Err(EvalError::TypeError {
                    expected: "list or vector".to_string(),
                    got: format!("{:?}", args[0]),
                }),
            }
        })),
    );
    
    env.borrow_mut().set(
        "rest".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            if args.len() != 1 {
                return Err(EvalError::ArityMismatch {
                    expected: 1,
                    got: args.len(),
                });
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        Ok(Value::List(vec![]))
                    } else {
                        Ok(Value::List(items[1..].to_vec()))
                    }
                }
                Value::Vector(items) => {
                    if items.is_empty() {
                        Ok(Value::Vector(vec![]))
                    } else {
                        Ok(Value::Vector(items[1..].to_vec()))
                    }
                }
                _ => Err(EvalError::TypeError {
                    expected: "list or vector".to_string(),
                    got: format!("{:?}", args[0]),
                }),
            }
        })),
    );
    
    env
}


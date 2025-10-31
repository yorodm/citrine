use std::rc::Rc;
use std::cell::RefCell;
use crate::reader::{Value, Function, Environment, EvalError};

/// Creates a new standard environment with built-in functions
pub fn standard_env() -> Rc<RefCell<Environment>> {
    let env = Rc::new(RefCell::new(Environment::new()));
    
    // Register all built-in functions
    register_arithmetic_ops(&env);
    register_comparison_ops(&env);
    register_logical_ops(&env);
    register_list_ops(&env);
    
    env
}

/// Register arithmetic operations (+, -, *, /)
fn register_arithmetic_ops(env: &Rc<RefCell<Environment>>) {
    // Addition (+)
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
    
    // Subtraction (-)
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
    
    // Multiplication (*)
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
    
    // Division (/)
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
}

/// Register comparison operations (=, <, >)
fn register_comparison_ops(env: &Rc<RefCell<Environment>>) {
    // Equality (=)
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
    
    // Less than (<)
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
    
    // Greater than (>)
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
}

/// Register logical operations (not)
fn register_logical_ops(env: &Rc<RefCell<Environment>>) {
    // Logical not
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
}

/// Register list operations (list, first, rest)
fn register_list_ops(env: &Rc<RefCell<Environment>>) {
    // Create a list
    env.borrow_mut().set(
        "list".to_string(),
        Value::Function(Function::builtin(|args, _env| {
            Ok(Value::List(args))
        })),
    );
    
    // Get the first element of a list or vector
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
    
    // Get all elements except the first one
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
}


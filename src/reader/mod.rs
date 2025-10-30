mod value;


pub use value::*;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use crate::syntax::{SyntaxKind, SyntaxNode};

/// Reads a syntax node and converts it to a Citrine value
pub fn read(node: &SyntaxNode) -> Result<Value, EvalError> {
    match node.kind() {
        SyntaxKind::Root => {
            // Process all forms in the root node
            let mut forms = Vec::new();
            for child in node.children() {
                if child.kind() != SyntaxKind::Eof {
                    forms.push(read(&child)?);
                }
            }
            
            // If there's only one form, return it directly
            if forms.len() == 1 {
                Ok(forms.remove(0))
            } else {
                Ok(Value::List(forms))
            }
        }
        
        // Literals
        SyntaxKind::NumberLit => {
            let text = node.text().to_string();
            let number = text.parse::<f64>().map_err(|_| {
                EvalError::SyntaxError(format!("Invalid number: {}", text))
            })?;
            Ok(Value::Number(number))
        }
        SyntaxKind::StringLit => {
            let text = node.text().to_string();
            // Remove the quotes
            let content = text[1..text.len() - 1].to_string();
            Ok(Value::String(content))
        }
        SyntaxKind::SymbolLit => {
            let text = node.text().to_string();
            Ok(Value::Symbol(text))
        }
        SyntaxKind::KeywordLit => {
            let text = node.text().to_string();
            // Remove the leading colon
            let content = text[1..].to_string();
            Ok(Value::Keyword(content))
        }
        
        // Collections
        SyntaxKind::List => {
            let mut items = Vec::new();
            for child in node.children() {
                if !is_delimiter(child.kind()) {
                    items.push(read(&child)?);
                }
            }
            Ok(Value::List(items))
        }
        SyntaxKind::Vector => {
            let mut items = Vec::new();
            for child in node.children() {
                if !is_delimiter(child.kind()) {
                    items.push(read(&child)?);
                }
            }
            Ok(Value::Vector(items))
        }
        SyntaxKind::Map => {
            let mut map = HashMap::new();
            let mut key = None;
            
            for child in node.children() {
                if !is_delimiter(child.kind()) {
                    if let Some(k) = key.take() {
                        let v = read(&child)?;
                        map.insert(k, v);
                    } else {
                        key = Some(read(&child)?);
                    }
                }
            }
            
            // Check if we have an odd number of elements
            if key.is_some() {
                return Err(EvalError::SyntaxError("Map literal must have an even number of forms".to_string()));
            }
            
            Ok(Value::Map(map))
        }
        SyntaxKind::Set => {
            let mut set = HashSet::new();
            for child in node.children() {
                if !is_delimiter(child.kind()) {
                    set.insert(read(&child)?);
                }
            }
            Ok(Value::Set(set))
        }
        
        // Reader macros
        SyntaxKind::Quote => {
            let mut items = Vec::new();
            items.push(Value::Symbol("quote".to_string()));
            
            for child in node.children() {
                if child.kind() != SyntaxKind::Quote {
                    items.push(read(&child)?);
                }
            }
            
            Ok(Value::List(items))
        }
        SyntaxKind::Backtick => {
            let mut items = Vec::new();
            items.push(Value::Symbol("quasiquote".to_string()));
            
            for child in node.children() {
                if child.kind() != SyntaxKind::Backtick {
                    items.push(read(&child)?);
                }
            }
            
            Ok(Value::List(items))
        }
        SyntaxKind::Comma => {
            let mut items = Vec::new();
            items.push(Value::Symbol("unquote".to_string()));
            
            for child in node.children() {
                if child.kind() != SyntaxKind::Comma {
                    items.push(read(&child)?);
                }
            }
            
            Ok(Value::List(items))
        }
        SyntaxKind::CommaAt => {
            let mut items = Vec::new();
            items.push(Value::Symbol("unquote-splicing".to_string()));
            
            for child in node.children() {
                if child.kind() != SyntaxKind::CommaAt {
                    items.push(read(&child)?);
                }
            }
            
            Ok(Value::List(items))
        }
        
        // Other node types
        _ => {
            // For other node types, try to process their children
            let mut forms = Vec::new();
            for child in node.children() {
                forms.push(read(&child)?);
            }
            
            if forms.len() == 1 {
                Ok(forms.remove(0))
            } else if forms.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::List(forms))
            }
        }
    }
}

/// Evaluates a Citrine value in the given environment
pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> Result<Value, EvalError> {
    match value {
        // Self-evaluating forms
        Value::Nil | Value::Boolean(_) | Value::Number(_) | Value::String(_) | Value::Keyword(_) => {
            Ok(value.clone())
        }
        
        // Symbol lookup
        Value::Symbol(name) => {
            env.borrow().get(name).ok_or_else(|| EvalError::UnboundSymbol(name.clone()))
        }
        
        // List evaluation (function call or special form)
        Value::List(items) => {
            if items.is_empty() {
                return Ok(Value::List(vec![]));
            }
            
            // Get the first item (function or special form)
            let first = &items[0];
            
            // Check for special forms
            if let Value::Symbol(name) = first {
                match name.as_str() {
                    // Special form: setq
                    "setq" => {
                        if items.len() != 3 {
                            return Err(EvalError::ArityMismatch {
                                expected: 2,
                                got: items.len() - 1,
                            });
                        }
                        
                        let symbol = match &items[1] {
                            Value::Symbol(s) => s.clone(),
                            _ => return Err(EvalError::TypeError {
                                expected: "symbol".to_string(),
                                got: format!("{:?}", items[1]),
                            }),
                        };
                        
                        let value = eval(&items[2], env)?;
                        env.borrow_mut().set(symbol, value.clone());
                        
                        Ok(value)
                    }
                    
                    // Special form: fn
                    "fn" => {
                        if items.len() < 3 {
                            return Err(EvalError::ArityMismatch {
                                expected: 2,
                                got: items.len() - 1,
                            });
                        }
                        
                        let params = match &items[1] {
                            Value::Vector(params) => {
                                let mut param_names = Vec::new();
                                for param in params {
                                    match param {
                                        Value::Symbol(name) => param_names.push(name.clone()),
                                        _ => return Err(EvalError::TypeError {
                                            expected: "symbol".to_string(),
                                            got: format!("{:?}", param),
                                        }),
                                    }
                                }
                                param_names
                            }
                            _ => return Err(EvalError::TypeError {
                                expected: "vector".to_string(),
                                got: format!("{:?}", items[1]),
                            }),
                        };
                        
                        let body = items[2..].to_vec();
                        
                        Ok(Value::Function(Function::new(params, body, env.clone())))
                    }
                    
                    // Special form: macro
                    "macro" => {
                        if items.len() < 3 {
                            return Err(EvalError::ArityMismatch {
                                expected: 2,
                                got: items.len() - 1,
                            });
                        }
                        
                        let params = match &items[1] {
                            Value::Vector(params) => {
                                let mut param_names = Vec::new();
                                for param in params {
                                    match param {
                                        Value::Symbol(name) => param_names.push(name.clone()),
                                        _ => return Err(EvalError::TypeError {
                                            expected: "symbol".to_string(),
                                            got: format!("{:?}", param),
                                        }),
                                    }
                                }
                                param_names
                            }
                            _ => return Err(EvalError::TypeError {
                                expected: "vector".to_string(),
                                got: format!("{:?}", items[1]),
                            }),
                        };
                        
                        let body = items[2..].to_vec();
                        
                        Ok(Value::Macro(Macro::new(params, body, env.clone())))
                    }
                    
                    // Regular function call
                    _ => apply_function(items, env),
                }
            } else {
                // First item is not a symbol, try to evaluate it as a function
                apply_function(items, env)
            }
        }
        
        // Vector evaluation
        Value::Vector(items) => {
            let mut result = Vec::new();
            for item in items {
                result.push(eval(item, env)?);
            }
            Ok(Value::Vector(result))
        }
        
        // Map evaluation
        Value::Map(entries) => {
            let mut result = HashMap::new();
            for (k, v) in entries {
                let key = eval(k, env)?;
                let value = eval(v, env)?;
                result.insert(key, value);
            }
            Ok(Value::Map(result))
        }
        
        // Set evaluation
        Value::Set(items) => {
            let mut result = HashSet::new();
            for item in items {
                result.insert(eval(item, env)?);
            }
            Ok(Value::Set(result))
        }
        
        // Functions and macros evaluate to themselves
        Value::Function(_) | Value::Macro(_) => Ok(value.clone()),
    }
}

/// Applies a function to arguments
fn apply_function(items: &[Value], env: &Rc<RefCell<Environment>>) -> Result<Value, EvalError> {
    if items.is_empty() {
        return Err(EvalError::SyntaxError("Empty function application".to_string()));
    }
    
    // Evaluate the function
    let func = eval(&items[0], env)?;
    
    // Evaluate the arguments
    let mut args = Vec::new();
    for arg in &items[1..] {
        args.push(eval(arg, env)?);
    }
    
    // Apply the function
    match func {
        Value::Function(f) => {
            if f.is_builtin {
                // Call the built-in function
                if let Some(builtin) = f.builtin_fn {
                    return builtin(args, env);
                } else {
                    return Err(EvalError::Other("Built-in function has no implementation".to_string()));
                }
            }
            
            // Check arity
            if f.params.len() != args.len() {
                return Err(EvalError::ArityMismatch {
                    expected: f.params.len(),
                    got: args.len(),
                });
            }
            
            // Create a new environment for the function call
            let func_env = Rc::new(RefCell::new(Environment::with_outer(f.env.clone())));
            
            // Bind the arguments to the parameters
            for (param, arg) in f.params.iter().zip(args) {
                func_env.borrow_mut().set(param.clone(), arg);
            }
            
            // Evaluate the body
            let mut result = Value::Nil;
            for expr in &f.body {
                result = eval(expr, &func_env)?;
            }
            
            Ok(result)
        }
        Value::Macro(_) => {
            Err(EvalError::Other("Macro application not yet implemented".to_string()))
        }
        _ => Err(EvalError::NotCallable(func)),
    }
}

/// Checks if a syntax kind is a delimiter (parentheses, brackets, braces)
fn is_delimiter(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::LeftParen
            | SyntaxKind::RightParen
            | SyntaxKind::LeftBracket
            | SyntaxKind::RightBracket
            | SyntaxKind::LeftBrace
            | SyntaxKind::RightBrace
    )
}

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// Represents a Citrine value
#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(HashMap<Value, Value>),
    Set(HashSet<Value>),
    Function(Function),
    Macro(Macro),
}

/// Represents a Citrine function
#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Value>,
    pub env: Rc<RefCell<Environment>>,
    pub is_builtin: bool,
    pub builtin_fn: Option<BuiltinFn>,
}

/// Represents a Citrine macro
#[derive(Clone)]
pub struct Macro {
    pub params: Vec<String>,
    pub body: Vec<Value>,
    pub env: Rc<RefCell<Environment>>,
}

/// Type for built-in functions
pub type BuiltinFn = fn(Vec<Value>, &Rc<RefCell<Environment>>) -> Result<Value, EvalError>;

/// Environment for storing variable and function bindings
#[derive(Clone)]
pub struct Environment {
    bindings: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

/// Evaluation error
#[derive(Debug, Clone)]
pub enum EvalError {
    UnboundSymbol(String),
    NotCallable(Value),
    ArityMismatch { expected: usize, got: usize },
    TypeError { expected: String, got: String },
    SyntaxError(String),
    Other(String),
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            outer: None,
        }
    }

    /// Create a new environment with the given outer environment
    pub fn with_outer(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            bindings: HashMap::new(),
            outer: Some(outer),
        }
    }

    /// Set a value in the environment
    pub fn set(&mut self, key: String, val: Value) {
        self.bindings.insert(key, val);
    }

    /// Get a value from the environment
    pub fn get(&self, key: &str) -> Option<Value> {
        match self.bindings.get(key) {
            Some(val) => Some(val.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(key),
                None => None,
            },
        }
    }
}

impl Function {
    /// Create a new user-defined function
    pub fn new(params: Vec<String>, body: Vec<Value>, env: Rc<RefCell<Environment>>) -> Self {
        Function {
            params,
            body,
            env,
            is_builtin: false,
            builtin_fn: None,
        }
    }

    /// Create a new built-in function
    pub fn builtin(builtin_fn: BuiltinFn) -> Self {
        Function {
            params: vec![],
            body: vec![],
            env: Rc::new(RefCell::new(Environment::new())),
            is_builtin: true,
            builtin_fn: Some(builtin_fn),
        }
    }
}

impl Macro {
    /// Create a new macro
    pub fn new(params: Vec<String>, body: Vec<Value>, env: Rc<RefCell<Environment>>) -> Self {
        Macro {
            params,
            body,
            env,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Keyword(k) => write!(f, ":{}", k),
            Value::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{:?}", item)?;
                }
                write!(f, ")")
            }
            Value::Vector(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            }
            Value::Map(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{:?} {:?}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Set(items) => {
                write!(f, "#{{")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{:?}", item)?;
                }
                write!(f, "}}")
            }
            Value::Function(_) => write!(f, "#<function>"),
            Value::Macro(_) => write!(f, "#<macro>"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::Keyword(a), Value::Keyword(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Vector(a), Value::Vector(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            // Functions and macros are compared by identity
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => 0.hash(state),
            Value::Boolean(b) => {
                1.hash(state);
                b.hash(state);
            }
            Value::Number(n) => {
                2.hash(state);
                n.to_bits().hash(state);
            }
            Value::String(s) => {
                3.hash(state);
                s.hash(state);
            }
            Value::Symbol(s) => {
                4.hash(state);
                s.hash(state);
            }
            Value::Keyword(k) => {
                5.hash(state);
                k.hash(state);
            }
            Value::List(items) => {
                6.hash(state);
                for item in items {
                    item.hash(state);
                }
            }
            Value::Vector(items) => {
                7.hash(state);
                for item in items {
                    item.hash(state);
                }
            }
            // Maps and sets can't be hashed in a meaningful way
            // Functions and macros can't be hashed in a meaningful way
            _ => {
                // Use the pointer address as a fallback
                std::ptr::addr_of!(*self).hash(state);
            }
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UnboundSymbol(s) => write!(f, "Unbound symbol: {}", s),
            EvalError::NotCallable(v) => write!(f, "Not callable: {}", v),
            EvalError::ArityMismatch { expected, got } => {
                write!(f, "Arity mismatch: expected {} arguments, got {}", expected, got)
            }
            EvalError::TypeError { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            EvalError::SyntaxError(s) => write!(f, "Syntax error: {}", s),
            EvalError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for EvalError {}


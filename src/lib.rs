//! Citrine - A Clojure-like language implemented in Rust
//!
//! This crate provides a lexer and parser for the Citrine language,
//! a Clojure-like Lisp dialect. The parser produces a concrete syntax tree (CST)
//! using the rowan library, which can be used for further processing.

pub mod lexer;
pub mod parser;
pub mod syntax;
pub mod reader;
pub mod builtins;

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
    builtins::standard_env()
}

# Citrine

A Clojure-like language implemented in Rust.

## Overview

Citrine is a Clojure-like Lisp dialect with a hand-written lexer and parser that produces a concrete syntax tree (CST) using the rowan library. This makes it suitable for implementing language tooling like a Language Server Protocol (LSP) implementation in the future.

## Grammar

The language grammar is based on Clojure with some modifications:

```
(* A Clojure like Lisp grammar *)

<forms> := form *;
<form> := literal | list | vector | map | reader_macro;

list := <'('> forms <')'>;
vector := <'['> forms <']'>;
map := <'{'> (form form)* <'}'>;
set := <'#{'> forms <'}'>;

<reader_macro> := set
                | tag
                | quote
                | backtick
                | comma
                | comma_at
                | discard
                | comment;

quote := <'\''> form;
backtick := <'`'> form;
comma := <','> form;
comma_at := <',@'> list;

tag := <'^'> form form;

discard := <'#_'> form;
comment := #";[^\r\n]*";

<literal> := string
           | number
           | character
           | keyword
           | symbol;

string := r#""([^"\\]|\\.)*""#;  (* Handles escaped characters *)

<number> := double | hex | binary | bign | long | ratio;
double := r"-?[0-9]*\.[0-9]+([eE]-?[0-9]+)?"
        | r"-?Infinity"
        | r"-?NaN";
hex := r"0[xX][0-9a-fA-F]+";
binary := r"0[bB][10]+";
bign := r"-?[0-9]+[nN]";
ratio := r"-?[0-9]+/-?[0-9]+";
long := !ratio !double !hex !binary !bign r"-?[0-9]+[lL]?";

character := char_named 
           | r"\\u[0-9a-fA-F]{4}"  (* Unicode *)
           | r"\\."                 (* Any escaped char *)
           ;

char_named := r"\\newline" | r"\\return" | r"\\space" 
            | r"\\tab" | r"\\formfeed" | r"\\backspace";

keyword := r#":[A-Za-z!?\-+<>=$*%_/][A-Za-z\d!?\-+<>=$*%_/]*"#;
symbol := r#"[A-Za-z!?\-+<>=$*%_/][A-Za-z\d!?\-+<>=$*%_/]*"#;
```

## Features

- Hand-written lexer (no parsing libraries)
- Rowan-based parser that produces a concrete syntax tree (CST)
- Support for all Clojure-like forms: lists, vectors, maps, sets
- Support for reader macros: quote, backtick, unquote, etc.
- Support for literals: strings, numbers, characters, keywords, symbols
- Comprehensive test suite

## Usage

```rust
use citrine::{parse, tokenize};

// Tokenize input
let tokens = tokenize("(+ 1 2)");
println!("{:#?}", tokens);

// Parse input
let syntax = parse("(+ 1 2)");
println!("{:#?}", syntax);
```

## Architecture

The codebase is organized into three main modules:

1. **Lexer**: Tokenizes the input text into a stream of tokens.
2. **Parser**: Parses the tokens into a concrete syntax tree.
3. **Syntax**: Defines the syntax kinds and rowan integration.

## Future Work

- Implement a Language Server Protocol (LSP) for IDE integration
- Add semantic analysis
- Implement an evaluator/interpreter
- Add macros and other advanced features

## License

This project is licensed under the MIT License - see the LICENSE file for details.


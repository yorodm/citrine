use citrine::{tokenize, parse, eval_str, standard_env};
use citrine::reader::Value;
use expect_test::{expect, Expect};

fn check_tokenize(input: &str, expected_tokens: Expect) {
    let tokens = tokenize(input);
    expected_tokens.assert_eq(&format!("{:#?}", tokens));
}

fn check_parse(input: &str, expected_tree: Expect) {
    let syntax = parse(input);
    expected_tree.assert_eq(&format!("{:#?}", syntax));
}

#[test]
fn test_tokenize_simple() {
    check_tokenize(
        "(+ 1 2)",
        expect![[r#"
            [
                Token {
                    kind: LeftParen,
                    text: "(",
                    start: 0,
                    end: 1,
                },
                Token {
                    kind: Symbol,
                    text: "+",
                    start: 1,
                    end: 2,
                },
                Token {
                    kind: Number,
                    text: "1",
                    start: 3,
                    end: 4,
                },
                Token {
                    kind: Number,
                    text: "2",
                    start: 5,
                    end: 6,
                },
                Token {
                    kind: RightParen,
                    text: ")",
                    start: 6,
                    end: 7,
                },
                Token {
                    kind: Eof,
                    text: "",
                    start: 7,
                    end: 7,
                },
            ]"#]],
    );
}

#[test]
fn test_parse_simple() {
    check_parse(
        "(+ 1 2)",
        expect![[r#"
            Root@0..5
              List@0..5
                LeftParen@0..1 "("
                SymbolLit@1..2
                  Symbol@1..2 "+"
                NumberLit@2..3
                  Number@2..3 "1"
                NumberLit@3..4
                  Number@3..4 "2"
                RightParen@4..5 ")"
              Eof@5..5 ""
        "#]],
    );
}

#[test]
fn test_parse_nested() {
    check_parse(
        "(defn factorial [n] (if (= n 0) 1 (* n (factorial (- n 1)))))",
        expect![[r#"
            Root@0..48
              List@0..48
                LeftParen@0..1 "("
                SymbolLit@1..5
                  Symbol@1..5 "defn"
                SymbolLit@5..14
                  Symbol@5..14 "factorial"
                Vector@14..17
                  LeftBracket@14..15 "["
                  SymbolLit@15..16
                    Symbol@15..16 "n"
                  RightBracket@16..17 "]"
                List@17..47
                  LeftParen@17..18 "("
                  SymbolLit@18..20
                    Symbol@18..20 "if"
                  List@20..25
                    LeftParen@20..21 "("
                    SymbolLit@21..22
                      Symbol@21..22 "="
                    SymbolLit@22..23
                      Symbol@22..23 "n"
                    NumberLit@23..24
                      Number@23..24 "0"
                    RightParen@24..25 ")"
                  NumberLit@25..26
                    Number@25..26 "1"
                  List@26..46
                    LeftParen@26..27 "("
                    SymbolLit@27..28
                      Symbol@27..28 "*"
                    SymbolLit@28..29
                      Symbol@28..29 "n"
                    List@29..45
                      LeftParen@29..30 "("
                      SymbolLit@30..39
                        Symbol@30..39 "factorial"
                      List@39..44
                        LeftParen@39..40 "("
                        SymbolLit@40..41
                          Symbol@40..41 "-"
                        SymbolLit@41..42
                          Symbol@41..42 "n"
                        NumberLit@42..43
                          Number@42..43 "1"
                        RightParen@43..44 ")"
                      RightParen@44..45 ")"
                    RightParen@45..46 ")"
                  RightParen@46..47 ")"
                RightParen@47..48 ")"
              Eof@48..48 ""
        "#]],
    );
}

#[test]
fn test_eval_simple() {
    let env = standard_env();
    
    // Test arithmetic
    assert_eq!(eval_str("(+ 1 2 3)", &env).unwrap(), Value::Number(6.0));
    assert_eq!(eval_str("(- 10 2 3)", &env).unwrap(), Value::Number(5.0));
    assert_eq!(eval_str("(* 2 3 4)", &env).unwrap(), Value::Number(24.0));
    assert_eq!(eval_str("(/ 12 2 3)", &env).unwrap(), Value::Number(2.0));
    
    // Test comparison
    assert_eq!(eval_str("(= 1 1 1)", &env).unwrap(), Value::Boolean(true));
    assert_eq!(eval_str("(= 1 2 1)", &env).unwrap(), Value::Boolean(false));
    assert_eq!(eval_str("(< 1 2)", &env).unwrap(), Value::Boolean(true));
    assert_eq!(eval_str("(> 3 2)", &env).unwrap(), Value::Boolean(true));
    
    // Test variable binding
    eval_str("(setq x 42)", &env).unwrap();
    assert_eq!(eval_str("x", &env).unwrap(), Value::Number(42.0));
    
    // Test function definition and application
    eval_str("(setq add (fn [a b] (+ a b)))", &env).unwrap();
    assert_eq!(eval_str("(add 2 3)", &env).unwrap(), Value::Number(5.0));
    
    // Test nested expressions
    assert_eq!(
        eval_str("(+ (* 2 3) (- 10 5))", &env).unwrap(),
        Value::Number(11.0)
    );
}

#[test]
fn test_data_structures() {
    let env = standard_env();
    
    // Test list
    let result = eval_str("(list 1 2 3)", &env).unwrap();
    match result {
        Value::List(items) => {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Number(1.0));
            assert_eq!(items[1], Value::Number(2.0));
            assert_eq!(items[2], Value::Number(3.0));
        }
        _ => panic!("Expected a list"),
    }
    
    // Test vector
    let result = eval_str("[1 2 3]", &env).unwrap();
    match result {
        Value::Vector(items) => {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Number(1.0));
            assert_eq!(items[1], Value::Number(2.0));
            assert_eq!(items[2], Value::Number(3.0));
        }
        _ => panic!("Expected a vector"),
    }
    
    // Test map
    let result = eval_str("{:a 1 :b 2}", &env).unwrap();
    match result {
        Value::Map(map) => {
            assert_eq!(map.len(), 2);
            assert_eq!(
                map.get(&Value::Keyword("a".to_string())),
                Some(&Value::Number(1.0))
            );
            assert_eq!(
                map.get(&Value::Keyword("b".to_string())),
                Some(&Value::Number(2.0))
            );
        }
        _ => panic!("Expected a map"),
    }
    
    // Test set
    let result = eval_str("#{1 2 3}", &env).unwrap();
    match result {
        Value::Set(set) => {
            assert_eq!(set.len(), 3);
            assert!(set.contains(&Value::Number(1.0)));
            assert!(set.contains(&Value::Number(2.0)));
            assert!(set.contains(&Value::Number(3.0)));
        }
        _ => panic!("Expected a set"),
    }
}

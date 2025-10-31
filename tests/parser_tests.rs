use citrine::parse;
use expect_test::{expect, Expect};

fn check(input: &str, expected_tree: Expect) {
    let syntax = parse(input);
    expected_tree.assert_eq(&format!("{:#?}", syntax));
}

#[test]
fn test_parse_empty() {
    check(
        "",
        expect![[r#"
            Root@0..0
              Eof@0..0 ""
        "#]],
    );
}

#[test]
fn test_parse_list() {
    check(
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
fn test_parse_vector() {
    check(
        "[1 2 3]",
        expect![[r#"
            Root@0..5
              Vector@0..5
                LeftBracket@0..1 "["
                NumberLit@1..2
                  Number@1..2 "1"
                NumberLit@2..3
                  Number@2..3 "2"
                NumberLit@3..4
                  Number@3..4 "3"
                RightBracket@4..5 "]"
              Eof@5..5 ""
        "#]],
    );
}

#[test]
fn test_parse_map() {
    check(
        "{:a 1 :b 2}",
        expect![[r#"
            Root@0..8
              Map@0..8
                LeftBrace@0..1 "{"
                KeywordLit@1..3
                  Keyword@1..3 ":a"
                NumberLit@3..4
                  Number@3..4 "1"
                KeywordLit@4..6
                  Keyword@4..6 ":b"
                NumberLit@6..7
                  Number@6..7 "2"
                RightBrace@7..8 "}"
              Eof@8..8 ""
        "#]],
    );
}

#[test]
fn test_parse_quote() {
    check(
        "'(1 2 3)",
        expect![[r#"
            Root@0..6
              Quote@0..6
                QuoteToken@0..1 "'"
                List@1..6
                  LeftParen@1..2 "("
                  NumberLit@2..3
                    Number@2..3 "1"
                  NumberLit@3..4
                    Number@3..4 "2"
                  NumberLit@4..5
                    Number@4..5 "3"
                  RightParen@5..6 ")"
              Eof@6..6 ""
        "#]],
    );
}

#[test]
fn test_parse_backtick() {
    check(
        "`(1 2 ,x)",
        expect![[r#"
            Root@0..7
              Backtick@0..7
                BacktickToken@0..1 "`"
                List@1..7
                  LeftParen@1..2 "("
                  NumberLit@2..3
                    Number@2..3 "1"
                  NumberLit@3..4
                    Number@3..4 "2"
                  Comma@4..6
                    CommaToken@4..5 ","
                    SymbolLit@5..6
                      Symbol@5..6 "x"
                  RightParen@6..7 ")"
              Eof@7..7 ""
        "#]],
    );
}

#[test]
fn test_parse_meta() {
    check(
        "^:private (defn foo [])",
        expect![[r#"
            Root@0..20
              Meta@0..20
                CaretToken@0..1 "^"
                KeywordLit@1..9
                  Keyword@1..9 ":private"
                List@9..20
                  LeftParen@9..10 "("
                  SymbolLit@10..14
                    Symbol@10..14 "defn"
                  SymbolLit@14..17
                    Symbol@14..17 "foo"
                  Vector@17..19
                    LeftBracket@17..18 "["
                    RightBracket@18..19 "]"
                  RightParen@19..20 ")"
              Eof@20..20 ""
        "#]],
    );
}

#[test]
fn test_parse_complex() {
    check(
        "(defn hello [name] (str \"Hello, \" name \"!\"))",
        expect![[r#"
            Root@0..38
              List@0..38
                LeftParen@0..1 "("
                SymbolLit@1..5
                  Symbol@1..5 "defn"
                SymbolLit@5..10
                  Symbol@5..10 "hello"
                Vector@10..16
                  LeftBracket@10..11 "["
                  SymbolLit@11..15
                    Symbol@11..15 "name"
                  RightBracket@15..16 "]"
                List@16..37
                  LeftParen@16..17 "("
                  SymbolLit@17..20
                    Symbol@17..20 "str"
                  StringLit@20..29
                    String@20..29 "\"Hello, \""
                  SymbolLit@29..33
                    Symbol@29..33 "name"
                  StringLit@33..36
                    String@33..36 "\"!\""
                  RightParen@36..37 ")"
                RightParen@37..38 ")"
              Eof@38..38 ""
        "#]],
    );
}


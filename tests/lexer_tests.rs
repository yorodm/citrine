use citrine::lexer::{Lexer, TokenKind};

#[test]
fn test_lexer_simple_tokens() {
    let input = "()[]{}";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_reader_macros() {
    let input = "'`^#,,@";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(lexer.next_token().kind, TokenKind::Caret);
    assert_eq!(lexer.next_token().kind, TokenKind::Hash);
    assert_eq!(lexer.next_token().kind, TokenKind::Comma);
    assert_eq!(lexer.next_token().kind, TokenKind::CommaAt);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_string() {
    let input = r#""hello world" "with \"escape\"" "unterminated"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::String);
    assert_eq!(lexer.next_token().kind, TokenKind::String);
    assert_eq!(lexer.next_token().kind, TokenKind::Error); // unterminated string
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_keyword() {
    let input = ":keyword :with-dash :123";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::Keyword);
    assert_eq!(lexer.next_token().kind, TokenKind::Keyword);
    assert_eq!(lexer.next_token().kind, TokenKind::Keyword);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_symbol() {
    let input = "symbol with-dash symbol123 *special* +";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_comment() {
    let input = "; This is a comment\nsymbol";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::Comment);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_complex() {
    let input = "(defn hello [name] (str \"Hello, \" name \"!\"))";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol); // defn
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol); // hello
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol); // name
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol); // str
    assert_eq!(lexer.next_token().kind, TokenKind::String); // "Hello, "
    assert_eq!(lexer.next_token().kind, TokenKind::Symbol); // name
    assert_eq!(lexer.next_token().kind, TokenKind::String); // "!"
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}


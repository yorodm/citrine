use smol_str::SmolStr;
use std::fmt;

/// Represents the type of a token in the Citrine language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    
    // Literals
    String,       // "hello"
    Number,       // 123, 3.14, 0xFF, etc.
    Character,    // \a, \newline, etc.
    
    // Identifiers
    Symbol,       // my-symbol
    Keyword,      // :keyword
    
    // Reader macros
    Quote,        // '
    Backtick,     // `
    Caret,        // ^
    Hash,         // #
    HashLeftBrace, // #{
    
    // Operators
    Comma,        // ,
    CommaAt,      // ,@
    
    // Whitespace and comments
    Whitespace,   // space, tab, newline
    Comment,      // ; comment
    
    // Special
    Error,        // Invalid token
    Eof,          // End of file
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::String => write!(f, "string"),
            TokenKind::Number => write!(f, "number"),
            TokenKind::Character => write!(f, "character"),
            TokenKind::Symbol => write!(f, "symbol"),
            TokenKind::Keyword => write!(f, "keyword"),
            TokenKind::Quote => write!(f, "'"),
            TokenKind::Backtick => write!(f, "`"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::HashLeftBrace => write!(f, "#{{"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::CommaAt => write!(f, ",@"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Error => write!(f, "error"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

/// Represents a token in the Citrine language
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The text of the token
    pub text: SmolStr,
    /// The start position of the token in the source
    pub start: usize,
    /// The end position of the token in the source
    pub end: usize,
}

impl Token {
    /// Creates a new token
    pub fn new(kind: TokenKind, text: impl Into<SmolStr>, start: usize, end: usize) -> Self {
        Self {
            kind,
            text: text.into(),
            start,
            end,
        }
    }

    /// Returns the length of the token
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns whether the token is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}@{}..{} '{}'", self.kind, self.start, self.end, self.text)
    }
}

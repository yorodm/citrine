use rowan::Language;
use std::fmt;

/// The language type for Citrine
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CitrineLanguage;

/// The syntax kind for Citrine
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // Root
    Root = 0,
    
    // Forms
    List,
    Vector,
    Map,
    Set,
    
    // Literals
    StringLit,
    NumberLit,
    CharacterLit,
    KeywordLit,
    SymbolLit,
    
    // Reader macros
    Quote,
    Backtick,
    Unquote,
    UnquoteSplicing,
    Deref,
    Meta,
    Tag,
    Discard,
    
    // Special
    Comment,
    Whitespace,
    Error,
    
    // Tokens (leaf nodes)
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    
    String,
    Number,
    Character,
    Symbol,
    Keyword,
    
    QuoteToken,
    BacktickToken,
    TildeToken,
    TildeAtToken,
    CaretToken,
    HashToken,
    HashLeftBraceToken,
    
    CommaToken,
    
    CommentToken,
    WhitespaceToken,
    ErrorToken,
    Eof,
}

impl SyntaxKind {
    /// Returns true if this syntax kind is a token
    pub fn is_token(&self) -> bool {
        match self {
            SyntaxKind::LeftParen |
            SyntaxKind::RightParen |
            SyntaxKind::LeftBracket |
            SyntaxKind::RightBracket |
            SyntaxKind::LeftBrace |
            SyntaxKind::RightBrace |
            SyntaxKind::String |
            SyntaxKind::Number |
            SyntaxKind::Character |
            SyntaxKind::Symbol |
            SyntaxKind::Keyword |
            SyntaxKind::QuoteToken |
            SyntaxKind::BacktickToken |
            SyntaxKind::TildeToken |
            SyntaxKind::TildeAtToken |
            SyntaxKind::CaretToken |
            SyntaxKind::HashToken |
            SyntaxKind::HashLeftBraceToken |
            SyntaxKind::CommaToken |
            SyntaxKind::CommentToken |
            SyntaxKind::WhitespaceToken |
            SyntaxKind::ErrorToken |
            SyntaxKind::Eof => true,
            _ => false,
        }
    }

    /// Returns true if this syntax kind is trivia (whitespace or comment)
    pub fn is_trivia(&self) -> bool {
        matches!(self, SyntaxKind::WhitespaceToken | SyntaxKind::CommentToken)
    }
}

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SyntaxKind::Root => "Root",
            SyntaxKind::List => "List",
            SyntaxKind::Vector => "Vector",
            SyntaxKind::Map => "Map",
            SyntaxKind::Set => "Set",
            SyntaxKind::StringLit => "StringLit",
            SyntaxKind::NumberLit => "NumberLit",
            SyntaxKind::CharacterLit => "CharacterLit",
            SyntaxKind::KeywordLit => "KeywordLit",
            SyntaxKind::SymbolLit => "SymbolLit",
            SyntaxKind::Quote => "Quote",
            SyntaxKind::Backtick => "Backtick",
            SyntaxKind::Unquote => "Unquote",
            SyntaxKind::UnquoteSplicing => "UnquoteSplicing",
            SyntaxKind::Deref => "Deref",
            SyntaxKind::Meta => "Meta",
            SyntaxKind::Tag => "Tag",
            SyntaxKind::Discard => "Discard",
            SyntaxKind::Comment => "Comment",
            SyntaxKind::Whitespace => "Whitespace",
            SyntaxKind::Error => "Error",
            SyntaxKind::LeftParen => "LeftParen",
            SyntaxKind::RightParen => "RightParen",
            SyntaxKind::LeftBracket => "LeftBracket",
            SyntaxKind::RightBracket => "RightBracket",
            SyntaxKind::LeftBrace => "LeftBrace",
            SyntaxKind::RightBrace => "RightBrace",
            SyntaxKind::String => "String",
            SyntaxKind::Number => "Number",
            SyntaxKind::Character => "Character",
            SyntaxKind::Symbol => "Symbol",
            SyntaxKind::Keyword => "Keyword",
            SyntaxKind::QuoteToken => "QuoteToken",
            SyntaxKind::BacktickToken => "BacktickToken",
            SyntaxKind::TildeToken => "TildeToken",
            SyntaxKind::TildeAtToken => "TildeAtToken",
            SyntaxKind::CaretToken => "CaretToken",
            SyntaxKind::HashToken => "HashToken",
            SyntaxKind::HashLeftBraceToken => "HashLeftBraceToken",
            SyntaxKind::CommaToken => "CommaToken",
            SyntaxKind::CommentToken => "CommentToken",
            SyntaxKind::WhitespaceToken => "WhitespaceToken",
            SyntaxKind::ErrorToken => "ErrorToken",
            SyntaxKind::Eof => "Eof",
        };
        write!(f, "{}", name)
    }
}

impl Language for CitrineLanguage {
    type Kind = SyntaxKind;
    
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Eof as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

/// The syntax node type for Citrine
pub type SyntaxNode = rowan::SyntaxNode<CitrineLanguage>;

/// The syntax token type for Citrine
pub type SyntaxToken = rowan::SyntaxToken<CitrineLanguage>;

/// The syntax element type for Citrine
pub type SyntaxElement = rowan::SyntaxElement<CitrineLanguage>;

/// Converts a token kind to a syntax kind
pub fn token_to_syntax_kind(kind: crate::lexer::TokenKind) -> SyntaxKind {
    match kind {
        crate::lexer::TokenKind::LeftParen => SyntaxKind::LeftParen,
        crate::lexer::TokenKind::RightParen => SyntaxKind::RightParen,
        crate::lexer::TokenKind::LeftBracket => SyntaxKind::LeftBracket,
        crate::lexer::TokenKind::RightBracket => SyntaxKind::RightBracket,
        crate::lexer::TokenKind::LeftBrace => SyntaxKind::LeftBrace,
        crate::lexer::TokenKind::RightBrace => SyntaxKind::RightBrace,
        crate::lexer::TokenKind::String => SyntaxKind::String,
        crate::lexer::TokenKind::Number => SyntaxKind::Number,
        crate::lexer::TokenKind::Character => SyntaxKind::Character,
        crate::lexer::TokenKind::Symbol => SyntaxKind::Symbol,
        crate::lexer::TokenKind::Keyword => SyntaxKind::Keyword,
        crate::lexer::TokenKind::Quote => SyntaxKind::QuoteToken,
        crate::lexer::TokenKind::Backtick => SyntaxKind::BacktickToken,
        crate::lexer::TokenKind::Tilde => SyntaxKind::TildeToken,
        crate::lexer::TokenKind::TildeAt => SyntaxKind::TildeAtToken,
        crate::lexer::TokenKind::Caret => SyntaxKind::CaretToken,
        crate::lexer::TokenKind::Hash => SyntaxKind::HashToken,
        crate::lexer::TokenKind::HashLeftBrace => SyntaxKind::HashLeftBraceToken,
        crate::lexer::TokenKind::Comma => SyntaxKind::CommaToken,
        crate::lexer::TokenKind::Whitespace => SyntaxKind::WhitespaceToken,
        crate::lexer::TokenKind::Comment => SyntaxKind::CommentToken,
        crate::lexer::TokenKind::Error => SyntaxKind::ErrorToken,
        crate::lexer::TokenKind::Eof => SyntaxKind::Eof,
    }
}

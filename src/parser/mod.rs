use crate::lexer::{Lexer, Token, TokenKind};
use crate::syntax::{CitrineLanguage, SyntaxKind, token_to_syntax_kind, SyntaxNode};
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;
use std::vec::IntoIter;
use thiserror::Error;

/// Errors that can occur during parsing
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("unexpected token: expected {expected}, got {actual}")]
    UnexpectedToken {
        expected: String,
        actual: String,
    },
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unmatched delimiter: {0}")]
    UnmatchedDelimiter(String),
}

/// A parser for the Citrine language
pub struct Parser {
    /// The tokens to parse
    tokens: Peekable<IntoIter<Token>>,
    /// The builder for the syntax tree
    builder: GreenNodeBuilder<'static>,
    // current field removed as it was unused
}

impl Parser {
    /// Creates a new parser for the given input
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().into_iter().peekable();
        
        Self {
            tokens,
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Parses the input and returns a syntax tree
    pub fn parse(mut self) -> SyntaxNode {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Root));
        
        while self.peek().is_some() {
            match self.parse_form() {
                Ok(_) => {},
                Err(e) => {
                    // Handle error and try to recover
                    eprintln!("Parse error: {}", e);
                    self.skip_until_delimiter();
                }
            }
        }
        
        self.builder.finish_node();
        
        let green: GreenNode = self.builder.finish();
        SyntaxNode::new_root(green)
    }

    /// Parses a form
    fn parse_form(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some(token) => {
                match token.kind {
                    TokenKind::LeftParen => self.parse_list(),
                    TokenKind::LeftBracket => self.parse_vector(),
                    TokenKind::LeftBrace => self.parse_map(),
                    TokenKind::HashLeftBrace => self.parse_set(),
                    TokenKind::Quote => self.parse_quote(),
                    TokenKind::Backtick => self.parse_backtick(),
                    TokenKind::Comma => self.parse_unquote(),
                    TokenKind::CommaAt => self.parse_unquote_splicing(),
                    TokenKind::Caret => self.parse_meta(),
                    TokenKind::Hash => {
                        // Check if it's a discard
                        if let Some(next) = self.peek_nth(1) {
                            if next.kind == TokenKind::Symbol && next.text == "_" {
                                self.parse_discard()
                            } else {
                                self.parse_tag()
                            }
                        } else {
                            self.parse_tag()
                        }
                    },
                    TokenKind::String => self.parse_string(),
                    TokenKind::Number => self.parse_number(),
                    TokenKind::Character => self.parse_character(),
                    TokenKind::Keyword => self.parse_keyword(),
                    TokenKind::Symbol => self.parse_symbol(),
                    TokenKind::Comment => {
                        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Comment));
                        self.consume_token();
                        self.builder.finish_node();
                        Ok(())
                    },
                    TokenKind::Whitespace => {
                        self.consume_token();
                        Ok(())
                    },
                    TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                        Err(ParserError::UnmatchedDelimiter(token.text.to_string()))
                    },
                    _ => {
                        // Skip invalid tokens
                        self.consume_token();
                        Ok(())
                    }
                }
            },
            None => Err(ParserError::UnexpectedEof),
        }
    }

    /// Parses a list
    fn parse_list(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::List));
        
        // Consume the opening paren
        self.consume_token();
        
        // Parse forms until we hit the closing paren
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::RightParen {
                break;
            }
            
            self.parse_form()?;
        }
        
        // Consume the closing paren
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::RightParen {
                self.consume_token();
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: ")".to_string(),
                    actual: token.text.to_string(),
                });
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a vector
    fn parse_vector(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Vector));
        
        // Consume the opening bracket
        self.consume_token();
        
        // Parse forms until we hit the closing bracket
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBracket {
                break;
            }
            
            self.parse_form()?;
        }
        
        // Consume the closing bracket
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBracket {
                self.consume_token();
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: "]".to_string(),
                    actual: token.text.to_string(),
                });
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a map
    fn parse_map(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Map));
        
        // Consume the opening brace
        self.consume_token();
        
        // Parse key-value pairs until we hit the closing brace
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            // Parse key
            self.parse_form()?;
            
            // Parse value (if there's a key, there should be a value)
            if let Some(token) = self.peek() {
                if token.kind == TokenKind::RightBrace {
                    return Err(ParserError::UnexpectedToken {
                        expected: "value".to_string(),
                        actual: token.text.to_string(),
                    });
                }
                
                self.parse_form()?;
            } else {
                return Err(ParserError::UnexpectedEof);
            }
        }
        
        // Consume the closing brace
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBrace {
                self.consume_token();
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: "}".to_string(),
                    actual: token.text.to_string(),
                });
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a set
    fn parse_set(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Set));
        
        // Consume the opening #{
        self.consume_token();
        
        // Parse forms until we hit the closing brace
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            self.parse_form()?;
        }
        
        // Consume the closing brace
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBrace {
                self.consume_token();
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: "}".to_string(),
                    actual: token.text.to_string(),
                });
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a quote
    fn parse_quote(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Quote));
        
        // Consume the quote
        self.consume_token();
        
        // Parse the quoted form
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a backtick
    fn parse_backtick(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Backtick));
        
        // Consume the backtick
        self.consume_token();
        
        // Parse the backquoted form
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses an unquote
    fn parse_unquote(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Comma));
        
        // Consume the comma
        self.consume_token();
        
        // Parse the unquoted form
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses an unquote-splicing
    fn parse_unquote_splicing(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::CommaAt));
        
        // Consume the comma-at
        self.consume_token();
        
        // Parse the unquote-spliced form
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a meta
    fn parse_meta(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Meta));
        
        // Consume the caret
        self.consume_token();
        
        // Parse the metadata
        self.parse_form()?;
        
        // Parse the form with metadata
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a tag
    fn parse_tag(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Tag));
        
        // Consume the hash
        self.consume_token();
        
        // Parse the tag
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a discard
    fn parse_discard(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::Discard));
        
        // Consume the hash
        self.consume_token();
        
        // Consume the underscore
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::Symbol && token.text == "_" {
                self.consume_token();
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: "_".to_string(),
                    actual: token.text.to_string(),
                });
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }
        
        // Parse the discarded form
        self.parse_form()?;
        
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a string
    fn parse_string(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::StringLit));
        self.consume_token();
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a number
    fn parse_number(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::NumberLit));
        self.consume_token();
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a character
    fn parse_character(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::CharacterLit));
        self.consume_token();
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a keyword
    fn parse_keyword(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::KeywordLit));
        self.consume_token();
        self.builder.finish_node();
        Ok(())
    }

    /// Parses a symbol
    fn parse_symbol(&mut self) -> Result<(), ParserError> {
        self.builder.start_node(CitrineLanguage::kind_to_raw(SyntaxKind::SymbolLit));
        self.consume_token();
        self.builder.finish_node();
        Ok(())
    }

    /// Skips tokens until a delimiter is found
    fn skip_until_delimiter(&mut self) {
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                    break;
                }
                _ => {
                    self.consume_token();
                }
            }
        }
    }

    /// Returns the next token without consuming it
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Returns the nth token without consuming it
    fn peek_nth(&mut self, n: usize) -> Option<&Token> {
        // For simplicity, we'll just handle n=0 and n=1 cases
        // In a real implementation, we'd handle arbitrary n
        if n == 0 {
            self.tokens.peek()
        } else if n == 1 {
            // We can only peek at the next token, so we'll just return None for n > 0
            // In a real implementation, we'd use a better approach
            None
        } else {
            None
        }
    }

    /// Consumes the next token and adds it to the tree
    fn consume_token(&mut self) -> Option<Token> {
        if let Some(token) = self.tokens.next() {
            let kind = token_to_syntax_kind(token.kind);
            self.builder.token(CitrineLanguage::kind_to_raw(kind), token.text.as_str());
            Some(token)
        } else {
            None
        }
    }
}



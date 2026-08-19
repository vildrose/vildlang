//! Tokens produced by the Vildlang lexer.

use crate::span::Span;

/// Represents a token produced by the Vildlang lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of this token.
    pub kind: TokenKind,
    /// The span of this token.
    pub span: Span,
}

impl Token {
    /// Creates a new token with the given kind and span.
    #[must_use]
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Represents the kind of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    /// Represents the `fn` keyword.
    Fn,
    /// Represents the `return` keyword.
    Return,
    /// Represents the `when` keyword.
    When,

    // Names and literals
    /// Represents an identifier.
    Ident(String),
    /// Represents an integer literal.
    Int(i128),

    // Punctuation
    /// Represents a left parenthesis `(`.
    LeftParen,
    /// Represents a right parenthesis `)`.
    RightParen,
    /// Represents a left brace `{`.
    LeftBrace,
    /// Represents a right brace `}`.
    RightBrace,
    /// Represents a colon `:`.
    Colon,
    /// Represents a comma `,`.
    Comma,
    /// Represents a semicolon `;`.
    Semicolon,

    // Operators
    /// Represents a plus `+`.
    Plus,
    /// Represents a minus `-`.
    Minus,
    /// Represents a star `*`.
    Star,
    /// Represents a define `define` `:=`.
    Define,
    /// Represents an assign `assign` `=`.
    Assign,
    /// Represents a `return type` / `arrow` `->`.
    Arrow,

    // Always the final token.
    /// Represents the end of the file.
    Eof,
}

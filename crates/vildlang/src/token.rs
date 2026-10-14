//! Tokens produced by the Vildlang lexer.

use crate::span::Span;
use num_bigint::BigUint;

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
    /// Represents the `const` keyword.
    Const,

    // Names and literals
    /// Represents an identifier.
    Ident(String),
    /// Represents an integer literal.
    Int(BigUint),

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
    /// Represents a newline `\n`.
    Newline,
    /// Represents a bang `!`.
    Bang,

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

    // Always the final token.
    /// Represents the end of the file.
    Eof,
}

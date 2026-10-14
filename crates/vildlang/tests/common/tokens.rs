use num_bigint::BigUint;
use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

/// Returns the token kinds of the given source string.
pub fn kinds(source: &str) -> Vec<TokenKind> {
    lex(source, FileId(0))
        .tokens
        .into_iter()
        .map(|token| token.kind)
        .collect()
}

/// Returns a `TokenKind` with `BigUint`, taking in just `u64`.
pub fn int(value: u64) -> TokenKind {
    TokenKind::Int(BigUint::from(value))
}

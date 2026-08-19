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

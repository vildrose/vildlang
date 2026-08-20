use crate::common::tokens::kinds;
use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_keywords_and_identifiers() {
        assert_eq!(
            kinds("fn return when ident"),
            vec![
                TokenKind::Fn,
                TokenKind::Return,
                TokenKind::When,
                TokenKind::Ident("ident".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn lexes_negative_integer_as_two_tokens() {
        assert_eq!(
            kinds("-42"),
            vec![TokenKind::Minus, TokenKind::Int(42), TokenKind::Eof,],
        );
    }

    #[test]
    fn lexes_punctuation_and_operators() {
        assert_eq!(
            kinds("(){}:;,+-*="),
            vec![
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
                TokenKind::Colon,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Assign,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn lexes_the_sign_fixture_without_errors() {
        let source = "
            fn sign(x : int) -> int {
                return when compare3(x, 0) {
                    less: -1
                    equal: 0
                    greater: 1
                }
            }
        ";

        let output = lex(source, FileId(0));

        assert!(
            output.diagnostics.is_empty(),
            "unexpected lexer diagnostics: {:#?}",
            output.diagnostics,
        );

        assert_eq!(
            output
                .tokens
                .into_iter()
                .map(|token| token.kind)
                .collect::<Vec<_>>(),
            vec![
                TokenKind::Newline,
                TokenKind::Fn,
                TokenKind::Ident("sign".to_owned()),
                TokenKind::LeftParen,
                TokenKind::Ident("x".to_owned()),
                TokenKind::Colon,
                TokenKind::Ident("int".to_owned()),
                TokenKind::RightParen,
                TokenKind::Arrow,
                TokenKind::Ident("int".to_owned()),
                TokenKind::LeftBrace,
                TokenKind::Newline,
                TokenKind::Return,
                TokenKind::When,
                TokenKind::Ident("compare3".to_owned()),
                TokenKind::LeftParen,
                TokenKind::Ident("x".to_owned()),
                TokenKind::Comma,
                TokenKind::Int(0),
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::Newline,
                TokenKind::Ident("less".to_owned()),
                TokenKind::Colon,
                TokenKind::Minus,
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Ident("equal".to_owned()),
                TokenKind::Colon,
                TokenKind::Int(0),
                TokenKind::Newline,
                TokenKind::Ident("greater".to_owned()),
                TokenKind::Colon,
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::RightBrace,
                TokenKind::Newline,
                TokenKind::RightBrace,
                TokenKind::Newline,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn empty_input_emits_only_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn lexes_identifier_with_underscores_and_digits() {
        assert_eq!(
            kinds("_temp value_2 compare3"),
            vec![
                TokenKind::Ident("_temp".to_owned()),
                TokenKind::Ident("value_2".to_owned()),
                TokenKind::Ident("compare3".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn keyword_prefixes_are_identifiers() {
        assert_eq!(
            kinds("function return_value whenever"),
            vec![
                TokenKind::Ident("function".to_owned()),
                TokenKind::Ident("return_value".to_owned()),
                TokenKind::Ident("whenever".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn digit_prefix_splits_integer_and_identifier() {
        assert_eq!(
            kinds("123abc"),
            vec![
                TokenKind::Int(123),
                TokenKind::Ident("abc".to_owned()),
                TokenKind::Eof,
            ],
        );
    }
}

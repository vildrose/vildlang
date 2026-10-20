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
                TokenKind::Ident,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn lexes_negative_integer_as_two_tokens() {
        assert_eq!(
            kinds("-42"),
            vec![TokenKind::Operator, TokenKind::Int, TokenKind::Eof,],
        );
    }

    // TODO: Remake this test with parsing in the future
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
                TokenKind::Operator, // One operator token, multiple operators. This is for the parser to deal with
                // NOTE: Even assign gets swallowed in this lexing loop...
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn lexes_the_sign_fixture_without_errors() {
        let source = "
            fn sign(x : int) : int {
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
                TokenKind::Ident,
                TokenKind::LeftParen,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Ident,
                TokenKind::RightParen,
                TokenKind::Colon,
                TokenKind::Ident,
                TokenKind::LeftBrace,
                TokenKind::Newline,
                TokenKind::Return,
                TokenKind::When,
                TokenKind::Ident,
                TokenKind::LeftParen,
                TokenKind::Ident,
                TokenKind::Comma,
                TokenKind::Int,
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::Newline,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Operator,
                TokenKind::Int,
                TokenKind::Newline,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Int,
                TokenKind::Newline,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Int,
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

    /* TODO: Move this kind of test to parser in the future
    #[test]
    fn lexes_identifier_with_underscores_and_digits() {
        assert_eq!(
            kinds("_temp value_2 compare3"),
            vec![
                TokenKind::Ident,
                TokenKind::Ident,
                TokenKind::Ident,
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
            vec![int(123), TokenKind::Ident("abc".to_owned()), TokenKind::Eof,],
        );
    }*/
}

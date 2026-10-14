use crate::common::tokens::kinds;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_define_operator_as_one_token() {
        assert_eq!(
            kinds("foo := bar"),
            vec![
                TokenKind::Ident("foo".to_owned()),
                TokenKind::Define,
                TokenKind::Ident("bar".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn lexes_assignment_operator() {
        assert_eq!(
            kinds("foo = bar"),
            vec![
                TokenKind::Ident("foo".to_owned()),
                TokenKind::Assign,
                TokenKind::Ident("bar".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn colon_without_equal_is_colon() {
        assert_eq!(
            kinds("x : int"),
            vec![
                TokenKind::Ident("x".to_owned()),
                TokenKind::Colon,
                TokenKind::Ident("int".to_owned()),
                TokenKind::Eof,
            ],
        );
    }
}

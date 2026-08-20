use crate::common::tokens::kinds;
use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newline_is_emitted_outside_parentheses() {
        assert_eq!(
            kinds("first\nsecond"),
            vec![
                TokenKind::Ident("first".to_owned()),
                TokenKind::Newline,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn multiple_blank_lines_emit_multiple_newline_tokens() {
        assert_eq!(
            kinds("first\n\n\nsecond"),
            vec![
                TokenKind::Ident("first".to_owned()),
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn newline_is_suppressed_inside_parentheses() {
        assert_eq!(
            kinds(
                "call(\n\
                    first,\n\
                    second,\n\
                )"
            ),
            vec![
                TokenKind::Ident("call".to_owned()),
                TokenKind::LeftParen,
                TokenKind::Ident("first".to_owned()),
                TokenKind::Comma,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Comma,
                TokenKind::RightParen,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn newline_is_emitted_inside_braces() {
        assert_eq!(
            kinds("{\nfirst\nsecond\n}"),
            vec![
                TokenKind::LeftBrace,
                TokenKind::Newline,
                TokenKind::Ident("first".to_owned()),
                TokenKind::Newline,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Newline,
                TokenKind::RightBrace,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn line_comment_preserves_following_newline() {
        assert_eq!(
            kinds("first // explanation\nsecond"),
            vec![
                TokenKind::Ident("first".to_owned()),
                TokenKind::Newline,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn block_comment_preserves_newline_separator() {
        assert_eq!(
            kinds("first /* first line\nsecond line */ second"),
            vec![
                TokenKind::Ident("first".to_owned()),
                TokenKind::Newline,
                TokenKind::Ident("second".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn block_comment_newline_is_suppressed_inside_parentheses() {
        assert_eq!(
            kinds("call(/* comment\ncontinued */ value)"),
            vec![
                TokenKind::Ident("call".to_owned()),
                TokenKind::LeftParen,
                TokenKind::Ident("value".to_owned()),
                TokenKind::RightParen,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn newline_span_covers_only_newline_character() {
        let output = lex("x\ny", FileId(0));

        let newline = &output.tokens[1];

        assert_eq!(newline.kind, TokenKind::Newline);
        assert_eq!(newline.span.start.byte, 1);
        assert_eq!(newline.span.end.byte, 2);
        assert_eq!(newline.span.start.line, 1);
        assert_eq!(newline.span.start.column, 2);
        assert_eq!(newline.span.end.line, 2);
        assert_eq!(newline.span.end.column, 1);
    }
}

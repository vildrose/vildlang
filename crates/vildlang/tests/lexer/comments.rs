use crate::common::tokens::kinds;
use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_comments_emit_no_tokens() {
        assert_eq!(kinds("// This is a comment"), vec![TokenKind::Eof],);
    }

    #[test]
    fn block_comments_emit_no_tokens() {
        assert_eq!(kinds("/* This is a comment */"), vec![TokenKind::Eof],);
    }

    #[test]
    fn unterminated_block_comment_is_a_diagnostic() {
        let output = lex("/* This is an unterminated comment", FileId(0));

        let diagnostic = output
            .diagnostics
            .first()
            .expect("unterminated block comment should produce one diagnostic");

        assert_eq!(diagnostic.message, "unterminated block comment");

        assert_eq!(
            output
                .tokens
                .into_iter()
                .map(|token| token.kind)
                .collect::<Vec<_>>(),
            vec![TokenKind::Eof],
        );
    }

    #[test]
    fn comments_are_skipped() {
        let source = "
            /* Block comment. */
            // Line comment.
            fn plus_one(x : int) : int {
                return x + 1; // Trailing line comment.
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
                TokenKind::Fn,
                TokenKind::Ident("plus_one".to_owned()),
                TokenKind::LeftParen,
                TokenKind::Ident("x".to_owned()),
                TokenKind::Colon,
                TokenKind::Ident("int".to_owned()),
                TokenKind::RightParen,
                TokenKind::Colon,
                TokenKind::Ident("int".to_owned()),
                TokenKind::LeftBrace,
                TokenKind::Return,
                TokenKind::Ident("x".to_owned()),
                TokenKind::Plus,
                TokenKind::Int(1),
                TokenKind::Semicolon,
                TokenKind::RightBrace,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn comments_update_line_numbers() {
        let output = lex(
            "/* First line.\nSecond line. */\nfn answer() : int { return 42; }",
            FileId(0),
        );

        let fn_token = output
            .tokens
            .first()
            .expect("expected `fn` token after comment");

        assert_eq!(fn_token.kind, TokenKind::Fn);
        assert_eq!(fn_token.span.start.line, 3);
        assert_eq!(fn_token.span.start.column, 1);
    }

    #[test]
    fn line_comment_at_end_of_file_needs_no_newline() {
        assert_eq!(
            kinds("x // Trailing comment with no newline"),
            vec![TokenKind::Ident("x".to_owned()), TokenKind::Eof,],
        );
    }

    #[test]
    fn empty_block_comment_is_skipped() {
        assert_eq!(
            kinds("x /**/ + 1"),
            vec![
                TokenKind::Ident("x".to_owned()),
                TokenKind::Plus,
                TokenKind::Int(1),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn block_comment_can_contain_punctuation() {
        assert_eq!(
            kinds("x /* () {} := -> + - * ; , */ + 1"),
            vec![
                TokenKind::Ident("x".to_owned()),
                TokenKind::Plus,
                TokenKind::Int(1),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn comment_between_tokens_does_not_merge_identifiers() {
        assert_eq!(
            kinds("left/* ignored */right"),
            vec![
                TokenKind::Ident("left".to_owned()),
                TokenKind::Ident("right".to_owned()),
                TokenKind::Eof,
            ],
        );
    }
}

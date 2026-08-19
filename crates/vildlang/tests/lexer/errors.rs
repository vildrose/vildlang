use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_unknown_characters_and_keeps_lexing() {
        let output = lex("fn @ sign", FileId(0));

        assert_eq!(output.diagnostics.len(), 1);
        assert_eq!(output.diagnostics[0].message, "unexpected character `@`");

        assert_eq!(
            output
                .tokens
                .into_iter()
                .map(|token| token.kind)
                .collect::<Vec<_>>(),
            vec![
                TokenKind::Fn,
                TokenKind::Ident("sign".to_owned()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn reports_each_invalid_character_and_recovers() {
        let output = lex("@ x #", FileId(0));

        assert_eq!(output.diagnostics.len(), 2);

        assert_eq!(output.diagnostics[0].message, "unexpected character `@`");
        assert_eq!(output.diagnostics[1].message, "unexpected character `#`");

        assert_eq!(
            output
                .tokens
                .into_iter()
                .map(|token| token.kind)
                .collect::<Vec<_>>(),
            vec![TokenKind::Ident("x".to_owned()), TokenKind::Eof,],
        );
    }

    #[test]
    fn integer_too_large_is_a_diagnostic() {
        let output = lex("999999999999999999999999999999999999999", FileId(0));

        assert_eq!(output.diagnostics.len(), 1);

        assert!(
            output.diagnostics[0]
                .message
                .contains("does not fit in i128"),
            "unexpected diagnostic: {}",
            output.diagnostics[0].message,
        );

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
    fn unexpected_character_diagnostic_has_correct_span() {
        let output = lex("x\n  @", FileId(0));

        assert_eq!(output.diagnostics.len(), 1);

        let diagnostic = &output.diagnostics[0];

        assert_eq!(diagnostic.message, "unexpected character `@`");

        assert_eq!(diagnostic.span.start.byte, 4);
        assert_eq!(diagnostic.span.end.byte, 5);

        assert_eq!(diagnostic.span.start.line, 2);
        assert_eq!(diagnostic.span.start.column, 3);

        assert_eq!(diagnostic.span.end.line, 2);
        assert_eq!(diagnostic.span.end.column, 4);
    }
}

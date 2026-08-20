use vildlang::lexer::lex;
use vildlang::span::FileId;
use vildlang::token::TokenKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_spans_track_source_locations() {
        let output = lex("fn sign", FileId(0));

        let fn_token = &output.tokens[0];
        assert_eq!(fn_token.kind, TokenKind::Fn);
        assert_eq!(fn_token.span.start.byte, 0);
        assert_eq!(fn_token.span.end.byte, 2);
        assert_eq!(fn_token.span.start.line, 1);
        assert_eq!(fn_token.span.start.line, 1);
        assert_eq!(fn_token.span.start.column, 1);
        assert_eq!(fn_token.span.end.column, 3);

        let sign_token = &output.tokens[1];
        assert_eq!(sign_token.kind, TokenKind::Ident("sign".to_owned()));
        assert_eq!(sign_token.span.start.byte, 3);
        assert_eq!(sign_token.span.end.byte, 7);
        assert_eq!(sign_token.span.start.line, 1);
        assert_eq!(sign_token.span.end.line, 1);
        assert_eq!(sign_token.span.start.column, 4);
        assert_eq!(sign_token.span.end.column, 8);
    }

    #[test]
    fn eof_span_is_at_end_of_source() {
        let output = lex("fn", FileId(0));

        let eof = output.tokens.last().expect("lexer must emit EOF");

        assert_eq!(eof.kind, TokenKind::Eof);
        assert_eq!(eof.span.start.byte, 2);
        assert_eq!(eof.span.end.byte, 2);
        assert_eq!(eof.span.start.line, 1);
        assert_eq!(eof.span.end.line, 1);
        assert_eq!(eof.span.start.column, 3);
        assert_eq!(eof.span.end.column, 3);
    }

    #[test]
    fn spans_track_multiple_lines() {
        let output = lex("x\n  y", FileId(0));

        let x = &output.tokens[0];
        assert_eq!(x.kind, TokenKind::Ident("x".to_owned()));
        assert_eq!(x.span.start.line, 1);
        assert_eq!(x.span.start.column, 1);
        assert_eq!(x.span.end.line, 1);
        assert_eq!(x.span.end.column, 2);

        let newline = &output.tokens[1];
        assert_eq!(newline.kind, TokenKind::Newline);

        let y = &output.tokens[2];
        assert_eq!(y.kind, TokenKind::Ident("y".to_owned()));
        assert_eq!(y.span.start.byte, 4);
        assert_eq!(y.span.end.byte, 5);
        assert_eq!(y.span.start.line, 2);
        assert_eq!(y.span.start.column, 3);
        assert_eq!(y.span.end.line, 2);
        assert_eq!(y.span.end.column, 4);
    }

    #[test]
    fn define_operator_span_covers_both_characters() {
        let output = lex("foo := bar", FileId(0));

        let define = &output.tokens[1];

        assert_eq!(define.kind, TokenKind::Define);
        assert_eq!(define.span.start.byte, 4);
        assert_eq!(define.span.end.byte, 6);
        assert_eq!(define.span.start.line, 1);
        assert_eq!(define.span.end.line, 1);
        assert_eq!(define.span.start.column, 5);
        assert_eq!(define.span.end.column, 7);
    }
}

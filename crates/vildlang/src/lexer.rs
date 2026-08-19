//! Lexer for the Vildlang programming language.

use crate::span::{FileId, Position, Span};
use crate::token::{Token, TokenKind};

/// Represents a diagnostic message from the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexDiagnostic {
    /// The span where the diagnostic occurred.
    pub span: Span,
    /// Human-readable diagnostic text.
    pub message: String,
}

/// Represents the output of lexing source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexOutput {
    /// Tokens emitted from the source, always ending with [`TokenKind::Eof`].
    pub tokens: Vec<Token>,
    /// Diagnostics emitted while lexing.
    pub diagnostics: Vec<LexDiagnostic>,
}

/// Lexes source text into Vildlang tokens.
#[must_use]
pub fn lex(source: &str, file: FileId) -> LexOutput {
    Lexer::new(source, file).lex_all()
}

struct Lexer<'a> {
    source: &'a str,
    file: FileId,
    chars: std::str::CharIndices<'a>,
    current: Option<(usize, char)>,
    next: Option<(usize, char)>,
    position: Position,
    tokens: Vec<Token>,
    diagnostics: Vec<LexDiagnostic>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str, file: FileId) -> Self {
        let mut chars = source.char_indices();

        Self {
            source,
            file,
            current: chars.next(),
            next: chars.next(),
            chars,
            position: Position::start(),
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn lex_all(mut self) -> LexOutput {
        while self.current.is_some() {
            if self.skip_trivia() {
                continue;
            }

            let start = self.position;
            let ch = self
                .current_char()
                .expect("lexer loop requires a current character");

            match ch {
                '(' => self.lex_single(TokenKind::LeftParen, start),
                ')' => self.lex_single(TokenKind::RightParen, start),
                '{' => self.lex_single(TokenKind::LeftBrace, start),
                '}' => self.lex_single(TokenKind::RightBrace, start),
                ',' => self.lex_single(TokenKind::Comma, start),
                ';' => self.lex_single(TokenKind::Semicolon, start),
                '+' => self.lex_single(TokenKind::Plus, start),
                '*' => self.lex_single(TokenKind::Star, start),
                '=' => self.lex_single(TokenKind::Assign, start),

                ':' => self.lex_colon_or_define(start),
                '-' => self.lex_minus_or_arrow(start),

                ch if ch.is_ascii_digit() => self.lex_integer(start),
                ch if is_ident_start(ch) => self.lex_identifier_or_keyword(start),

                _ => self.lex_unexpected_character(start),
            }
        }

        self.finish()
    }

    /// Skips source content that is relevant to spans but irrelevant to grammar.
    ///
    /// This includes whitespace and ordinary comments. Returns `true` if any
    /// source text was consumed.
    fn skip_trivia(&mut self) -> bool {
        match self.current_char() {
            Some(' ' | '\t' | '\r' | '\n') => {
                self.advance();
                true
            }

            Some('/') if self.next_is('/') => {
                self.skip_line_comment();
                true
            }

            Some('/') if self.next_is('*') => {
                self.skip_block_comment();
                true
            }

            _ => false,
        }
    }

    /// Skips `//` through the end of the current line.
    ///
    /// The newline itself remains for `skip_trivia` to consume on the next
    /// lexer iteration, so all newline position updates share one path.
    fn skip_line_comment(&mut self) {
        self.advance(); // First '/'
        self.advance(); // Second '/'

        while !matches!(self.current_char(), None | Some('\n')) {
            self.advance();
        }
    }

    /// Skips a non-nesting `/* ... */` comment.
    ///
    /// Emits one diagnostic when the source ends before the closing `*/`.
    fn skip_block_comment(&mut self) {
        let start = self.position;

        self.advance(); // '/'
        self.advance(); // '*'

        loop {
            match (self.current_char(), self.next_char()) {
                (None, _) => {
                    self.diagnostics.push(LexDiagnostic {
                        span: Span::new(self.file, start, self.position),
                        message: "unterminated block comment".to_owned(),
                    });
                    return;
                }

                (Some('*'), Some('/')) => {
                    self.advance(); // '*'
                    self.advance(); // '/'
                    return;
                }

                _ => self.advance(),
            }
        }
    }

    /// Consumes one source character and emits a fixed token kind.
    fn lex_single(&mut self, kind: TokenKind, start: Position) {
        self.advance();
        self.emit(kind, start);
    }

    /// Lexes `:` or the declaration operator `:=`.
    fn lex_colon_or_define(&mut self, start: Position) {
        self.advance(); // ':'

        if self.current_is('=') {
            self.advance();
            self.emit(TokenKind::Define, start);
        } else {
            self.emit(TokenKind::Colon, start);
        }
    }

    /// Lexes `-` or the return-type arrow `->`.
    fn lex_minus_or_arrow(&mut self, start: Position) {
        self.advance(); // '-'

        if self.current_is('>') {
            self.advance();
            self.emit(TokenKind::Arrow, start);
        } else {
            self.emit(TokenKind::Minus, start);
        }
    }

    /// Lexes an ASCII decimal integer literal.
    fn lex_integer(&mut self, start: Position) {
        let start_byte = start.byte;

        while matches!(self.current_char(), Some(ch) if ch.is_ascii_digit()) {
            self.advance();
        }

        let end_byte = self.position.byte;
        let text = &self.source[start_byte..end_byte];

        match text.parse::<i128>() {
            Ok(value) => self.emit(TokenKind::Int(value), start),
            Err(_) => self.diagnostics.push(LexDiagnostic {
                span: Span::new(self.file, start, self.position),
                message: format!("integer literal `{text}` does not fit in i128"),
            }),
        }
    }

    /// Lexes an identifier, converting reserved words to keyword tokens.
    fn lex_identifier_or_keyword(&mut self, start: Position) {
        let start_byte = start.byte;

        while matches!(self.current_char(), Some(ch) if is_ident_continue(ch)) {
            self.advance();
        }

        let end_byte = self.position.byte;
        let text = &self.source[start_byte..end_byte];

        let kind = match text {
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "when" => TokenKind::When,
            _ => TokenKind::Ident(text.to_owned()),
        };

        self.emit(kind, start);
    }

    /// Reports one invalid source character and consumes it to recover.
    fn lex_unexpected_character(&mut self, start: Position) {
        let unexpected = self
            .current_char()
            .expect("unexpected-character handler requires current character");

        self.advance();

        self.diagnostics.push(LexDiagnostic {
            span: Span::new(self.file, start, self.position),
            message: format!("unexpected character `{unexpected}`"),
        });
    }

    fn emit(&mut self, kind: TokenKind, start: Position) {
        let span = Span::new(self.file, start, self.position);
        self.tokens.push(Token::new(kind, span));
    }

    fn finish(mut self) -> LexOutput {
        let eof = Span::empty(self.file, self.position);
        self.tokens.push(Token::new(TokenKind::Eof, eof));

        LexOutput {
            tokens: self.tokens,
            diagnostics: self.diagnostics,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.current.map(|(_, ch)| ch)
    }

    fn next_char(&self) -> Option<char> {
        self.next.map(|(_, ch)| ch)
    }

    fn advance(&mut self) {
        let Some((byte, ch)) = self.current else {
            return;
        };

        if ch == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }

        self.position.byte = byte + ch.len_utf8();
        self.current = self.next;
        self.next = self.chars.next();
    }

    const fn current_is(&self, expected: char) -> bool {
        matches!(self.current, Some((_, ch)) if ch == expected)
    }

    const fn next_is(&self, expected: char) -> bool {
        matches!(self.next, Some((_, ch)) if ch == expected)
    }
}

const fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

const fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

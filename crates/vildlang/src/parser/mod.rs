//! Vildlang parser
//!
//! Handles:
//! - the Parser<'a> struct
//! - tracking LexOutput,
//! - current token cursor
//! - diagnostics vector
//! - token advancement `advance()`
//! - assertions `expect(TokenKind::Colon)`
//! - lookahead operations `peek()`

pub mod decl;
pub mod dialect;
pub mod expr;
pub mod stmt;

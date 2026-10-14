//! Example: Lexing the sign function from the Vildlang standard library.

use vildlang::lexer::lex;
use vildlang::span::FileId;

/// Lexes the sign function from the Vildlang standard library.
fn main() {
    let source = "fn sign(x : int) : int {
        return when compare3(x, 0) {
            less: -1
            equal: 0
            greater: 1
        }
    }";

    let output = lex(source, FileId(0));

    println!(
        "{:?}",
        output
            .tokens
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>()
    );
}

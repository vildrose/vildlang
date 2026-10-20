//! Pratt parser implementation for expressions for the parser
//!
//! Handles:
//! - Variables `Ident`
//! - literals `Int`, `String`
//! - array/slice indexing `[x]`
//! - function calls `foo(x)`

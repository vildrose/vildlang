//! Statement evaluator for parser
//!
//! Handles:
//! - `when` blocks and their individual `match` arms
//! - `return` statements
//! - Variable bindings `mut x := 5`, `y := 10`
//! - Block scopes `({ ... })`
//! - Expression statements

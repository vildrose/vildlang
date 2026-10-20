//! Dialect desugaring helper for parser
//!
//! Handles:
//! - A map of which oprators are currently active in scope
//! - provides a method like `get_operator_info(token_text)` that `expr.rs` calls

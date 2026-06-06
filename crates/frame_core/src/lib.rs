//! Shared AST, diagnostics, semantic validation, and token tables for Frame.
//!
//! This crate is intentionally integration-agnostic so the parser, code
//! generators, CLI, and future LSP can agree on one compiler data model.

pub mod ast;
pub mod diagnostics;
pub mod formatting;
pub mod ir;
pub mod knowledge;
pub mod semantic;
pub mod symbols;
pub mod tokens;

pub use ast::*;
pub use diagnostics::*;

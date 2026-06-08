//! AST-backed semantic cursor model for Frame IDE intelligence.
//!
//! This module builds a semantic understanding of the cursor position
//! from the parsed AST and source text. It powers completions, hovers,
//! diagnostics, references, and future editor features.
//!
//! Design goals:
//! - AST-first: use the parser/AST instead of only line-prefix scanning
//! - Tolerant: work with incomplete/broken syntax
//! - Reusable: consumed by completions, hover, references, diagnostics
//! - Fallback: degrade gracefully to text heuristics when AST is insufficient

pub mod cursor;

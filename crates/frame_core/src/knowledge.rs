//! Knowledge tables — canonical definitions live in `language.rs`.
//!
//! This module is a compatibility wrapper that re-exports the legacy
//! types and delegates lookups to the canonical registry where possible.

pub use crate::language::{
    declaration_keywords, property_keywords, ConceptKind, FrameConcept, FrameScope,
};

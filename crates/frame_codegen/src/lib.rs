//! CSS and TypeScript generators for parsed Frame documents.
//!
//! The output is deterministic and uses stable `fr-*` class names so generated
//! files are practical to inspect and consume from Svelte.

pub mod css;
pub mod ir_json;
pub mod typescript;

pub use css::generate_css;
pub use ir_json::generate_ir_json;
pub use typescript::generate_typescript;

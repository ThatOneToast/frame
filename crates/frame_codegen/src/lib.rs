//! CSS and TypeScript generators for parsed Frame documents.
//!
//! The output is deterministic and uses stable `fr-*` class names so generated
//! files are practical to inspect and consume from Svelte.

pub mod css;
pub mod typescript;

pub use css::generate_css;
pub use typescript::generate_typescript;

pub use frame_core::language::CompletionCategory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionSuggestion {
    pub label: String,
    pub detail: &'static str,
    pub documentation: String,
    pub insert_text: Option<String>,
    pub is_snippet: bool,
    pub category: CompletionCategory,
    pub sort_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SnippetScope {
    Root,
    Grid,
    Component,
    State,
    Keyframes,
    Animation,
}

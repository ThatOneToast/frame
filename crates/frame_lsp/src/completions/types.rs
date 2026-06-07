#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionSuggestion {
    pub label: String,
    pub detail: &'static str,
    pub documentation: String,
    pub insert_text: Option<String>,
    pub is_snippet: bool,
    pub category: CompletionCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionCategory {
    Snippet,
    Declaration,
    Include,
    LayoutProperty,
    VisualProperty,
    MotionProperty,
    TypographyProperty,
    TokenProperty,
    AdvancedProperty,
    StateBlock,
    Value,
    ProjectSymbol,
    GridReference,
    GridSection,
    KeyframeSelector,
    AnimationOption,
}

impl CompletionCategory {
    pub fn label(self) -> &'static str {
        match self {
            CompletionCategory::Snippet => "Snippet",
            CompletionCategory::Declaration => "Declaration",
            CompletionCategory::Include => "Include",
            CompletionCategory::LayoutProperty => "Layout",
            CompletionCategory::VisualProperty => "Visual",
            CompletionCategory::MotionProperty => "Motion",
            CompletionCategory::TypographyProperty => "Typography",
            CompletionCategory::TokenProperty => "Token",
            CompletionCategory::AdvancedProperty => "Advanced",
            CompletionCategory::StateBlock => "State",
            CompletionCategory::Value => "Value",
            CompletionCategory::ProjectSymbol => "Project Symbol",
            CompletionCategory::GridReference => "Grid Reference",
            CompletionCategory::GridSection => "Grid Section",
            CompletionCategory::KeyframeSelector => "Keyframe Selector",
            CompletionCategory::AnimationOption => "Animation Option",
        }
    }

    pub fn sort_prefix(self) -> &'static str {
        match self {
            CompletionCategory::Snippet => "00",
            CompletionCategory::GridReference
            | CompletionCategory::GridSection
            | CompletionCategory::ProjectSymbol => "01",
            CompletionCategory::KeyframeSelector => "02",
            CompletionCategory::Declaration => "03",
            CompletionCategory::LayoutProperty => "04",
            CompletionCategory::VisualProperty => "05",
            CompletionCategory::MotionProperty | CompletionCategory::AnimationOption => "06",
            CompletionCategory::TypographyProperty => "07",
            CompletionCategory::StateBlock => "08",
            CompletionCategory::TokenProperty => "09",
            CompletionCategory::Value => "10",
            CompletionCategory::Include => "11",
            CompletionCategory::AdvancedProperty => "12",
        }
    }
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

use crate::completions::helpers::{
    category_for_detail, completion_documentation, property_category,
};
use crate::completions::types::{CompletionCategory, CompletionSuggestion, SnippetScope};
use frame_core::knowledge;
use std::path::PathBuf;

struct FrameSnippet {
    label: &'static str,
    body: &'static str,
    documentation: &'static str,
    scopes: &'static [SnippetScope],
}

const SNIPPETS: &[FrameSnippet] = &[
    FrameSnippet {
        label: "dashboard",
        body: "grid Dashboard {\n  columns sidebar content inspector\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  place sidebar\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  place content\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  place inspector\n  surface panel\n  padding medium\n}",
        documentation: "Creates a named dashboard grid with sidebar, content, and inspector areas.\n\nFrame:\n\n```frame\ngrid Dashboard {\n  columns sidebar content inspector\n}\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "dashboard-percent",
        body: "grid Dashboard {\n  columns 25% 50% 25%\n  gap medium\n  height screen\n}\n\narea Sidebar {\n  in Dashboard\n  col 1\n  surface panel\n  padding medium\n}\n\narea Content {\n  in Dashboard\n  col 2\n  surface main\n  padding large\n}\n\narea Inspector {\n  in Dashboard\n  col 3\n  surface panel\n  padding medium\n}",
        documentation: "Creates a dashboard grid with explicit percentage columns.\n\nFrame:\n\n```frame\ngrid Dashboard {\n  columns 25% 50% 25%\n}\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "hover-card",
        body: "card HoverCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n  text bright\n\n  hover {\n    lift small\n    glow accent\n    brighten subtle\n  }\n}",
        documentation: "Creates an interactive card with a gradient surface and hover effects.\n\nFrame:\n\n```frame\ncard HoverCard {\n  surface gradient dusk\n  hover { lift small }\n}\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "toolbar",
        body: "row Toolbar {\n  align center\n  justify between\n  gap small\n  padding medium\n  surface panel\n}",
        documentation: "Creates a horizontal toolbar layout.\n\nFrame:\n\n```frame\nrow Toolbar {\n  align center\n  justify between\n}\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "empty-state",
        body: "center EmptyState {\n  height screen\n  surface main\n  text muted\n}",
        documentation: "Creates a centered empty state.\n\nFrame:\n\n```frame\ncenter EmptyState {\n  height screen\n  text muted\n}\n```",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "keyframe-animation",
        body: "keyframes FloatIn {\n  from {\n    opacity 0\n    transform translateY(12px) scale(0.98)\n  }\n\n  to {\n    opacity 1\n    transform translateY(0) scale(1)\n  }\n}\n\ncard Panel {\n  animation FloatIn {\n    duration 240ms\n    ease smooth\n    fill both\n  }\n}",
        documentation: "Creates custom keyframes and applies them to a component with structured animation controls.\n\nCSS output includes `@keyframes frame-FloatIn` and an `animation` declaration on `.fr-Panel`.",
        scopes: &[SnippetScope::Root],
    },
    FrameSnippet {
        label: "responsive-breakpoint",
        body: "grid AppShell {\n  columns sidebar content inspector\n\n  below tablet {\n    columns content\n    rows sidebar content inspector\n  }\n}",
        documentation: "Creates a responsive grid override. `below tablet` emits a media query for tablet-and-smaller viewports.",
        scopes: &[SnippetScope::Root, SnippetScope::Grid],
    },
    FrameSnippet {
        label: "container-query",
        body: "grid Cards {\n  columns responsive cards\n\n  container narrow {\n    columns content\n  }\n}",
        documentation: "Creates a container-query override for a grid when its container becomes narrow.",
        scopes: &[SnippetScope::Root, SnippetScope::Grid],
    },
    FrameSnippet {
        label: "below tablet block",
        body: "below tablet {\n  columns content\n}",
        documentation: "Adds a viewport-responsive override for tablet-and-smaller layouts.\n\nCSS output: `@media (max-width: 1023px)`.",
        scopes: &[SnippetScope::Grid],
    },
    FrameSnippet {
        label: "container narrow block",
        body: "container narrow {\n  columns content\n}",
        documentation: "Adds a container query override for a narrow component container.\n\nCSS output: `@container (max-width: 42rem)`.",
        scopes: &[SnippetScope::Grid],
    },
    FrameSnippet {
        label: "hover state",
        body: "hover {\n  lift small\n  glow accent\n  brighten subtle\n}",
        documentation: "Adds common hover feedback for an interactive component.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "focus state",
        body: "focus {\n  ring accent\n}",
        documentation: "Adds an accessible focus-visible ring for keyboard navigation.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "animation block",
        body: "animation FloatIn {\n  duration 240ms\n  ease smooth\n  fill both\n}",
        documentation: "Applies custom keyframes with explicit timing and fill behavior.",
        scopes: &[SnippetScope::Component],
    },
    FrameSnippet {
        label: "hover effects",
        body: "lift small\nglow accent\nbrighten subtle",
        documentation: "Adds common state effects inside `hover`, `focus`, `active`, or `disabled` blocks.",
        scopes: &[SnippetScope::State],
    },
    FrameSnippet {
        label: "from/to keyframes",
        body: "from {\n  opacity 0\n  transform translateY(12px) scale(0.98)\n}\n\nto {\n  opacity 1\n  transform translateY(0) scale(1)\n}",
        documentation: "Adds starting and ending animation states inside a `keyframes` declaration.",
        scopes: &[SnippetScope::Keyframes],
    },
    FrameSnippet {
        label: "50% keyframe",
        body: "50% {\n  opacity 0.72\n}",
        documentation: "Adds an intermediate keyframe selector.",
        scopes: &[SnippetScope::Keyframes],
    },
    FrameSnippet {
        label: "full animation controls",
        body: "duration 240ms\ndelay 0ms\nease smooth\niteration 1\ndirection normal\nfill both\nplay-state running",
        documentation: "Adds every supported structured animation control.",
        scopes: &[SnippetScope::Animation],
    },
];

pub(crate) fn suggestions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    suggestions_with_category(labels, detail, documentation, category_for_detail(detail))
}

pub(crate) fn suggestions_with_category(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
    category: CompletionCategory,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| CompletionSuggestion {
            label: (*label).to_string(),
            detail,
            documentation: completion_documentation(label)
                .or_else(|| knowledge::completion_doc(label))
                .unwrap_or_else(|| documentation.to_string()),
            insert_text: None,
            is_snippet: false,
            category,
        })
        .collect()
}

pub(crate) fn property_suggestions(
    labels: &[&str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| {
            let category = property_category(label);
            CompletionSuggestion {
                label: (*label).to_string(),
                detail,
                documentation: completion_documentation(label)
                    .or_else(|| knowledge::completion_doc(label))
                    .unwrap_or_else(|| documentation.to_string()),
                insert_text: None,
                is_snippet: false,
                category,
            }
        })
        .collect()
}

pub(crate) fn snippet_suggestions(scope: SnippetScope) -> Vec<CompletionSuggestion> {
    SNIPPETS
        .iter()
        .filter(|snippet| snippet.scopes.contains(&scope))
        .map(|snippet| CompletionSuggestion {
            label: snippet.label.to_string(),
            detail: "Frame snippet",
            documentation: snippet.documentation.to_string(),
            insert_text: Some(snippet.body.to_string()),
            is_snippet: true,
            category: CompletionCategory::Snippet,
        })
        .collect()
}

pub(crate) fn dynamic_suggestions(
    mut labels: Vec<String>,
    detail: &'static str,
    documentation: &'static str,
    category: CompletionCategory,
) -> Vec<CompletionSuggestion> {
    labels.sort();
    labels
        .into_iter()
        .map(|label| CompletionSuggestion {
            label,
            detail,
            documentation: documentation.to_string(),
            insert_text: None,
            is_snippet: false,
            category,
        })
        .collect()
}

pub(crate) fn motion_amount_suggestions(
    amounts: &[&str],
    detail: &'static str,
) -> Vec<CompletionSuggestion> {
    let mut items = suggestions(
        amounts,
        detail,
        "Intent amount. Add `%0` through `%100` to tune toward the next stronger amount.",
    );
    if let Some(base) = amounts.get(1) {
        items.push(CompletionSuggestion {
            label: format!("{base}%44"),
            detail,
            documentation:
                "Tuned amount: interpolates from this named amount toward the next stronger amount."
                    .to_string(),
            insert_text: None,
            is_snippet: false,
            category: CompletionCategory::MotionProperty,
        });
    }
    if let Some(strongest) = amounts.last() {
        items.push(CompletionSuggestion {
            label: format!("{strongest}%50"),
            detail,
            documentation: "Tuned strongest amount: extrapolates beyond the strongest preset by the previous step distance.".to_string(),
            insert_text: None,
            is_snippet: false,
            category: CompletionCategory::MotionProperty,
        });
    }
    items
}

pub(crate) fn supports_predicate_completions(line_words: &[String]) -> Vec<CompletionSuggestion> {
    match line_words.get(1).map(String::as_str) {
        None => suggestions_with_category(
            &[
                "display",
                "backdrop",
                "color",
                "selector",
                "container",
                "subgrid",
            ],
            "support predicate",
            "Typed feature query category.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("display") => suggestions_with_category(
            &["grid", "flex"],
            "support value",
            "Display feature query value.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("backdrop") => suggestions_with_category(
            &["blur"],
            "support value",
            "Backdrops feature query value.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("color") => suggestions_with_category(
            &["oklch"],
            "support value",
            "Color-space feature query value.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("selector") => suggestions_with_category(
            &["has"],
            "support value",
            "Selector feature query value.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("container") => suggestions_with_category(
            &["queries"],
            "support value",
            "Container-query feature query value.",
            CompletionCategory::AdvancedProperty,
        ),
        Some("subgrid") => Vec::new(),
        Some(_) => Vec::new(),
    }
}

pub(crate) fn include_suggestions(mut include_files: Vec<PathBuf>) -> Vec<CompletionSuggestion> {
    include_files.sort();
    include_files.dedup();
    include_files
        .into_iter()
        .filter_map(|path| {
            let label = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(ToOwned::to_owned)?;
            Some(CompletionSuggestion {
                label,
                detail: "Frame include",
                documentation: include_documentation(),
                insert_text: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(ToOwned::to_owned),
                is_snippet: false,
                category: CompletionCategory::Include,
            })
        })
        .collect()
}

pub(crate) fn include_documentation() -> String {
    "Includes another Frame file before the current declarations.\n\nWhere: root of a `.frame` file.\n\nFrame:\n\n```frame\n#include tokens\n#include ./cards.frame\n\ncard LocalCard {\n  surface panel\n  padding medium\n}\n```\n\nSvelte:\n\n```svelte\n<script lang=\"ts\">\n  import { ui } from '$lib/frame/generated';\n  import '$lib/frame/generated.css';\n</script>\n```\n\nRelated: `tokens`, `grid`, `area`\n\nDocs: `docs/imports.md`".to_string()
}

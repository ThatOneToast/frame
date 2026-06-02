use frame_core::tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionSuggestion {
    pub label: &'static str,
    pub detail: &'static str,
    pub documentation: &'static str,
}

const DECLARATIONS: &[&str] = &[
    "tokens", "grid", "area", "card", "stack", "row", "button", "text", "center", "split",
    "overlay", "dock",
];

const PROPERTIES: &[&str] = &[
    "columns",
    "rows",
    "gap",
    "place",
    "in",
    "col",
    "row",
    "span",
    "layout",
    "position",
    "align",
    "justify",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "surface",
    "theme",
    "text",
    "background",
    "padding",
    "margin",
    "radius",
    "border",
    "shadow",
    "font",
    "size",
    "weight",
    "line",
    "letter",
];

const GRID_VALUES: &[&str] = &[
    "responsive",
    "cards",
    "compact",
    "comfortable",
    "sidebar",
    "content",
    "inspector",
    "header",
    "footer",
    "main",
    "auto",
    "fill",
    "screen",
];

const STATES: &[&str] = &["hover", "focus", "active", "disabled"];
const RESPONSIVE: &[&str] = &[
    "mobile", "tablet", "desktop", "wide", "stack", "hide", "show", "only",
];
const TYPOGRAPHY: &[&str] = &[
    "heading", "body", "caption", "mono", "bold", "semibold", "normal", "thin",
];

pub fn completions_at(source: &str, offset: usize) -> Vec<CompletionSuggestion> {
    let line = line_prefix_at(source, offset);
    let words = line.split_whitespace().collect::<Vec<_>>();

    if in_state_block(source, offset) {
        return suggestions(
            tokens::EFFECTS,
            "effect",
            "Effect used inside an interaction state.",
        );
    }

    if words.len() <= 1 && line.trim_start() == line {
        return suggestions(DECLARATIONS, "declaration", "Starts a Frame declaration.");
    }

    if let Some(keyword) = words.first().copied() {
        if words.len() >= 2 || line.ends_with(' ') {
            return value_completions(keyword);
        }
    }

    if inside_declaration(source, offset) {
        let mut items = suggestions(
            PROPERTIES,
            "property",
            "Adds design intent to this declaration.",
        );
        items.extend(suggestions(
            STATES,
            "state",
            "Adds interaction-specific effects.",
        ));
        items.extend(suggestions(
            RESPONSIVE,
            "responsive",
            "Adds viewport-specific intent.",
        ));
        return items;
    }

    suggestions(DECLARATIONS, "declaration", "Starts a Frame declaration.")
}

fn value_completions(keyword: &str) -> Vec<CompletionSuggestion> {
    match keyword {
        "columns" | "rows" | "place" | "col" | "row" | "span" => {
            suggestions(GRID_VALUES, "grid value", "Grid placement or sizing value.")
        }
        "surface" | "background" => suggestions(
            &[
                "panel", "main", "glass", "flat", "raised", "gradient", "dusk", "midnight",
                "aurora",
            ],
            "surface value",
            "Named visual surface or gradient.",
        ),
        "padding" | "margin" | "gap" | "offset" => {
            suggestions(tokens::SPACING, "space value", "Named spacing token.")
        }
        "radius" => suggestions(tokens::RADII, "radius value", "Named corner radius token."),
        "shadow" => suggestions(tokens::SHADOWS, "shadow value", "Named shadow depth."),
        "width" | "height" | "min-width" | "max-width" | "min-height" | "max-height" => {
            suggestions(tokens::SIZES, "size value", "Named sizing intent.")
        }
        "theme" | "text" | "color" => {
            suggestions(tokens::COLORS, "color value", "Named color intent.")
        }
        "align" => suggestions(tokens::ALIGN, "alignment value", "Cross-axis alignment."),
        "justify" => suggestions(
            tokens::JUSTIFY,
            "justification value",
            "Main-axis distribution.",
        ),
        "position" => suggestions(tokens::POSITIONS, "position value", "Positioning intent."),
        "border" => suggestions(
            &["none", "soft", "accent", "danger", "success", "width"],
            "border value",
            "Named border style.",
        ),
        "font" | "size" | "weight" | "line" | "letter" => {
            suggestions(TYPOGRAPHY, "type value", "Typography token.")
        }
        _ => Vec::new(),
    }
}

fn suggestions(
    labels: &'static [&'static str],
    detail: &'static str,
    documentation: &'static str,
) -> Vec<CompletionSuggestion> {
    labels
        .iter()
        .map(|label| CompletionSuggestion {
            label,
            detail,
            documentation,
        })
        .collect()
}

fn line_prefix_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    &source[start..safe_offset]
}

fn inside_declaration(source: &str, offset: usize) -> bool {
    block_depth(source, offset) > 0
}

fn in_state_block(source: &str, offset: usize) -> bool {
    let safe_offset = offset.min(source.len());
    let before = &source[..safe_offset];
    let last_open = before.rfind('{');
    let last_close = before.rfind('}');

    if last_open.is_none() || last_close > last_open {
        return false;
    }

    let header_start = before[..last_open.unwrap()]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let name = before[header_start..last_open.unwrap()].trim();
    STATES.contains(&name)
}

fn block_depth(source: &str, offset: usize) -> usize {
    source[..offset.min(source.len())]
        .chars()
        .fold(0usize, |depth, character| match character {
            '{' => depth + 1,
            '}' => depth.saturating_sub(1),
            _ => depth,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_declarations_at_file_start() {
        let labels = completions_at("", 0)
            .into_iter()
            .map(|item| item.label)
            .collect::<Vec<_>>();

        assert!(labels.contains(&"grid"));
        assert!(labels.contains(&"card"));
    }

    #[test]
    fn suggests_properties_inside_declaration() {
        let source = "card ProjectCard {\n  ";
        let labels = completions_at(source, source.len())
            .into_iter()
            .map(|item| item.label)
            .collect::<Vec<_>>();

        assert!(labels.contains(&"surface"));
        assert!(labels.contains(&"hover"));
    }

    #[test]
    fn suggests_effects_inside_state_block() {
        let source = "card ProjectCard {\n  hover {\n    ";
        let labels = completions_at(source, source.len())
            .into_iter()
            .map(|item| item.label)
            .collect::<Vec<_>>();

        assert!(labels.contains(&"lift"));
        assert!(labels.contains(&"glow"));
    }

    #[test]
    fn suggests_values_after_property() {
        let source = "card ProjectCard {\n  surface ";
        let labels = completions_at(source, source.len())
            .into_iter()
            .map(|item| item.label)
            .collect::<Vec<_>>();

        assert!(labels.contains(&"panel"));
        assert!(labels.contains(&"gradient"));
    }
}

pub fn hover_doc(word: &str) -> Option<&'static str> {
    Some(match word {
        "tokens" => "Defines reusable design tokens for a Frame file.\nUse tokens to name shared visual decisions before applying them to components.",
        "grid" => "Defines a layout container using Frame's grid system.\nUse `columns`, `rows`, `gap`, and child `area` declarations to place content.\n\nExample:\n\ngrid AppShell {\n  columns sidebar content inspector\n  gap medium\n}",
        "area" => "Defines a child region inside a named grid.\nUse `in` to reference the parent grid and `place` to claim a named grid column or area.\n\nExample:\n\narea Sidebar {\n  in AppShell\n  place sidebar\n}",
        "card" => "Defines a reusable content surface.\nCards commonly combine surface, padding, radius, shadow, and hover effects.\n\nExample:\n\ncard ProjectCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n}",
        "stack" => "Defines a vertical layout group.\nUse `gap` and `align` to control spacing and cross-axis alignment.",
        "row" => "Defines a horizontal layout group.\nUse `gap`, `align`, and `justify` for toolbar-like arrangements.",
        "button" => "Defines an interactive control surface.\nUse surface, padding, radius, focus, active, and disabled states.",
        "text" => "Defines reusable typography intent.\nUse size, weight, font, and color tokens instead of raw font CSS.",
        "center" => "Defines a container that centers its content.\nUse it for empty states, loading states, and focused prompts.",
        "split" => "Defines a two-region layout.\nUse first, second, direction, and gap to describe the relationship between panes.",
        "overlay" => "Defines a layer above the page.\nUse position, backdrop, blur, and z values for modal-like UI.",
        "dock" => "Defines an anchored interface region.\nUse it for app bars, command bars, and persistent controls.",
        "columns" => "Names grid columns or requests a responsive card grid.\nExample: `columns sidebar content inspector` or `columns responsive cards`.",
        "rows" => "Names grid rows for layout intent.\nRows pair with child placement and readable generated CSS.",
        "gap" => "Sets spacing between children using Frame spacing tokens like small, medium, and large.",
        "place" => "Claims a named grid slot from the parent grid.\nUse inside an `area` declaration.",
        "in" => "References the parent grid for an area.\nExample: `in AppShell`.",
        "surface" => "Sets the visual surface of a component.\nUse named surfaces like `panel`, `main`, `glass`, or gradients like `gradient dusk`.\n\nExample:\n\nsurface gradient dusk",
        "theme" => "Applies semantic color intent such as danger, success, or warning.",
        "background" => "Sets background intent using Frame surface or color tokens.",
        "gradient" => "Selects a named gradient surface such as dusk, midnight, or aurora.",
        "padding" => "Adds inner spacing using Frame spacing tokens.",
        "margin" => "Adds outer spacing using Frame spacing tokens.",
        "radius" => "Sets corner shape with named values like small, large, pill, or none.",
        "border" => "Sets border intent with named styles like soft, accent, danger, or none.",
        "shadow" => "Sets depth using named shadow values like soft, medium, or deep.",
        "height" => "Sets height intent with values such as screen, fill, content, or spacing sizes.",
        "width" => "Sets width intent with values such as fill, content, screen, or sidebar.",
        "hover" => "Defines effects applied when the user hovers this component.\n\nExample:\n\nhover {\n  lift small\n  glow accent\n}",
        "focus" => "Defines effects applied when keyboard or assistive focus reaches this component.",
        "active" => "Defines effects applied while this component is being pressed.",
        "disabled" => "Defines visual treatment for unavailable controls.",
        "lift" => "Moves a component upward to express hover elevation.",
        "glow" => "Adds a semantic glow, commonly using accent, danger, or success.",
        "brighten" => "Slightly increases visual brightness for interactive feedback.",
        "dim" => "Reduces visual emphasis for disabled or inactive states.",
        "blur" => "Applies blur intent, usually for overlays or state effects.",
        "press" => "Adds a pressed movement for active controls.",
        "ring" => "Adds an accessible focus ring using a semantic color.",
        "smooth" => "Expresses smooth transition intent for interaction effects.",
        "responsive" => "Requests viewport-aware behavior, such as responsive card grids.",
        "cards" => "Used with `columns responsive` to create an auto-fitting card grid.",
        "screen" => "Sizes an element to the viewport in the relevant axis.",
        "fill" => "Sizes an element to fill available space.",
        "panel" => "A contained surface for sidebars, cards, and utility regions.",
        "main" => "The primary page surface.",
        "glass" => "A translucent elevated surface for overlays and panels.",
        "danger" => "Semantic color intent for destructive or error states.",
        "success" => "Semantic color intent for successful or positive states.",
        "warning" => "Semantic color intent for cautionary states.",
        "accent" => "Semantic color intent for primary emphasis.",
        "muted" => "Semantic color intent for secondary text or subdued UI.",
        "font" => "Selects a typography family intent such as mono.",
        "size" => "Selects a typography size intent such as heading, body, or caption.",
        "weight" => "Selects type emphasis such as normal, semibold, or bold.",
        _ => return None,
    })
}

pub fn word_at(source: &str, offset: usize) -> Option<&str> {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind(|character: char| !is_word_character(character))
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find(|character: char| !is_word_character(character))
        .map_or(source.len(), |index| safe_offset + index);

    if start == end {
        None
    } else {
        Some(&source[start..end])
    }
}

fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '-'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hover_docs_for_concepts() {
        let doc = hover_doc("grid").expect("grid should have docs");

        assert!(doc.contains("layout container"));
        assert!(doc.contains("columns"));
    }

    #[test]
    fn finds_word_at_offset() {
        let source = "card ProjectCard {\n  surface panel\n}\n";
        let offset = source.find("surface").unwrap() + 2;

        assert_eq!(word_at(source, offset), Some("surface"));
    }
}

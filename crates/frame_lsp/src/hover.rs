use frame_core::{knowledge, symbols::SymbolIndex};

#[allow(dead_code)]
pub fn hover_doc_at(source: &str, offset: usize) -> Option<String> {
    hover_doc_at_with_symbols(source, offset, None)
}

pub fn hover_doc_at_with_symbols(
    source: &str,
    offset: usize,
    symbols: Option<&SymbolIndex>,
) -> Option<String> {
    let word = word_at(source, offset)?;
    let line = line_at(source, offset);
    let words = line.split_whitespace().collect::<Vec<_>>();

    if line.starts_with("#include") {
        return Some(INCLUDE_DOC.to_string());
    }

    match words.as_slice() {
        ["surface", "panel"] if word == "panel" || word == "surface" => {
            return Some(doc_for("surface panel", SURFACE_PANEL_DOC));
        }
        ["surface", "main"] if word == "main" || word == "surface" => {
            return Some(doc_for("surface main", SURFACE_MAIN_DOC));
        }
        ["surface", "glass"] if word == "glass" || word == "surface" => {
            return Some(doc_for("surface glass", SURFACE_GLASS_DOC));
        }
        ["surface", "gradient", ..] if word == "gradient" => {
            return Some(SURFACE_GRADIENT_DOC.to_string())
        }
        ["width" | "height", value] if value.ends_with('%') => {
            return Some(doc_for("width 25%", WIDTH_PERCENT_DOC))
        }
        _ => {}
    }

    if let Some(symbols) = symbols {
        if let Some(color) = symbols.colors.get(word) {
            return Some(format!(
                "## `{}`\n\nCustom color token.\n\nValue:\n\n```css\n{}\n```\n\nUse it anywhere Frame accepts color intent, including `background`, `color`, `border`, `glow`, and `ring`.\n\n### Frame\n\n```frame\ncard BrandCard {{\n  background {}\n  color {}\n}}\n```",
                color.name,
                color.value.as_deref().unwrap_or("custom color"),
                color.name,
                color.name
            ));
        }

        if let Some(gradient) = symbols.gradients.get(word) {
            return Some(format!(
                "## `{}`\n\nCustom gradient token.\n\nGenerated behavior:\n\n```css\n{}\n```\n\nUse it for hero cards, highlighted dashboard cards, panels, and sign-in screens.\n\n### Frame\n\n```frame\ncard HeroCard {{\n  background {}\n  color white\n}}\n```",
                gradient.name,
                gradient.value.as_deref().unwrap_or("linear-gradient(...)"),
                gradient.name
            ));
        }
    }

    hover_doc(word)
}

pub fn hover_doc(word: &str) -> Option<String> {
    if let Some(doc) = knowledge::completion_doc(word) {
        return Some(doc);
    }

    Some(match word {
        "tokens" => "Defines reusable design tokens for a Frame file.\nUse tokens to name shared visual decisions before applying them to components.",
        "grid" => GRID_DOC,
        "area" => "Defines a child region inside a named grid.\nUse `in` to reference the parent grid and `place` to claim a named grid column or area.\n\nExample:\n\narea Sidebar {\n  in AppShell\n  place sidebar\n}",
        "card" => "Defines a reusable content surface.\nCards commonly combine surface, padding, radius, shadow, and hover effects.\n\nExample:\n\ncard ProjectCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n}",
        "stack" => "Defines a vertical layout group.\nUse `gap` and `align` to control spacing and cross-axis alignment.",
        "row" => ROW_DOC,
        "button" => "Defines an interactive control surface.\nUse surface, padding, radius, focus, active, and disabled states.",
        "text" => "Defines reusable typography intent.\nUse size, weight, font, and color tokens instead of raw font CSS.",
        "center" => "Defines a container that centers its content.\nUse it for empty states, loading states, and focused prompts.",
        "split" => SPLIT_DOC,
        "overlay" => OVERLAY_DOC,
        "dock" => DOCK_DOC,
        "keyframes" => KEYFRAMES_DOC,
        "columns" => COLUMNS_DOC,
        "rows" => ROWS_DOC,
        "gap" => "Sets spacing between children using Frame spacing tokens like small, medium, and large.",
        "place" => PLACE_DOC,
        "in" => "References the parent grid for an area.\nExample: `in AppShell`.",
        "col" => COL_DOC,
        "span" => "Makes an area span multiple grid tracks.\nUse it for headers, footers, or wide content regions.",
        "surface" => "Sets the visual surface of a component.\nUse named surfaces like `panel`, `main`, `glass`, or gradients like `gradient dusk`.\n\nExample:\n\nsurface gradient dusk",
        "theme" => "Applies semantic color intent such as danger, success, or warning.",
        "background" => "Sets background intent using Frame surface or color tokens.",
        "gradient" => "Selects a named gradient surface such as dusk, midnight, or aurora.",
        "padding" => "Adds inner spacing using Frame spacing tokens.",
        "margin" => "Adds outer spacing using Frame spacing tokens.",
        "radius" => "Sets corner shape with named values like small, large, pill, or none.",
        "border" => "Sets border intent with named styles like soft, accent, danger, or none.",
        "shadow" => "Sets depth using named shadow values like soft, medium, or deep.",
        "transition" => TRANSITION_DOC,
        "duration" => "Sets motion duration intent. Use `fast`, `normal`, or `slow` with transitions and animations.",
        "ease" => "Sets easing intent. Use `linear`, `smooth`, `bounce`, or `sharp` to describe motion feel.",
        "animation" | "animate" => ANIMATION_DOC,
        "delay" => "Sets the delay before an animation starts. Use named timing or CSS time values like `120ms`.",
        "iteration" => "Sets animation repeat count. Use a number or `infinite`.",
        "direction" => "Sets animation playback direction such as `normal`, `reverse`, or `alternate`.",
        "play-state" => "Controls whether an animation is `running` or `paused`.",
        "below" => RESPONSIVE_DOC,
        "above" => RESPONSIVE_DOC,
        "between" => RESPONSIVE_DOC,
        "container" => CONTAINER_DOC,
        "from" | "to" => KEYFRAME_SELECTOR_DOC,
        "opacity" => "Animates opacity in keyframes.\n\nGenerated CSS writes `opacity: ...`.",
        "transform" => "Animates transform functions in keyframes.\n\nGenerated CSS writes `transform: ...`.",
        "height" => "Sets height intent with values such as screen, fill, content, or percentages.\nGenerated CSS writes `height`, with `screen` becoming `100vh`.",
        "width" => "Sets width intent with values such as fill, content, screen, sidebar, or percentages.\nGenerated CSS writes `width`.",
        "min-width" => "Sets minimum width intent using named sizes or percentages.",
        "max-width" => "Sets maximum width intent using named sizes or percentages.",
        "min-height" => "Sets minimum height intent using named sizes or percentages.",
        "max-height" => "Sets maximum height intent using named sizes or percentages.",
        "align" => ALIGN_DOC,
        "justify" => JUSTIFY_DOC,
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
        "fill" => "Sizes an element to fill available space. Inside an `animation` block, `fill` sets animation fill mode such as `both`.",
        "panel" => SURFACE_PANEL_DOC,
        "main" => SURFACE_MAIN_DOC,
        "glass" => SURFACE_GLASS_DOC,
        "danger" => "Semantic color intent for destructive actions, errors, and dangerous status.\nUse it for delete buttons, invalid states, and error badges.",
        "success" => "Semantic color intent for successful or positive states.\nUse it for completed tasks, saved states, and positive status.",
        "warning" => "Semantic color intent for cautionary states.\nUse it for warnings, pending work, and attention states.",
        "accent" => "Use accent for important interactive UI:\n- primary buttons\n- active nav items\n- focus rings\n- highlighted cards",
        "muted" => "Semantic color intent for secondary text or subdued UI.\nUse it for captions, helper text, and lower-priority metadata.",
        "primary" => "Primary color intent for the most important interactive elements and highlighted content.",
        "secondary" => "Secondary color intent for supporting actions and secondary emphasis.",
        "info" => "Informational color intent for neutral notices, tips, and status messages.",
        "font" => "Selects a typography family intent such as mono.",
        "size" => "Selects a typography size intent such as heading, body, or caption.",
        "weight" => "Selects type emphasis such as normal, semibold, or bold.",
        _ => return None,
    }.to_string())
}

fn doc_for(name: &str, fallback: &str) -> String {
    knowledge::completion_doc(name).unwrap_or_else(|| fallback.to_string())
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
    character.is_ascii_alphanumeric() || character == '-' || character == '%'
}

fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let end = source[safe_offset..]
        .find('\n')
        .map_or(source.len(), |index| safe_offset + index);

    source[start..end].trim()
}

const SURFACE_PANEL_DOC: &str = r#"surface panel

A panel surface is for secondary UI areas like sidebars, inspectors, cards, and tool panels.
It usually uses a slightly raised or separated background color.

Generated CSS: `background: var(--frame-surface-panel);`

Use it for:
- sidebars
- right panels
- cards
- menu surfaces

Svelte example:

<aside class="fr-Sidebar">
  Channels
</aside>

<style lang="frame">
  area Sidebar {
    in Dashboard
    place sidebar
    surface panel
    padding medium
  }
</style>"#;

const SURFACE_MAIN_DOC: &str = r#"surface main

The main surface is for the primary page/content background.
Use it for the main content region, large pages, and app shells.

Generated CSS: `background: var(--frame-surface-main);`

Svelte example:

<main class="fr-Content">
  Main content
</main>

<style lang="frame">
  area Content {
    in Dashboard
    place content
    surface main
    padding large
  }
</style>"#;

const SURFACE_GLASS_DOC: &str = "surface glass\n\nA translucent surface for overlays, floating panels, and command palettes.\nGenerated CSS uses `background: var(--frame-surface-glass);`.";
const SURFACE_GRADIENT_DOC: &str = "surface gradient\n\nApplies a named Frame gradient such as `dusk`, `midnight`, or `aurora`.\nUse gradients for feature cards, callouts, and interactive surfaces that need extra emphasis.";
const WIDTH_PERCENT_DOC: &str = "width 25%\n\nMakes this item take a percentage of the available width.\nUseful for sidebars and split layouts.\nGenerated CSS writes values like `width: 25%;` or `height: 50%;`.";

const INCLUDE_DOC: &str = r#"#include

Includes another Frame file before the current declarations.

Use it to split large style systems into focused files such as `tokens.frame`, `layout.frame`, and `cards.frame`.

Frame:

#include tokens
#include ./styles/cards.frame

card LocalCard {
  surface panel
  padding medium
}

CLI:

frame compile src/lib/frame/app.frame --out src/lib/frame --include src/lib/frame

Docs: `docs/imports.md`"#;

const TRANSITION_DOC: &str = r#"transition

Sets named transition intent for interactive changes.

Use `transition smooth` on a component or inside `hover`, `focus`, and `active` blocks.

Frame:

card HoverCard {
  transition smooth

  hover {
    lift small
    glow accent
    transition fast
  }
}

Generated CSS writes predictable transition timing such as `all 200ms ease`.

Docs: `docs/animations.md`"#;

const ANIMATION_DOC: &str = r#"animation

Applies a named entrance or emphasis animation.

Common values: `fade-in`, `slide-up`, `pop-in`, `pulse`, and `none`.

Frame:

card Notice {
  surface panel
  animation pop-in
}

Generated CSS uses deterministic keyframes such as `frame-pop-in`.

Docs: `docs/animations.md`"#;

const KEYFRAMES_DOC: &str = r#"keyframes

Defines reusable animation keyframes in Frame's structured syntax.

Use `from`, `to`, and percentage selector blocks to describe animation states. Inside selectors, use animatable properties such as `opacity`, `transform`, and `filter`.

Frame:

keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px) scale(0.98)
  }

  to {
    opacity 1
    transform translateY(0) scale(1)
  }
}

Generated CSS:

@keyframes frame-FloatIn { ... }

Related: `animation`, `duration`, `ease`, `fill`

Docs: `docs/animations.md`"#;

const KEYFRAME_SELECTOR_DOC: &str = r#"keyframe selector

Marks a point in an animation timeline.

Use `from` for the initial state, `to` for the final state, and percentages like `50%` for intermediate states.

Generated CSS keeps the selector inside `@keyframes frame-Name`."#;

const RESPONSIVE_DOC: &str = r#"responsive block

Overrides declaration rules at viewport breakpoints.

Use `below tablet`, `above desktop`, or `between tablet desktop` inside a declaration when layout should change with viewport size.

Frame:

grid AppShell {
  columns sidebar content inspector

  below tablet {
    columns content
    rows sidebar content inspector
  }
}

Generated CSS emits an `@media` rule for the same generated class."#;

const CONTAINER_DOC: &str = r#"container

Overrides declaration rules based on container size instead of viewport size.

Use `container narrow` when a component should adapt to the space it receives.

Frame:

grid Cards {
  columns responsive cards

  container narrow {
    columns content
  }
}

Generated CSS emits an `@container` rule."#;

const GRID_DOC: &str = r#"grid

Defines a layout container using Frame's grid system.
Use `columns`, `rows`, `gap`, and child `area` declarations to place content.

Generated CSS: `display: grid` plus grid-template properties.

Svelte example:

<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Chat</main>
  <section class="fr-Inspector">Details</section>
</div>

<style lang="frame">
  grid Dashboard {
    columns 25% 50% 25%
    gap medium
    height screen
  }

  area Sidebar {
    in Dashboard
    col 1
    surface panel
    padding medium
  }

  area Content {
    in Dashboard
    col 2
    surface main
    padding large
  }

  area Inspector {
    in Dashboard
    col 3
    surface panel
    padding medium
  }
</style>"#;

const ROW_DOC: &str = r#"row

Defines a horizontal layout group.
Use it for NavBars, toolbars, button groups, and header rows.

Generated CSS: `display: flex; flex-direction: row;`

NavBar example:

row NavBar {
  align center
  justify between
  gap medium
  padding medium
  surface panel
}

button NavAction {
  surface flat
  text accent
  padding small
  radius pill
}

Svelte:

<nav class="fr-NavBar">
  <a class="fr-NavAction">Home</a>
  <a class="fr-NavAction">Docs</a>
</nav>"#;

const SPLIT_DOC: &str = r#"split

Defines a two-region layout.
Use it for sidebar/content, editor/preview, or master/detail views.

Generated CSS currently creates a grid with an auto column and a fill column.
For precise horizontal ratios, use `grid` with percentage `columns`.

Example:

grid Workspace {
  columns 33% 67%
  gap medium
  height screen
}

area NavPane {
  in Workspace
  col 1
  surface panel
  padding medium
}

area ContentPane {
  in Workspace
  col 2
  surface main
  padding large
}"#;

const OVERLAY_DOC: &str = r#"overlay

Defines a fixed layer above the page.
Use it for modals, command palettes, popovers, and blocking dialogs.

Generated CSS: fixed positioning with full-page inset.

Example:

overlay ModalLayer {
  surface glass
  position center
  z modal
  padding large
}

card ModalCard {
  surface panel
  padding large
  radius large
  shadow deep
}"#;

const DOCK_DOC: &str = r#"dock

Defines an anchored interface region.
Use it for persistent app bars, bottom command bars, and docked controls.

Current generated CSS docks to the bottom of the viewport.
For a top NavBar, prefer `row NavBar` inside a page grid header area.

Top NavBar pattern:

grid AppShell {
  rows auto fill
  gap medium
  min-height screen
}

area Header {
  in AppShell
  row 1
  surface panel
}

row NavBar {
  align center
  justify between
  padding medium
  gap medium
}"#;

const COLUMNS_DOC: &str = r#"columns

Defines the horizontal sections of a grid.

Generated CSS:
- named columns become equal `minmax(0, 1fr)` tracks and named grid areas
- percentage columns become exact `grid-template-columns` percentages
- `responsive cards` becomes an auto-fitting card grid

Examples:

columns sidebar content inspector
columns 25% 50% 25%
columns responsive cards"#;

const ROWS_DOC: &str = r#"rows

Defines the vertical sections of a grid.
Use rows for NavBars, page headers, content bands, and footers.

Generated CSS creates `grid-template-rows`.

Example:

grid AppShell {
  rows auto fill auto
  gap medium
  min-height screen
}

area Header {
  in AppShell
  row 1
  surface panel
  padding medium
}

area Content {
  in AppShell
  row 2
  surface main
  padding large
}"#;

const PLACE_DOC: &str = r#"place

Claims a named grid slot from the parent grid.

grid Dashboard {
  columns sidebar content inspector
}

area Sidebar {
  in Dashboard
  place sidebar
}"#;

const COL_DOC: &str = r#"col

Places an area in a numeric grid column.
Use this when columns are percentages or explicit tracks.

grid Dashboard {
  columns 25% 50% 25%
}

area Sidebar {
  in Dashboard
  col 1
}"#;

const ALIGN_DOC: &str = r#"align

Controls vertical or cross-axis placement.
Generated CSS writes `align-items`.

row Toolbar {
  align center
  justify between
}"#;

const JUSTIFY_DOC: &str = r#"justify

Controls horizontal or main-axis placement and distribution.
Generated CSS writes `justify-content`.

row Toolbar {
  align center
  justify between
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hover_docs_for_concepts() {
        let doc = hover_doc("grid").expect("grid should have docs");

        assert!(doc.contains("layout container"));
        assert!(doc.contains("<style lang=\"frame\">"));
    }

    #[test]
    fn finds_word_at_offset() {
        let source = "card ProjectCard {\n  surface panel\n}\n";
        let offset = source.find("surface").unwrap() + 2;

        assert_eq!(word_at(source, offset), Some("surface"));
    }

    #[test]
    fn returns_surface_value_hover_docs() {
        let source = "area Sidebar {\n  surface panel\n}\n";
        let offset = source.find("panel").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("panel should have docs");

        assert!(doc.contains("Svelte example"));
        assert!(doc.contains("fr-Sidebar"));
    }

    #[test]
    fn returns_columns_and_alignment_hover_docs() {
        assert!(hover_doc("columns").unwrap().contains("25% 50% 25%"));
        assert!(hover_doc("align").unwrap().contains("cross-axis"));
        assert!(hover_doc("justify").unwrap().contains("main-axis"));
    }

    #[test]
    fn returns_percentage_hover_docs() {
        let source = "card A {\n  width 25%\n}\n";
        let offset = source.find("25%").unwrap() + 1;

        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("available width"));
    }
}

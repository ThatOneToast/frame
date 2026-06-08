use crate::completions::types::CompletionCategory;

pub(crate) fn is_inside_block(source: &str, offset: usize, block: &str) -> bool {
    let safe_offset = offset.min(source.len());
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..safe_offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
        .last()
        .is_some_and(|header| header == block || header.starts_with(&format!("{block} ")))
}

pub(crate) fn is_inside_ancestor_block(source: &str, offset: usize, block: &str) -> bool {
    let safe_offset = offset.min(source.len());
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..safe_offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
        .iter()
        .any(|header| header == block || header.starts_with(&format!("{block} ")))
}

pub(crate) fn is_inside_keyframe_selector(source: &str, offset: usize) -> bool {
    let safe_offset = offset.min(source.len());
    let mut stack = Vec::new();
    let mut line_start = 0usize;

    for (index, character) in source[..safe_offset].char_indices() {
        match character {
            '\n' => line_start = index + 1,
            '{' => {
                let header = source[line_start..index].trim();
                stack.push(header.to_string());
            }
            '}' => {
                stack.pop();
            }
            _ => {}
        }
    }

    stack
        .last()
        .is_some_and(|header| matches!(header.as_str(), "from" | "to") || header.ends_with('%'))
}

pub(crate) fn line_words_at(source: &str, offset: usize) -> Vec<String> {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);

    source[start..safe_offset]
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect()
}

pub(crate) fn line_at(source: &str, offset: usize) -> &str {
    let safe_offset = offset.min(source.len());
    let start = source[..safe_offset]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    &source[start..safe_offset]
}

pub(crate) fn completion_documentation(label: &str) -> Option<String> {
    if let Some(doc) = frame_core::language::hover_doc_for(label) {
        if !doc.is_empty() {
            return Some(doc);
        }
    }
    custom_completion_documentation(label)
}

fn custom_completion_documentation(label: &str) -> Option<String> {
    Some(match label {
        "tokens" => "Declares reusable design tokens for a Frame file.",
        "grid" => "Defines a layout container. Use `columns`, `rows`, `gap`, and child `area` declarations.",
        "area" => "Defines a child region inside a grid. Usually includes `in GridName` and `place name` or `col 1`.",
        "card" => "Defines a reusable content surface. Good for panels, links, tiles, and settings sections.",
        "screen" => "Defines a full interface surface. Renderers choose the platform container.",
        "stack" => "Arranges children in one ordered direction without exposing flexbox.",
        "row" => "As a declaration, creates a horizontal layout for NavBars and toolbars. As a property, places an area in a grid row.",
        "action" => "Represents a user-triggered command. Renderers lower it to their preferred accessible action control.",
        "link" => "Represents navigation intent. Use `goto` for the destination.",
        "editor" => "Represents multi-line text entry. Use `bind $state` for the edited value.",
        "toggle" => "Represents a binary setting. Use `bind $state` for checked state.",
        "choice" => "Represents choosing from a small set of options.",
        "composer" => "Represents message composition. Use `draft bind $state` and `send @handler`.",
        "menu" => "Represents navigation or command choices.",
        "toolbar" => "Represents a compact group of related actions.",
        "tabs" => "Represents switching between related panels.",
        "list" => "Represents repeated items from `source $items`.",
        "feed" => "Represents chronological or activity-stream content.",
        "data" => "Represents structured records without exposing table rows and cells.",
        "item" => "Defines the repeated item body inside `list`, `feed`, or `data`.",
        "empty" => "Defines fallback content when a list/feed/data source has no items.",
        "popover" => "Represents a lightweight contextual surface.",
        "badge" => "Represents compact status or metadata.",
        "avatar" => "Represents a person or entity image and requires alternate text unless decorative.",
        "icon" => "Represents symbolic visual content.",
        "image" => "Represents meaningful imagery and requires alternate text unless decorative.",
        "text" => "Defines visible text content or reusable typography intent.",
        "center" => "Centers content. Good for empty states and loading states.",
        "split" => "Defines a two-region layout. For exact horizontal ratios, use `grid` with percentage `columns`.",
        "overlay" => "Defines a fixed layer above the page. Use for modals, command palettes, and blocking dialogs.",
        "dock" => "Defines a docked command region. Current output docks to the bottom; use `row NavBar` for top navigation.",
        "keyframes" => "Defines reusable animation keyframes.\n\nExample:\n\nkeyframes FloatIn {\n  from {\n    opacity 0\n  }\n\n  to {\n    opacity 1\n  }\n}",
        "columns" => "Defines grid columns. Examples: `columns sidebar content`, `columns 25% 50% 25%`, `columns responsive cards`.",
        "rows" => "Defines grid rows. Examples: `rows header main footer` or `rows auto fill auto`.",
        "flow" => "Controls grid section direction. Use `flow vertical` to stack named `columns` as rows.",
        "section" => "Starts spacing and alignment controls for a named grid section.\n\nExample:\n\nsection title {\n  padding bottom small\n}",
        "gap" => "Sets spacing between children using `none`, `small`, `medium`, `large`, or `xlarge`.",
        "display" => "Sets display behavior with structured values like `block`, `flex`, `grid`, `contents`, or `none`.",
        "height" => "Sets height intent. Use `screen`, `fill`, `content`, `auto`, or percentages like `50%`.",
        "width" => "Sets width intent. Use `fill`, `content`, `sidebar`, `narrow`, `wide`, or percentages like `25%`.",
        "inline-size" => "Sets logical inline size. In horizontal writing modes this usually maps to width.",
        "block-size" => "Sets logical block size. In horizontal writing modes this usually maps to height.",
        "min-inline-size" => "Sets minimum logical inline size.",
        "max-inline-size" => "Sets maximum logical inline size.",
        "min-block-size" => "Sets minimum logical block size.",
        "max-block-size" => "Sets maximum logical block size.",
        "padding" => "Sets inner spacing using Frame spacing tokens.",
        "margin" => "Sets outer spacing using Frame spacing tokens.",
        "surface" => "Sets background surface intent: `panel`, `main`, `glass`, `raised`, `flat`, or `gradient ...`.",
        "align" => "Controls vertical or cross-axis placement: `start`, `center`, `end`, or `stretch`.",
        "justify" => "Controls horizontal or main-axis distribution: `start`, `center`, `end`, `between`, `around`, or `evenly`.",
        "in" => "References the parent grid for an `area`.",
        "place" => "Claims a named grid slot from the referenced grid.",
        "col" => "Places an area in a numeric grid column. Use with percentage grids.",
        "span" => "Makes an area span multiple grid tracks.",
        "radius" => "Sets corner shape using radius tokens like `large`, `pill`, or `none`.",
        "border" => "Sets border intent. Use `border accent`, `border width medium`, `border style dashed`, `border radius large`, or `border none`.",
        "outline" => "Sets outline intent. Use `outline accent`, `outline none`, or `outline offset small`.",
        "visibility" => "Sets visibility as `visible`, `hidden`, or `collapse`.",
        "flex" => "Sets flexbox controls. Use `flex direction row`, `flex wrap wrap`, `flex grow 1`, `flex shrink 0`, or `flex basis fill`.",
        "shadow" => "Sets depth using shadow tokens like `soft`, `medium`, or `deep`.",
        "color" => "Sets text color using semantic color tokens.",
        "background" => "Sets background with surface or semantic color tokens.",
        "advanced" => "Starts an explicit escape hatch block for scoped raw CSS declarations.\n\nExample:\n\nadvanced {\n  css \"backdrop-filter\" blur(12px)\n}",
        "gradient" => "Defines or selects a structured gradient token.\n\nToken example:\n\ngradient hero-gradient {\n  type linear\n  angle 135deg\n  stop brand-purple 0%\n  stop brand-panel 100%\n}",
        "below" => "Starts a responsive override for viewports below a breakpoint.\n\nExample:\n\nbelow tablet {\n  columns content\n}",
        "above" => "Starts a responsive override for viewports at or above a breakpoint.",
        "between" => "Starts a responsive override between two breakpoints.\n\nExample:\n\nbetween tablet desktop {\n  columns sidebar content\n}",
        "container" => "Starts a container query override.\n\nExample:\n\ncontainer narrow {\n  columns content\n}",
        "supports" => "Starts a typed feature query block.\n\nExample:\n\nsupports display grid {\n  grid AppShell {\n    columns sidebar content\n  }\n}\n\nGenerated CSS emits `@supports (display: grid)`.",
        "style-group" => "Starts a cascade layer group using Frame terminology.\n\nExample:\n\nstyle-group components {\n  button PrimaryButton {\n    surface accent\n  }\n}\n\nGenerated CSS emits `@layer components`.",
        "style-order" => "Declares deterministic style group order.\n\nExample:\n\nstyle-order reset, base, components, utilities\n\nGenerated CSS emits `@layer reset, base, components, utilities;`.",
        "theme" => "Applies semantic color intent to text and border.",
        "hover" => "Starts hover effects. Only effect keywords are valid inside.",
        "focus" => "Starts keyboard focus effects, usually `ring accent`.",
        "focus-visible" => "Starts keyboard-visible focus effects. Emits `:focus-visible`.",
        "focus-within" => "Starts effects when this element or a descendant has focus. Emits `:focus-within`.",
        "active" => "Starts pressed-state effects, usually `press`.",
        "disabled" => "Starts unavailable-state effects, usually `dim medium`.",
        "checked" => "Starts effects for checked controls. Emits `:checked`.",
        "invalid" => "Starts effects for invalid form controls. Emits `:invalid`.",
        "required" => "Starts effects for required form controls. Emits `:required`.",
        "target" => "Starts effects for URL fragment targets. Emits `:target`.",
        "responsive" => "Use with `columns responsive cards` for an auto-fitting card grid.",
        "cards" => "Completes `columns responsive cards`.",
        "auto" => "Automatic sizing.",
        "fill" => "Fill available space.",
        "sidebar" => "Common named grid slot or sidebar width token.",
        "content" => "Common content slot or max-content sizing token.",
        "main" => "Primary page/content surface or grid slot.",
        "inspector" => "Common right-side panel grid slot.",
        "header" => "Common top grid row or slot.",
        "footer" => "Common bottom grid row or slot.",
        "panel" => "Secondary surface for sidebars, inspectors, cards, menus, and tool panels.",
        "glass" => "Translucent elevated surface for overlays and floating panels.",
        "raised" => "Elevated solid surface for cards and controls.",
        "flat" => "Transparent or visually flat surface.",
        "gradient dusk" => "Gradient surface for highlighted cards.",
        "gradient midnight" => "Dark gradient surface for hero or feature cards.",
        "gradient aurora" => "Colorful gradient surface for high-emphasis cards.",
        "dusk" => "Gradient name used after `surface gradient`.",
        "midnight" => "Gradient name used after `surface gradient`.",
        "aurora" => "Gradient name used after `surface gradient`.",
        "small" => "Small spacing, radius, shadow, or effect strength.",
        "medium" => "Default spacing, radius, shadow, or effect strength.",
        "large" => "Large spacing, radius, shadow, or effect strength.",
        "xlarge" => "Extra large spacing or radius.",
        "none" => "Removes spacing, radius, border, or shadow depending on property.",
        "pill" => "Fully rounded pill radius.",
        "full" => "Fully rounded radius.",
        "bright" => "High-emphasis text color.",
        "muted" => "Secondary text color for captions, metadata, and helper text.",
        "accent" => "Interactive emphasis color for primary actions, links, and focus rings.",
        "primary" => "Primary semantic color.",
        "secondary" => "Secondary semantic color.",
        "danger" => "Destructive or error semantic color.",
        "success" => "Positive or completed semantic color.",
        "warning" => "Caution semantic color.",
        "info" => "Informational semantic color.",
        "lift" => "Moves a component upward. Use movement amounts `tiny`, `small`, `medium`, `large`, or `huge`; suffix with `%0` through `%100` to tune toward the next stronger amount.",
        "sink" => "Moves a component downward. Use movement amounts such as `small` or tuned values like `small%44`.",
        "shift" => "Moves a component sideways or vertically. Use `shift left small`, `shift right small`, `shift up small`, or `shift down small`.",
        "grow" => "Slightly scales a component up. Use visual amounts `slight`, `subtle`, `normal`, `strong`, or `dramatic`; tuned values like `slight%5` are allowed.",
        "shrink" => "Slightly scales a component down using visual amount tokens.",
        "tilt" => "Rotates a component by intent. Use `tilt left subtle` or `tilt right subtle`; tuned values like `subtle%23` are allowed.",
        "glow" => "Adds semantic glow, commonly `glow accent`.",
        "brighten" => "Increases brightness for interactive feedback.",
        "dim" => "Reduces emphasis, commonly for disabled state.",
        "blur" => "Applies blur effect.",
        "press" => "Adds a pressed movement for active controls.",
        "pop" => "Adds a small positive scale movement for appearing or selected states.",
        "ring" => "Adds an accessible focus ring.",
        "scale" => "Slightly scales an element.",
        "fade" => "Reduces opacity.",
        "slide" => "Expresses slide movement intent.",
        "from" => "Keyframe selector for the starting state. Generates `from { ... }` inside `@keyframes`.",
        "to" => "Keyframe selector for the ending state. Generates `to { ... }` inside `@keyframes`.",
        "0%" | "25%" | "50%" | "75%" | "100%" => "Percentage keyframe selector for intermediate animation states.",
        "delay" => "Sets how long an animation waits before starting.",
        "iteration" => "Sets how many times an animation repeats. Use a number or `infinite`.",
        "direction" => "Sets animation playback direction.",
        "play-state" => "Controls whether an animation is running or paused.",
        "opacity" => "Animates opacity in keyframes. Generates `opacity: ...`.",
        "transform" => "Animates transform functions in keyframes. Generates `transform: ...`.",
        "decoration" => "Sets text decoration line intent: `underline`, `overline`, `line-through`, or `none`.",
        "whitespace" => "Controls white-space preservation and wrapping, including `pre-wrap` and `break-spaces`.",
        "word-break" => "Controls how long words break in narrow layouts.",
        "hyphenate" => "Controls CSS hyphenation using `none`, `manual`, or `auto`.",
        _ => return None,
    }.to_string())
}

pub(crate) fn category_for_detail(detail: &str) -> CompletionCategory {
    match detail {
        "declaration" => CompletionCategory::Declaration,
        "include" => CompletionCategory::Include,
        "token property" | "gradient property" => CompletionCategory::TokenProperty,
        "advanced css" => CompletionCategory::AdvancedProperty,
        "animation option" => CompletionCategory::AnimationOption,
        "keyframe selector" => CompletionCategory::KeyframeSelector,
        detail
            if detail.contains("grid") || detail.contains("layout") || detail.contains("area") =>
        {
            CompletionCategory::LayoutProperty
        }
        detail
            if detail.contains("effect")
                || detail.contains("transition")
                || detail.contains("animation") =>
        {
            CompletionCategory::MotionProperty
        }
        _ => CompletionCategory::Value,
    }
}

pub(crate) fn layer_sort_prefix(
    layer: Option<frame_core::language::LanguageLayer>,
) -> &'static str {
    match layer {
        Some(frame_core::language::LanguageLayer::Ui) => "0",
        Some(frame_core::language::LanguageLayer::Advanced)
        | Some(frame_core::language::LanguageLayer::EscapeHatch) => "2",
        _ => "1",
    }
}

pub(crate) fn property_category(label: &str) -> CompletionCategory {
    if let Some(item) = frame_core::language::item(label) {
        // Use the registry category when it is meaningful for completions.
        if !matches!(
            item.completion_category,
            CompletionCategory::Snippet
                | CompletionCategory::Value
                | CompletionCategory::ProjectSymbol
                | CompletionCategory::GridReference
                | CompletionCategory::GridSection
                | CompletionCategory::KeyframeSelector
                | CompletionCategory::AnimationOption
        ) {
            return item.completion_category;
        }
    }
    match label {
        "columns" | "rows" | "tracks" | "areas" | "flow" | "section" | "layout" | "display"
        | "gap" | "height" | "width" | "min-height" | "max-height" | "min-width" | "max-width"
        | "inline-size" | "block-size" | "min-inline-size" | "max-inline-size"
        | "min-block-size" | "max-block-size" | "place" | "in" | "col" | "row" | "span"
        | "position" | "offset" | "z" | "align" | "justify" | "self" | "anchor" | "padding"
        | "margin" | "overflow" | "scroll" | "scrollbar" | "box" | "flex" | "square" | "nudge" => {
            CompletionCategory::LayoutProperty
        }
        "surface" | "background" | "theme" | "text" | "color" | "palette" | "tone" | "opacity"
        | "radius" | "border" | "shadow" | "outline" | "visibility" | "control" | "interactive" => {
            CompletionCategory::VisualProperty
        }
        "font" | "size" | "weight" | "line" | "letter" | "truncate" | "wrap" | "case"
        | "align-text" | "decoration" | "whitespace" | "word-break" | "hyphenate" => {
            CompletionCategory::TypographyProperty
        }
        "lift" | "sink" | "shift" | "grow" | "shrink" | "tilt" | "press" | "pop" | "glow"
        | "brighten" | "dim" | "blur" | "ring" | "smooth" | "fade" | "scale" | "rotate"
        | "slide" | "transition" | "duration" | "ease" | "animation" | "animate" | "delay"
        | "iteration" | "direction" | "fill" | "play-state" | "transform" | "filter" => {
            CompletionCategory::MotionProperty
        }
        "hover" | "focus" | "focus-visible" | "focus-within" | "active" | "disabled"
        | "checked" | "invalid" | "required" | "target" => CompletionCategory::StateBlock,
        "advanced" | "css" => CompletionCategory::AdvancedProperty,
        "type" | "angle" | "stop" | "corner" | "at" | "shape" | "gradient" => {
            CompletionCategory::TokenProperty
        }
        _ => CompletionCategory::Value,
    }
}

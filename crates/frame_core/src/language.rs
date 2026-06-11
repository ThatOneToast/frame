//! Canonical language registry for Frame.
//!
//! This module is the single source of truth for all language facts
//! previously scattered across tokens.rs, knowledge.rs, and frame_lsp.

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageItemKind {
    Primitive,
    Declaration,
    Property,
    Value,
    Event,
    EventModifier,
    StateKeyword,
    BindingKeyword,
    UiKeyword,
    IncludeKeyword,
    Effect,
    Special,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageLayer {
    Ui,
    Style,
    Motion,
    Layout,
    Typography,
    Advanced,
    EscapeHatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageItemStatus {
    Stable,
    Experimental,
    Deprecated,
    Advanced,
    EscapeHatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenClass {
    Keyword,
    Class,
    Property,
    EnumMember,
    Variable,
    Number,
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptKind {
    Declaration,
    Property,
    Value,
    State,
    Effect,
    Snippet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameScope {
    Root,
    Grid,
    Area,
    Component,
    Text,
    State,
    Tokens,
    Gradient,
    Animation,
    Keyframes,
    Responsive,
    Container,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameConcept {
    pub name: &'static str,
    pub kind: ConceptKind,
    pub summary: &'static str,
    pub description: &'static str,
    pub generated_css: Option<&'static str>,
    pub frame_examples: &'static [&'static str],
    pub svelte_examples: &'static [&'static str],
    pub allowed_in: &'static [FrameScope],
    pub related: &'static [&'static str],
    pub values: &'static [&'static str],
    pub docs_anchor: Option<&'static str>,
}

impl FrameConcept {
    pub fn markdown(&self) -> String {
        let mut output = format!("## `{}`\n\n{}\n", self.name, self.description);
        if !self.values.is_empty() {
            output.push_str("\n### Common Values\n\n");
            for value in self.values {
                output.push_str("- `");
                output.push_str(value);
                output.push_str("`\n");
            }
        }
        if let Some(generated_css) = self.generated_css {
            output.push_str("\n### Generated CSS\n\n");
            output.push_str(generated_css);
            output.push('\n');
        }
        if let Some(example) = self.frame_examples.first() {
            output.push_str("\n### Frame\n\n```frame\n");
            output.push_str(example);
            output.push_str("\n```\n");
        }
        if let Some(example) = self.svelte_examples.first() {
            output.push_str("\n### Svelte example\n\n```svelte\n");
            output.push_str(example);
            output.push_str("\n```\n");
        }
        if !self.related.is_empty() {
            output.push_str("\n### Related\n\n");
            for related in self.related {
                output.push_str("- `");
                output.push_str(related);
                output.push_str("`\n");
            }
        }
        if let Some(anchor) = self.docs_anchor {
            output.push_str("\nSee `");
            output.push_str(anchor);
            output.push_str("`.\n");
        }
        output
    }
}

#[derive(Clone)]
pub struct LanguageItem {
    pub name: &'static str,
    pub kind: LanguageItemKind,
    pub layer: LanguageLayer,
    pub detail: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub documentation: &'static str,
    pub generated_css: Option<&'static str>,
    pub frame_examples: &'static [&'static str],
    pub svelte_examples: &'static [&'static str],
    pub allowed_in: &'static [FrameScope],
    pub related: &'static [&'static str],
    pub values: &'static [&'static str],
    pub aliases: &'static [&'static str],
    pub lowers_to: Option<&'static str>,
    pub docs_anchor: Option<&'static str>,
    pub status: LanguageItemStatus,
    pub completion_category: CompletionCategory,
    pub semantic_class: SemanticTokenClass,
}

impl LanguageItem {
    pub fn markdown(&self) -> String {
        if !self.documentation.is_empty() {
            return self.documentation.to_string();
        }
        let mut has_content = false;
        let mut output = String::new();
        if !self.description.is_empty() {
            has_content = true;
            output.push_str(&format!("## `{}`\n\n{}\n", self.name, self.description));
        }
        if !self.values.is_empty() {
            has_content = true;
            output.push_str("\n### Common Values\n\n");
            for value in self.values {
                output.push_str("- `");
                output.push_str(value);
                output.push_str("`\n");
            }
        }
        if let Some(generated_css) = self.generated_css {
            has_content = true;
            output.push_str("\n### Generated CSS\n\n");
            output.push_str(generated_css);
            output.push('\n');
        }
        if let Some(example) = self.frame_examples.first() {
            has_content = true;
            output.push_str("\n### Frame\n\n```frame\n");
            output.push_str(example);
            output.push_str("\n```\n");
        }
        if let Some(example) = self.svelte_examples.first() {
            has_content = true;
            output.push_str("\n### Svelte example\n\n```svelte\n");
            output.push_str(example);
            output.push_str("\n```\n");
        }
        if !self.related.is_empty() {
            has_content = true;
            output.push_str("\n### Related\n\n");
            for related in self.related {
                output.push_str("- `");
                output.push_str(related);
                output.push_str("`\n");
            }
        }
        if let Some(anchor) = self.docs_anchor {
            has_content = true;
            output.push_str(&format!("\nSee `{}`.\n", anchor));
        }
        if has_content {
            output
        } else {
            String::new()
        }
    }
}

macro_rules! item {
    ($name:expr, $kind:ident, $layer:ident, $category:ident, $class:ident) => {
        LanguageItem {
            name: $name,
            kind: LanguageItemKind::$kind,
            layer: LanguageLayer::$layer,
            detail: $name,
            summary: $name,
            description: "",
            documentation: "",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::$category,
            semantic_class: SemanticTokenClass::$class,
        }
    };
}

pub const SPACING: &[&str] = &["none", "small", "medium", "large", "xlarge"];
pub const EDGES: &[&str] = &[
    "top", "right", "bottom", "left", "x", "y", "inline", "block",
];
pub const RADII: &[&str] = &["none", "small", "medium", "large", "xlarge", "pill", "full"];
pub const SURFACES: &[&str] = &[
    "panel", "main", "glass", "flat", "raised", "overlay", "inset", "sunken", "gradient",
];
pub const EFFECTS: &[&str] = &[
    "lift",
    "sink",
    "shift",
    "grow",
    "shrink",
    "tilt",
    "glow",
    "brighten",
    "dim",
    "press",
    "pop",
    "ring",
    "blur",
    "smooth",
    "fade",
    "scale",
    "rotate",
    "slide",
    "transition",
    "duration",
    "ease",
    "animation",
    "animate",
];
pub const MOVEMENT_AMOUNTS: &[&str] = &["tiny", "small", "medium", "large", "huge"];
pub const VISUAL_AMOUNTS: &[&str] = &["slight", "subtle", "normal", "strong", "dramatic"];
pub const SHADOWS: &[&str] = &[
    "none", "soft", "small", "medium", "large", "deep", "floating",
];
pub const GLOWS: &[&str] = &[
    "none", "accent", "danger", "success", "warning", "soft", "strong",
];
pub const SIZES: &[&str] = &[
    "screen", "fill", "content", "auto", "sidebar", "narrow", "wide", "chart", "panel", "small",
    "medium", "large", "xlarge", "zero", "modal", "icon",
];
pub const TRACKS: &[&str] = &[
    "rail", "panel", "side", "header", "composer", "fill", "auto", "content",
];
pub const LAYOUTS: &[&str] = &[
    "icon-content-action",
    "avatar-content",
    "header",
    "composer",
    "center",
];
pub const OVERFLOWS: &[&str] = &["hidden", "visible", "auto", "clip"];
pub const SCROLL_AXES: &[&str] = &["x", "y", "both"];
pub const SCROLLBARS: &[&str] = &["dense", "normal"];
pub const TEXT_WRAPS: &[&str] = &["anywhere", "normal"];
pub const TEXT_CASES: &[&str] = &["uppercase", "lowercase", "capitalize", "normal"];
pub const TEXT_ALIGN: &[&str] = &[
    "left",
    "center",
    "right",
    "start",
    "end",
    "justify",
    "match-parent",
];
pub const TEXT_DECORATIONS: &[&str] = &["none", "underline", "overline", "line-through"];
pub const WHITE_SPACE: &[&str] = &[
    "normal",
    "nowrap",
    "pre",
    "pre-wrap",
    "pre-line",
    "break-spaces",
];
pub const WORD_BREAKS: &[&str] = &["normal", "break-all", "keep-all", "break-word"];
pub const HYPHENS: &[&str] = &["none", "manual", "auto"];
pub const LINES: &[&str] = &["normal", "relaxed", "tight"];
pub const LETTERS: &[&str] = &["normal"];
pub const CONTROLS: &[&str] = &["reset"];
pub const BOX_SIZING: &[&str] = &["border", "content"];
pub const DISPLAY: &[&str] = &[
    "block",
    "inline",
    "inline-block",
    "flex",
    "inline-flex",
    "grid",
    "inline-grid",
    "contents",
    "none",
];
pub const VISIBILITY: &[&str] = &["visible", "hidden", "collapse"];
pub const FLEX_DIRECTIONS: &[&str] = &["row", "row-reverse", "column", "column-reverse"];
pub const FLEX_WRAPS: &[&str] = &["nowrap", "wrap", "wrap-reverse"];
pub const SQUARES: &[&str] = &["server", "avatar", "icon", "presence", "unread"];
pub const SELF_ALIGN: &[&str] = &["center", "start", "end", "stretch"];
pub const NUDGES: &[&str] = &["top-right"];
pub const COLORS: &[&str] = &[
    "main",
    "accent",
    "muted",
    "danger",
    "success",
    "warning",
    "info",
    "primary",
    "secondary",
    "bright",
    "white",
    "black",
    "gray",
    "slate",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "purple",
    "pink",
    "cyan",
    "transparent",
    "dusk",
    "midnight",
    "aurora",
    "ember",
    "ocean",
    "forest",
    "subtle",
];
pub const ALIGN: &[&str] = &["start", "center", "end", "stretch"];
pub const JUSTIFY: &[&str] = &["start", "center", "end", "between", "around", "evenly"];
pub const POSITIONS: &[&str] = &[
    "relative",
    "absolute",
    "sticky",
    "fixed",
    "center",
    "top",
    "bottom",
    "top-left",
    "top-right",
    "bottom-left",
    "bottom-right",
];
pub const ANCHORS: &[&str] = &[
    "top",
    "bottom",
    "left",
    "right",
    "top-left",
    "top-right",
    "bottom-left",
    "bottom-right",
];
pub const GRADIENT_TYPES: &[&str] = &["linear", "radial", "conic", "layered"];
pub const GRADIENT_CORNERS: &[&str] = &["top-left", "top-right", "bottom-left", "bottom-right"];
pub const GRID_FLOWS: &[&str] = &["horizontal", "vertical"];
pub const TRANSITIONS: &[&str] = &["smooth", "fast", "slow", "none"];
pub const DURATIONS: &[&str] = &["fast", "normal", "slow"];
pub const EASES: &[&str] = &["linear", "smooth", "bounce", "sharp"];
pub const ANIMATIONS: &[&str] = &["fade-in", "slide-up", "pop-in", "pulse", "none"];
pub const ANIMATION_PROPERTIES: &[&str] = &[
    "duration",
    "delay",
    "iteration",
    "direction",
    "fill",
    "play-state",
    "ease",
];
pub const ANIMATION_FILLS: &[&str] = &["none", "forwards", "backwards", "both"];
pub const ANIMATION_DIRECTIONS: &[&str] = &["normal", "reverse", "alternate", "alternate-reverse"];
pub const ANIMATION_PLAY_STATES: &[&str] = &["running", "paused"];
pub const BREAKPOINTS: &[&str] = &["mobile", "tablet", "desktop", "wide"];
pub const CONTAINERS: &[&str] = &["narrow", "content", "wide"];
pub const KEYFRAME_PROPERTIES: &[&str] = &[
    "opacity",
    "transform",
    "filter",
    "scale",
    "translate",
    "rotate",
];
pub const BORDER_STYLES: &[&str] = &[
    "none", "soft", "strong", "accent", "muted", "danger", "success", "warning", "width", "radius",
    "style",
];
pub const BORDER_LINE_STYLES: &[&str] = &[
    "none", "solid", "dashed", "dotted", "double", "groove", "ridge", "inset", "outset",
];
pub const Z_LAYERS: &[&str] = &[
    "base", "above", "dropdown", "sticky", "overlay", "modal", "toast",
];
pub const OPACITIES: &[&str] = &["none", "slight", "subtle", "half", "strong", "full"];

pub static REGISTRY: &[LanguageItem] = &[LanguageItem {
            name: "area",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "area",
            summary: "Defines a child region inside a grid.",
            description: "Defines a child region inside a named `grid`. Use `in` to reference the parent grid and `place`, `col`, or `row` to claim space.",
            documentation: "",
            generated_css: Some("Emits grid placement rules for the generated class."),
            frame_examples: &[r#"area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}"#],
            svelte_examples: &[r#"<aside class="fr-Sidebar">Channels</aside>"#],
            allowed_in: &[FrameScope::Root],
            related: &["grid", "in", "place", "col", "row"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/areas.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    item!("button", Declaration, Layout, Declaration, Keyword),
    LanguageItem {
            name: "card",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "card",
            summary: "Defines a reusable content surface.",
            description: "Defines a reusable content surface for panels, links, tiles, settings sections, and interactive cards. Cards commonly combine `surface`, `padding`, `radius`, `shadow`, and state effects.",
            documentation: "",
            generated_css: Some("Emits a component class with background, spacing, shape, and interaction rules."),
            frame_examples: &[r#"card HoverCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
  }
}"#],
            svelte_examples: &[r#"<a class="fr-HoverCard">Open project</a>"#],
            allowed_in: &[FrameScope::Root],
            related: &["surface", "padding", "radius", "shadow", "hover"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/cards.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "center",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "center",
            summary: "center",
            description: "",
            documentation: r#"Defines a container that centers its content.
Use it for empty states, loading states, and focused prompts."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "component",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "component",
            summary: "component",
            description: "",
            documentation: r#"## `component`

Defines a Frame UI component.

Use it for reusable interface units with typed inputs, local state, and a semantic view tree. Components may contain `props`, `state`, `view`, and `slot` blocks.

Produces compiler AST, Frame IR component metadata, TypeScript contracts, and runtime mount targets.

```frame
component Counter {
  state { count number = 0 }
  view { action Increment { on press @increment } }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "dock",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "dock",
            summary: "dock",
            description: "",
            documentation: r#"Defines an anchored interface region.
Use it for persistent app bars, bottom command bars, and docked controls.

Current generated CSS docks to the bottom of the viewport.
For a top NavBar, prefer `row NavBar` inside a page grid header area.

Top NavBar pattern:

```frame
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
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "html",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "html",
            summary: "Styles the root <html> element.",
            description: "Styles the root `<html>` element. Use for page-level background, text color, and font settings.",
            documentation: "",
            generated_css: Some("Emits a bare `html { ... }` rule (not a class)."),
            frame_examples: &[r#"html {
  background #0a0f1a
  color #e2e8f0
}"#],
            svelte_examples: &[],
            allowed_in: &[FrameScope::Root],
            related: &["body"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "page-body",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "page-body",
            summary: "Styles the <body> element.",
            description: "Styles the `<body>` element. Use for page-level layout, margin, min-height, and background.",
            documentation: "",
            generated_css: Some("Emits a bare `body { ... }` rule (not a class)."),
            frame_examples: &[r#"page-body {
  margin none
  min-height screen
  background #0a0f1a
  color #e2e8f0
}"#],
            svelte_examples: &[],
            allowed_in: &[FrameScope::Root],
            related: &["html"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "grid",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "grid",
            summary: "Defines a layout container.",
            description: r#"Defines a layout container using Frame's grid system. Use `columns`, `rows`, `gap`, and child `area` declarations to create app shells, dashboards, sidebars, inspectors, and card grids. Inline Svelte components can use it inside `<style lang="frame">`."#,
            documentation: "",
            generated_css: Some("Emits `display: grid` with readable grid template rules."),
            frame_examples: &[r#"grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}"#],
            svelte_examples: &[r#"<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>"#],
            allowed_in: &[FrameScope::Root],
            related: &["area", "columns", "rows", "place", "col"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/grid.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "keyframes",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "keyframes",
            summary: "Defines reusable animation keyframes.",
            description: "Defines reusable animation keyframes with structured timeline selectors. Use `from`, `to`, and percentage blocks to teach Frame how an animation changes over time.",
            documentation: "",
            generated_css: Some("Emits `@keyframes frame-Name` with supported animatable properties."),
            frame_examples: &[r#"keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px) scale(0.98)
  }

  to {
    opacity 1
    transform translateY(0) scale(1)
  }
}"#],
            svelte_examples: &[r#"<section class="fr-Panel">...</section>"#],
            allowed_in: &[FrameScope::Root],
            related: &["animation", "duration", "ease", "fill"],
            values: &["from", "to", "50%", "opacity", "transform"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/animations.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "overlay",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "overlay",
            summary: "overlay",
            description: "",
            documentation: r#"Defines a fixed layer above the page.
Use it for modals, command palettes, popovers, and blocking dialogs.

Generated CSS: fixed positioning with full-page inset.

Example:

```frame
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
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "row",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "row",
            summary: "Defines a horizontal layout group.",
            description: "As a declaration, creates a horizontal layout for toolbars, NavBars, button groups, and header rows. As an area property, `row` places an area in a numeric grid row.",
            documentation: "",
            generated_css: Some("Declaration output uses flex row layout; area placement output uses grid row placement."),
            frame_examples: &[r#"row Toolbar {
  align center
  justify between
  gap small
  padding medium
  surface panel
}"#],
            svelte_examples: &[r#"<div class="fr-Toolbar">
  <button>Back</button>
  <button>Save</button>
</div>"#],
            allowed_in: &[FrameScope::Root, FrameScope::Area],
            related: &["stack", "align", "justify", "gap"],
            values: &["1", "2", "3", "all"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/layout.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "split",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "split",
            summary: "split",
            description: "",
            documentation: r#"Defines a two-region layout.
Use it for sidebar/content, editor/preview, or master/detail views.

Generated CSS currently creates a grid with an auto column and a fill column.
For precise horizontal ratios, use `grid` with percentage `columns`.

Example:

```frame
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
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "stack",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "stack",
            summary: "Defines a vertical layout group.",
            description: "Defines a vertical layout group. Use `gap`, `align`, `padding`, and `surface` for panels, forms, and settings layouts.",
            documentation: "",
            generated_css: Some("Emits a column-oriented layout class."),
            frame_examples: &[r#"stack SettingsPanel {
  gap medium
  padding large
  surface panel
}"#],
            svelte_examples: &[r#"<section class="fr-SettingsPanel">...</section>"#],
            allowed_in: &[FrameScope::Root],
            related: &["row", "gap", "align"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/layout.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "style-group",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "style-group",
            summary: "style-group",
            description: "",
            documentation: r#"Starts a named style group.

Style groups map to CSS cascade layers while keeping Frame syntax intent-focused.

Example:

```frame
style-group components {
  button PrimaryButton {
    surface accent
  }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "style-order",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "style-order",
            summary: "style-order",
            description: "",
            documentation: r#"Declares deterministic style group order.

Example:

```frame
style-order reset, base, components, utilities
```

Generated CSS emits a cascade layer order rule."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "supports",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "supports",
            summary: "supports",
            description: "",
            documentation: r#"Starts a typed feature query block.

Use predicates like `supports display grid`, `supports backdrop blur`, `supports color oklch`, `supports selector has`, `supports container queries`, or `supports subgrid`.

Generated CSS emits an `@supports` rule."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "text",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "text",
            summary: "text",
            description: "",
            documentation: r#"Defines reusable typography intent.
Use size, weight, font, and color tokens instead of raw font CSS."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "tokens",
            kind: LanguageItemKind::Declaration,
            layer: LanguageLayer::Layout,
            detail: "tokens",
            summary: "tokens",
            description: "",
            documentation: r#"Defines reusable design tokens for a Frame file.
Use tokens to name shared visual decisions before applying them to components."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "action",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "action",
            summary: "action",
            description: "",
            documentation: r#"## `action`

Represents a user-triggered command.

Use `on press @handler` for activation instead of browser event attributes. It lowers to an accessible action control.

```frame
action Send:PrimaryAction {
  text "Send"
  on press @sendMessage
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "avatar",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "avatar",
            summary: "avatar",
            description: "",
            documentation: r#"## `avatar`

Represents a person or entity image.

Use `source` and `alt` unless the image is decorative. It lowers to image-like renderer output.

```frame
avatar AuthorAvatar { source $avatar alt $author }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "badge",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "badge",
            summary: "badge",
            description: "",
            documentation: r#"## `badge`

Represents compact status or metadata.

Use it for counts, states, and short labels.

```frame
badge Status { text "New" }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "choice",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "choice",
            summary: "choice",
            description: "",
            documentation: r#"## `choice`

Represents a small option choice.

Use it for radio-like or segmented choices. Renderers decide the exact control shape.

```frame
choice ThemeChoice {
  selected bind $theme
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "composer",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "composer",
            summary: "composer",
            description: "",
            documentation: r#"## `composer`

Represents input collection and submission intent.

Use it for message composers, forms, and submit flows without writing browser form syntax.

```frame
composer MessageComposer {
  draft bind $draft
  send @sendMessage
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "data",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "data",
            summary: "data",
            description: "",
            documentation: r#"## `data`

Represents structured records.

Use it for rows and fields without exposing table syntax in Frame source.

```frame
data Invoices {
  for invoice in $invoices key $invoice.id { row Invoice { text $invoice.total } }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "dialog",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "dialog",
            summary: "dialog",
            description: "",
            documentation: r#"## `dialog`

Represents a modal or attention surface.

Use `show when $state` and explicit close actions. Renderers handle focus and modal behavior as support matures.

```frame
dialog SettingsDialog { show when $open }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "editor",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "editor",
            summary: "editor",
            description: "",
            documentation: r#"## `editor`

Represents multi-line text editing.

Use it for comments, messages, and document text. It lowers to a multi-line editing control in DOM.

```frame
editor BodyEditor {
  value bind $body
  on keydown.ctrl.enter @save
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "empty",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "empty",
            summary: "empty",
            description: "",
            documentation: r#"## `empty`

Represents fallback content for an empty collection.

Use it inside collection primitives.

```frame
empty NoMessages { text "No messages yet" }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "feed",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "feed",
            summary: "feed",
            description: "",
            documentation: r#"## `feed`

Represents chronological or activity-stream content.

Use it for messages, events, and updates where order matters.

```frame
feed Activity {
  for event in $events key $event.id { item Event { text $event.title } }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "field",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "field",
            summary: "field",
            description: "",
            documentation: r#"## `field`

Groups a label, help text, validation state, and one control.

Use it around `input`, `editor`, `toggle`, `choice`, or `select` so accessibility and layout stay semantic. It lowers to a neutral field container.

```frame
field EmailField {
  label "Email"
  input EmailInput { value bind $email }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "icon",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "icon",
            summary: "icon",
            description: "",
            documentation: r#"## `icon`

Represents symbolic visual content.

Use it for decorative or named symbols. Decorative icons lower with hidden accessibility metadata.

```frame
icon SearchIcon { label "Search" }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "image",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "image",
            summary: "image",
            description: "",
            documentation: r#"## `image`

Represents meaningful imagery.

Use `source` or `sources` and `alt`. Renderers validate URL-like sinks.

```frame
image Cover { source $cover alt "Cover" }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "input",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "input",
            summary: "input",
            description: "",
            documentation: r#"## `input`

Represents single-value text entry.

Use `value bind $state` to connect it to component state. It lowers to the renderer's text-input control.

```frame
input MessageInput {
  value bind $draft
  placeholder "Message"
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "item",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "item",
            summary: "item",
            description: "",
            documentation: r#"## `item`

Represents one repeated collection entry.

Use it inside `list`, `feed`, or `data` loops.

```frame
item Message { text $message.body }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "label",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "label",
            summary: "label",
            description: "",
            documentation: r#"## `label`

Represents visible naming text or a control label.

Use it inside fields and controls so renderers can preserve accessibility relationships.

```frame
label "Email"
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "link",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "link",
            summary: "link",
            description: "",
            documentation: r#"## `link`

Represents navigation intent.

Use `goto` for the destination. Renderers validate the target and lower to platform navigation.

```frame
link Docs {
  goto "/docs"
  text "Docs"
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "list",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "list",
            summary: "list",
            description: "",
            documentation: r#"## `list`

Represents repeated content.

Use `for item in $items key $item.id` for stable identity and `empty` for fallback content.

```frame
list Messages {
  for message in $messages key $message.id {
    item Message { text $message.body }
  }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "media",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "media",
            summary: "media",
            description: "",
            documentation: r#"## `media`

Represents audio or video playback intent.

Use `sources`, `poster`, and labels. The DOM runtime lowers it to media controls.

```frame
media Preview { sources $video poster $poster }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "menu",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "menu",
            summary: "menu",
            description: "",
            documentation: r#"## `menu`

Represents navigation or command choices.

Use it for app navigation and command groups. It lowers to renderer navigation/menu structures.

```frame
menu MainNav {
  link Docs { goto "/docs" text "Docs" }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "panel",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "panel",
            summary: "panel",
            description: "",
            documentation: r#"## `panel`

Represents a named region of interface content.

Use it for sidebars, panes, inspectors, and grouped app regions. It lowers to a neutral container while preserving semantic region intent.

```frame
panel Sidebar {
  title "Channels"
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "popover",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "popover",
            summary: "popover",
            description: "",
            documentation: r#"## `popover`

Represents lightweight contextual content.

Use it for small overlays tied to another interaction.

```frame
popover HelpPopover { text "More detail" }
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "screen",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "screen",
            summary: "screen",
            description: "",
            documentation: r#"## `screen`

Represents a full UI surface.

Use it as a view root for pages, tools, and app screens. It lowers to renderer root/container metadata and defaults to a DOM container in the DOM runtime.

```frame
screen AppScreen:AppShell {
  stack Content { text "Hello" }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "scroll",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "scroll",
            summary: "scroll",
            description: "",
            documentation: "Makes a region scroll on an axis. Use `scroll y` for channel panels, message lists, and member lists.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    item!("section", Primitive, Ui, Declaration, Keyword),
    LanguageItem {
            name: "select",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "select",
            summary: "select",
            description: "",
            documentation: r#"## `select`

Represents selection from a larger or dynamic option set.

Use `selected bind $state` and `options $items`. It lowers to a selection control.

```frame
select ChannelSelect {
  selected bind $channel
  options $channels
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "tabs",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "tabs",
            summary: "tabs",
            description: "",
            documentation: r#"## `tabs`

Represents switching between related panels.

Use it when the user selects one visible panel from a small set. It records tab intent for renderer accessibility behavior.

```frame
tabs SettingsTabs {
  action General { on press @openGeneral }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "title",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "title",
            summary: "title",
            description: "",
            documentation: r#"## `title`

Represents semantic title text.

Use it for headings without choosing a browser heading level. Renderers choose the appropriate output.

```frame
title "Settings"
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "toggle",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "toggle",
            summary: "toggle",
            description: "",
            documentation: r#"## `toggle`

Represents a binary setting.

Use `checked bind $state` for two-way boolean state. It lowers to a checkable control.

```frame
toggle CompactMode {
  label "Compact mode"
  checked bind $compact
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "toolbar",
            kind: LanguageItemKind::Primitive,
            layer: LanguageLayer::Ui,
            detail: "toolbar",
            summary: "toolbar",
            description: "",
            documentation: r#"## `toolbar`

Represents a compact group of related actions.

Use it for editor commands and app chrome. It lowers to a command region with grouped actions.

```frame
toolbar EditorTools {
  action Save { on press @save }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Declaration,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "active",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "active",
            summary: "active",
            description: "",
            documentation: "Defines effects applied while this component is being pressed.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "checked",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "checked",
            summary: "checked",
            description: "",
            documentation: r#"Defines effects applied to checked controls.

Generated CSS emits `:checked`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "disabled",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "disabled",
            summary: "disabled",
            description: "",
            documentation: "Defines visual treatment for unavailable controls.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "focus",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "focus",
            summary: "focus",
            description: "",
            documentation: "Defines effects applied when keyboard or assistive focus reaches this component.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "focus-visible",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "focus-visible",
            summary: "focus-visible",
            description: "",
            documentation: r#"Defines effects applied when focus should be visibly indicated.

Generated CSS emits `:focus-visible`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "focus-within",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "focus-within",
            summary: "focus-within",
            description: "",
            documentation: r#"Defines effects applied when this element or any descendant has focus.

Generated CSS emits `:focus-within`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "hover",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "hover",
            summary: "Defines hover effects.",
            description: "Defines effects applied when the pointer hovers this component. Keep this block focused on interaction effects like `lift`, `glow`, and `brighten`.",
            documentation: "",
            generated_css: Some("Emits `:hover` rules for the generated class."),
            frame_examples: &[r#"hover {
  lift small
  glow accent
}"#],
            svelte_examples: &[r#"<a class="fr-HoverCard">Docs</a>"#],
            allowed_in: &[FrameScope::Component],
            related: &["focus", "active", "lift", "glow"],
            values: &["lift", "glow", "brighten", "dim"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/effects.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "invalid",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "invalid",
            summary: "invalid",
            description: "",
            documentation: r#"Defines effects applied to invalid form controls.

Generated CSS emits `:invalid`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "required",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "required",
            summary: "required",
            description: "",
            documentation: r#"Defines effects applied to required form controls.

Generated CSS emits `:required`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "target",
            kind: LanguageItemKind::StateKeyword,
            layer: LanguageLayer::Ui,
            detail: "target",
            summary: "target",
            description: "",
            documentation: r#"In UI attributes, controls where a link or form opens. Use `rel "noopener"` or `rel "noreferrer"` with `target "_blank"`.

In style declarations, defines effects applied when this element matches the URL fragment target and emits `:target`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::StateBlock,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "blur",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "blur",
            summary: "blur",
            description: "",
            documentation: "Applies blur intent, usually for overlays or state effects.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "change",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "change",
            summary: "change",
            description: "",
            documentation: "Fires when the value of this control changes.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "click",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "click",
            summary: "click",
            description: "",
            documentation: "Fires when the user clicks this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "close",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "close",
            summary: "close",
            description: "",
            documentation: "Fires when this dialog or popover should close.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "keydown",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "keydown",
            summary: "keydown",
            description: "",
            documentation: "Fires when a key is pressed down while this element has focus.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "keyup",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "keyup",
            summary: "keyup",
            description: "",
            documentation: "Fires when a key is released while this element has focus.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "mouseenter",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "mouseenter",
            summary: "mouseenter",
            description: "",
            documentation: "Fires when the pointer enters this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "mouseleave",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "mouseleave",
            summary: "mouseleave",
            description: "",
            documentation: "Fires when the pointer leaves this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "open",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "open",
            summary: "open",
            description: "",
            documentation: "Fires when this dialog or popover should open.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "pointerdown",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "pointerdown",
            summary: "pointerdown",
            description: "",
            documentation: "Fires when the pointer is pressed down on this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "pointermove",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "pointermove",
            summary: "pointermove",
            description: "",
            documentation: "Fires when the pointer moves over this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "pointerup",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "pointerup",
            summary: "pointerup",
            description: "",
            documentation: "Fires when the pointer is released over this element.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "press",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "press",
            summary: "press",
            description: "",
            documentation: "Adds a pressed movement for active controls.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "reset",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "reset",
            summary: "reset",
            description: "",
            documentation: "Fires when this form or composer should reset.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "send",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "send",
            summary: "send",
            description: "",
            documentation: "References the external handler that sends or submits a composer-like primitive.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "submit",
            kind: LanguageItemKind::Event,
            layer: LanguageLayer::Ui,
            detail: "submit",
            summary: "submit",
            description: "",
            documentation: "Fires when this form or composer is submitted.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "alt",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "alt",
            summary: "alt",
            description: "",
            documentation: "Event modifier requiring the Alt/Option key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "capture",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "capture",
            summary: "capture",
            description: "",
            documentation: "Event modifier that adds the listener in capture phase.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "ctrl",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "ctrl",
            summary: "ctrl",
            description: "",
            documentation: "Event modifier requiring the Control key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "down",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "down",
            summary: "down",
            description: "",
            documentation: "Event modifier for the Down Arrow key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "enter",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "enter",
            summary: "enter",
            description: "",
            documentation: "Event modifier for the Enter/Return key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "escape",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "escape",
            summary: "escape",
            description: "",
            documentation: "Event modifier for the Escape key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "left",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "left",
            summary: "left",
            description: "",
            documentation: "Event modifier for the Left Arrow key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "meta",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "meta",
            summary: "meta",
            description: "",
            documentation: "Event modifier requiring the Meta/Command key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "once",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "once",
            summary: "once",
            description: "",
            documentation: "Event modifier that removes the listener after the first firing.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "passive",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "passive",
            summary: "passive",
            description: "",
            documentation: "Event modifier that marks the listener as passive (cannot call preventDefault).",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "prevent",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "prevent",
            summary: "prevent",
            description: "",
            documentation: "Event modifier that calls preventDefault on the event.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "right",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "right",
            summary: "right",
            description: "",
            documentation: "Event modifier for the Right Arrow key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "shift",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "shift",
            summary: "shift",
            description: "",
            documentation: "Event modifier requiring the Shift key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "space",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "space",
            summary: "space",
            description: "",
            documentation: "Event modifier for the Space key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "stop",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "stop",
            summary: "stop",
            description: "",
            documentation: "Event modifier that stops event propagation.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "tab",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "tab",
            summary: "tab",
            description: "",
            documentation: "Event modifier for the Tab key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "up",
            kind: LanguageItemKind::EventModifier,
            layer: LanguageLayer::Ui,
            detail: "up",
            summary: "up",
            description: "",
            documentation: "Event modifier for the Up Arrow key.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "class",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "class",
            summary: "class",
            description: "",
            documentation: "Adds an explicit CSS class to a UI node. Use it for renderer-specific styling that does not yet have a Frame-native equivalent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "rel",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "rel",
            summary: "rel",
            description: "",
            documentation: "Sets link relationship intent. Common values: `noopener`, `noreferrer`, `nofollow`. Use with `target` or `new-window` for external links.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("data-test-id", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "decorative",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "decorative",
            summary: "decorative",
            description: "",
            documentation: "Marks image-like content as visual-only when set to `true`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "description",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "description",
            summary: "description",
            description: "",
            documentation: "Adds descriptive accessibility metadata without manual ARIA wiring.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("download", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "draft",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "draft",
            summary: "draft",
            description: "",
            documentation: "Binds composer draft text to component state.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "for",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "for",
            summary: "for",
            description: "",
            documentation: r#"Starts renderer-neutral list rendering.

Use `for item in $items { ... }` for positional lists or `for item in $items key $id { ... }` when stable identity is available. The compiler lowers this to Frame IR list metadata; renderers decide how to realize updates."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "goto",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "goto",
            summary: "goto",
            description: "",
            documentation: "Declares navigation intent for `link`. Renderers validate the destination and lower it to their platform.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "hidden",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "hidden",
            summary: "hidden",
            description: "",
            documentation: "`hidden when $state` records visibility intent without requiring a specific renderer mechanism.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "hint",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "hint",
            summary: "hint",
            description: "",
            documentation: "Adds helper text for form-like primitives without manual ARIA wiring.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("id", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "in",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "in",
            summary: "in",
            description: "",
            documentation: r#"References the parent grid for an area, or introduces the collection in a `for` loop.

- In an `area`: `in AppShell` references the parent grid.
- In a `for` loop: `for item in $items` introduces the collection to iterate."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "key",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "key",
            summary: "key",
            description: "",
            documentation: r#"Declares stable identity for a Frame list.

Keyed lists allow renderers to reuse item instances by identity. Non-keyed lists use positional update behavior."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("kind", UiKeyword, Ui, Value, EnumMember),
    item!("new-window", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "on",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "on",
            summary: "on",
            description: "",
            documentation: r#"Binds a UI event to an external handler reference.
Frame stores `@handlerName`, not inline JavaScript or TypeScript bodies."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("options", UiKeyword, Ui, Value, EnumMember),
    item!("placeholder", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "poster",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "poster",
            summary: "poster",
            description: "",
            documentation: "Media destination. Renderers validate URL-like values before writing platform sinks.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "props",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "props",
            summary: "props",
            description: "",
            documentation: r#"## `props`

Declares typed inputs accepted by a component.

Use props for data supplied by a parent component. Props are read by `$name` references and lower to IR prop descriptors plus TypeScript prop contracts.

```frame
props {
  title text
  selected bool
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("readonly", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "selected",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "selected",
            summary: "selected",
            description: "",
            documentation: "`selected bind $state` connects select-like controls to typed component state.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "show",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "show",
            summary: "show",
            description: "",
            documentation: "`show when $state` records conditional rendering intent. The runtime tracks the dependency; renderers decide whether to create, skip, or serialize the node.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "slot",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "slot",
            summary: "slot",
            description: "",
            documentation: r#"Defines a named content region inside a component.
Slots allow parent components to inject content. The default slot is named `Default`.

Example:

```frame
slot Default {
  text "Fallback content"
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "source",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "source",
            summary: "source",
            description: "",
            documentation: "Declares data or media source intent. Lists use `source $items`; images use `source $image`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("sources", UiKeyword, Ui, Value, EnumMember),
    LanguageItem {
            name: "state",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "state",
            summary: "state",
            description: "",
            documentation: r#"## `state`

Declares local mutable component data.

Use state for values changed by handlers or bindings. State lowers to IR state descriptors with serialized defaults and runtime state slots.

```frame
state {
  draft text = ""
  sending bool = false
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "style",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "style",
            summary: "style",
            description: "",
            documentation: r#"## `style`

Applies state-driven style switching inside a UI node.

Use it when a node should gain a style class while state is true. It lowers to an IR conditional style and the DOM runtime patches classes.

```frame
action Send:PrimaryAction {
  style SendingAction when $sending
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "value",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "value",
            summary: "value",
            description: "",
            documentation: "Stores a visible value for content primitives or a value binding target for input-like primitives.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "view",
            kind: LanguageItemKind::UiKeyword,
            layer: LanguageLayer::Ui,
            detail: "view",
            summary: "view",
            description: "",
            documentation: r#"## `view`

Declares the component UI tree with Frame primitives.

Use semantic primitives such as `screen`, `panel`, `stack`, `field`, `input`, `list`, and `action`. Renderers lower that intent to their target platform.

```frame
view {
  stack Content {
    text $title
    action Save { on press @save }
  }
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "bind",
            kind: LanguageItemKind::BindingKeyword,
            layer: LanguageLayer::Ui,
            detail: "bind",
            summary: "bind",
            description: "",
            documentation: "`bind $state`, `draft bind $state`, and similar forms record two-way state binding intent without exposing browser form controls.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "when",
            kind: LanguageItemKind::BindingKeyword,
            layer: LanguageLayer::Ui,
            detail: "when",
            summary: "when",
            description: "",
            documentation: "Introduces a state-driven condition such as `disabled when $sending` or `style when $sending = LoadingButton`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::Keyword,
        },
    LanguageItem {
            name: "animate",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "animate",
            summary: "animate",
            description: "",
            documentation: r#"Applies a named entrance or emphasis animation.

Common values: `fade-in`, `slide-up`, `pop-in`, `pulse`, and `none`.

```frame
card Notice {
  surface panel
  animation pop-in
}
```

Generated CSS uses deterministic keyframes such as `frame-pop-in`.

Docs: `docs/animations.md`"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "animation",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "animation",
            summary: "Applies preset or custom animation motion.",
            description: "Applies a preset animation or a custom `keyframes` declaration. Use a one-line preset for common motion, or an `animation Name { ... }` block when timing, delay, iteration, direction, fill mode, or play state matter.",
            documentation: "",
            generated_css: Some("Emits `animation: frame-name ...` and `animation-play-state` when configured."),
            frame_examples: &[r#"card Panel {
  animation FloatIn {
    duration 240ms
    ease smooth
    fill both
  }
}"#],
            svelte_examples: &[r#"<section class="fr-Panel">...</section>"#],
            allowed_in: &[FrameScope::Component, FrameScope::State],
            related: &["keyframes", "duration", "delay", "ease", "iteration"],
            values: &["fade-in", "slide-up", "pop-in", "pulse"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/animations.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "brighten",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "brighten",
            summary: "brighten",
            description: "",
            documentation: "Slightly increases visual brightness for interactive feedback.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "dim",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "dim",
            summary: "dim",
            description: "",
            documentation: "Reduces visual emphasis for disabled or inactive states.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "duration",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "duration",
            summary: "duration",
            description: "",
            documentation: "Sets motion duration intent. Use `fast`, `normal`, or `slow` with transitions and animations.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "ease",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "ease",
            summary: "ease",
            description: "",
            documentation: "Sets easing intent. Use `linear`, `smooth`, `bounce`, or `sharp` to describe motion feel.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "fade",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "fade",
            summary: "fade",
            description: "",
            documentation: "Fade effect for transitions and animations.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "glow",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "glow",
            summary: "glow",
            description: "",
            documentation: "Adds a semantic glow, commonly using accent, danger, or success.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "grow",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "grow",
            summary: "grow",
            description: "",
            documentation: r#"Scales a component up by intent.

Use visual amounts `slight`, `subtle`, `normal`, `strong`, or `dramatic`. Add `%0` through `%100` for fine tuning, for example `grow slight%5`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "lift",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "lift",
            summary: "lift",
            description: "",
            documentation: r#"Moves a component upward to express elevation.

Use movement amounts `tiny`, `small`, `medium`, `large`, or `huge`. Add `%0` through `%100` to tune toward the next stronger amount, for example `lift small%44`.

Generated CSS composes this into `transform: translateY(...)`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "pop",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "pop",
            summary: "pop",
            description: "",
            documentation: "Adds a small positive scale movement for appearing or selected states.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "ring",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "ring",
            summary: "ring",
            description: "",
            documentation: "Adds an accessible focus ring using a semantic color.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "rotate",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "rotate",
            summary: "rotate",
            description: "",
            documentation: "Rotate effect for transitions and animations.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "scale",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "scale",
            summary: "scale",
            description: "",
            documentation: "Scale effect for transitions and animations.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "shrink",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "shrink",
            summary: "shrink",
            description: "",
            documentation: "Scales a component down by intent using visual amounts and optional percent tuning.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "sink",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "sink",
            summary: "sink",
            description: "",
            documentation: r#"Moves a component downward.

Use movement amounts like `small` or tuned values like `small%44`. Generated CSS composes this into `transform: translateY(...)`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "slide",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "slide",
            summary: "slide",
            description: "",
            documentation: "Slide effect for transitions and animations.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "smooth",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "smooth",
            summary: "smooth",
            description: "",
            documentation: "Expresses smooth transition intent for interaction effects.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "tilt",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "tilt",
            summary: "tilt",
            description: "",
            documentation: r#"Rotates a component by intent.

Use `tilt left subtle` or `tilt right subtle`. Visual amounts can be tuned with suffix percentages, for example `tilt right subtle%23`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "transition",
            kind: LanguageItemKind::Effect,
            layer: LanguageLayer::Motion,
            detail: "transition",
            summary: "transition",
            description: "",
            documentation: r#"Sets named transition intent for interactive changes.

Use `transition smooth` on a component or inside `hover`, `focus`, and `active` blocks.

```frame
card HoverCard {
  transition smooth

  hover {
    lift small
    glow accent
    transition fast
  }
}
```

Generated CSS writes predictable transition timing such as `all 200ms ease`.

Docs: `docs/animations.md`"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "advanced",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Advanced,
            detail: "advanced",
            summary: "advanced",
            description: "",
            documentation: "Advanced CSS escape hatch. Use `advanced` for properties that do not yet have structured Frame syntax.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "align",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "align",
            summary: "Controls cross-axis placement.",
            description: "Controls cross-axis placement. In a `row`, this usually means vertical alignment. In a `stack`, it usually means horizontal alignment.",
            documentation: "",
            generated_css: Some("Emits `align-items` or an equivalent placement rule."),
            frame_examples: &[r#"row Toolbar {
  align center
  justify between
  gap small
  padding medium
  surface panel
}"#],
            svelte_examples: &[r#"<div class="fr-Toolbar">
  <button>Back</button>
  <button>Save</button>
</div>"#],
            allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
            related: &["justify", "row", "stack"],
            values: &["start", "center", "end", "stretch"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/layout.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "align-text",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "align-text",
            summary: "align-text",
            description: "",
            documentation: "Aligns text inside controls and rows. Use `align-text left` for dense navigation buttons.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "anchor",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "anchor",
            summary: "anchor",
            description: "",
            documentation: "Sets anchor intent for positioned elements.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "angle",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "angle",
            summary: "angle",
            description: "",
            documentation: "Sets gradient angle intent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "areas",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "areas",
            summary: "areas",
            description: "",
            documentation: r#"Defines one row of a named grid area template.
Repeat `areas ...` lines to build multi-row app shells without raw `grid-template-areas`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "at",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "at",
            summary: "at",
            description: "",
            documentation: "Sets animation or gradient position intent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "background",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "background",
            summary: "background",
            description: "",
            documentation: "Sets background intent using Frame surface or color tokens.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "block-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "block-size",
            summary: "block-size",
            description: "",
            documentation: r#"Sets logical block size. In horizontal writing modes this usually behaves like height.
Generated CSS writes `block-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "border",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "border",
            summary: "border",
            description: "",
            documentation: r#"Sets border intent.

Use semantic border colors such as `border accent`, thickness with `border width medium`, line styles with `border style dashed`, or `border none`.

Generated CSS writes border color, width, radius, or style rules."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "box",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "box",
            summary: "box",
            description: "",
            documentation: "Sets box sizing intent. Use `box border` for app surfaces where borders should be included in dimensions.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "case",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "case",
            summary: "case",
            description: "",
            documentation: "Controls text casing intent. Use `case uppercase` for compact section labels.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "col",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "col",
            summary: "col",
            description: "",
            documentation: r#"Places an area in a numeric grid column.
Use this when columns are percentages or explicit tracks.

```frame
grid Dashboard {
  columns 25% 50% 25%
}

area Sidebar {
  in Dashboard
  col 1
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "color",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "color",
            summary: "color",
            description: "",
            documentation: "Sets foreground color intent using Frame color tokens.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "columns",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "columns",
            summary: "Defines horizontal sections of a grid.",
            description: "Defines horizontal sections of a `grid`. Use named sections for readable `place` references, percentages for explicit ratios, or `responsive cards` for card grids.",
            documentation: "",
            generated_css: Some("Emits `grid-template-columns`; named columns also create named grid areas."),
            frame_examples: &[r#"columns sidebar content inspector
columns 25% 50% 25%
columns responsive cards"#],
            svelte_examples: &[r#"<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>"#],
            allowed_in: &[FrameScope::Grid],
            related: &["grid", "area", "place", "col"],
            values: &["sidebar", "content", "inspector", "25%", "50%", "responsive cards"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/grid.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "control",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "control",
            summary: "control",
            description: "",
            documentation: "Applies control affordance intent. Use `control reset` to remove browser-specific button or input appearance.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "corner",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "corner",
            summary: "corner",
            description: "",
            documentation: "Sets gradient corner intent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "css",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Advanced,
            detail: "css",
            summary: "css",
            description: "",
            documentation: "Advanced CSS escape hatch. Use `css { ... }` to pass raw CSS through when Frame syntax does not yet cover a property.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::AdvancedProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "decoration",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "decoration",
            summary: "decoration",
            description: "",
            documentation: r#"Sets text decoration line intent.

Use `decoration underline`, `decoration overline`, `decoration line-through`, or `decoration none`.

Generated CSS writes `text-decoration-line`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "delay",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "delay",
            summary: "delay",
            description: "",
            documentation: "Sets the delay before an animation starts. Use named timing or CSS time values like `120ms`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "direction",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "direction",
            summary: "direction",
            description: "",
            documentation: "Sets animation playback direction such as `normal`, `reverse`, or `alternate`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "display",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "display",
            summary: "display",
            description: "",
            documentation: r#"Sets the element display mode without using the raw CSS escape hatch.

Common values:
- `block`
- `inline`
- `inline-block`
- `flex`
- `inline-flex`
- `grid`
- `inline-grid`
- `contents`
- `none`

Generated CSS writes `display: ...`.

```frame
card Toolbar {
  display flex
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "fill",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "fill",
            summary: "fill",
            description: "",
            documentation: "Sizes an element to fill available space. Inside an `animation` block, `fill` sets animation fill mode such as `both`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "filter",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "filter",
            summary: "filter",
            description: "",
            documentation: "Applies CSS filter intent for effects like blur or brightness.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "flex",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "flex",
            summary: "flex",
            description: "",
            documentation: r#"Controls flexbox behavior through structured subcommands.

Supported forms:
- `flex direction row`
- `flex direction column`
- `flex wrap wrap`
- `flex grow 1`
- `flex shrink 0`
- `flex basis fill`

Generated CSS writes `flex-direction`, `flex-wrap`, `flex-grow`, `flex-shrink`, or `flex-basis`.

```frame
row Toolbar {
  flex wrap wrap
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "flow",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "flow",
            summary: "flow",
            description: "",
            documentation: "Sets grid auto-flow intent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "font",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "font",
            summary: "font",
            description: "",
            documentation: "Selects a typography family intent such as mono.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "gap",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "gap",
            summary: "gap",
            description: "",
            documentation: "Sets spacing between children using Frame spacing tokens like small, medium, and large.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "gradient",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "gradient",
            summary: "gradient",
            description: "",
            documentation: "Selects a named gradient surface such as dusk, midnight, or aurora.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "height",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "height",
            summary: "height",
            description: "",
            documentation: r#"Sets height intent with values such as screen, fill, content, or percentages.
Generated CSS writes `height`, with `screen` becoming `100vh`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "hyphenate",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "hyphenate",
            summary: "hyphenate",
            description: "",
            documentation: r#"Controls hyphenation behavior.

Use `hyphenate auto` for prose where browser hyphenation is acceptable.

Generated CSS writes `hyphens`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "inline-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "inline-size",
            summary: "inline-size",
            description: "",
            documentation: r#"Sets logical inline size. In horizontal writing modes this usually behaves like width.
Generated CSS writes `inline-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "interactive",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "interactive",
            summary: "interactive",
            description: "",
            documentation: "Marks a surface as pointer-interactive and emits cursor affordance.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "iteration",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "iteration",
            summary: "iteration",
            description: "",
            documentation: "Sets animation repeat count. Use a number or `infinite`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "justify",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "justify",
            summary: "Controls main-axis placement.",
            description: "Controls main-axis placement and distribution. In a `row`, this usually means horizontal distribution.",
            documentation: "",
            generated_css: Some("Emits `justify-content` or an equivalent placement rule."),
            frame_examples: &[r#"row Toolbar {
  align center
  justify between
  gap small
  padding medium
  surface panel
}"#],
            svelte_examples: &[r#"<div class="fr-Toolbar">
  <button>Back</button>
  <button>Save</button>
</div>"#],
            allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
            related: &["align", "row", "stack"],
            values: &["start", "center", "end", "between", "around", "evenly"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/layout.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "layout",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "layout",
            summary: "layout",
            description: "",
            documentation: "Applies a dense internal layout preset for repeated app patterns such as icon/content/action rows and avatar/content message rows.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    item!("letter", Property, Typography, TypographyProperty, Property),
    item!("line", Property, Typography, TypographyProperty, Property),
    LanguageItem {
            name: "margin",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "margin",
            summary: "margin",
            description: "",
            documentation: "Adds outer spacing using Frame spacing tokens.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "max-block-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "max-block-size",
            summary: "max-block-size",
            description: "",
            documentation: r#"Sets maximum logical block size.
Generated CSS writes `max-block-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "max-height",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "max-height",
            summary: "max-height",
            description: "",
            documentation: "Sets maximum height intent using named sizes or percentages.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "max-inline-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "max-inline-size",
            summary: "max-inline-size",
            description: "",
            documentation: r#"Sets maximum logical inline size.
Generated CSS writes `max-inline-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "max-width",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "max-width",
            summary: "max-width",
            description: "",
            documentation: "Sets maximum width intent using named sizes or percentages.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "min-block-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "min-block-size",
            summary: "min-block-size",
            description: "",
            documentation: r#"Sets minimum logical block size.
Generated CSS writes `min-block-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "min-height",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "min-height",
            summary: "min-height",
            description: "",
            documentation: "Sets minimum height intent using named sizes or percentages.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "min-inline-size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "min-inline-size",
            summary: "min-inline-size",
            description: "",
            documentation: r#"Sets minimum logical inline size.
Generated CSS writes `min-inline-size`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "min-width",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "min-width",
            summary: "min-width",
            description: "",
            documentation: "Sets minimum width intent using named sizes or percentages.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "nudge",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "nudge",
            summary: "nudge",
            description: "",
            documentation: "Applies a small positional adjustment for badges and status dots.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "offset",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "offset",
            summary: "offset",
            description: "",
            documentation: "Sets offset intent for positioned elements.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "opacity",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "opacity",
            summary: "Controls element opacity.",
            description: "Controls element opacity using named levels that map to numeric values: none (0), slight (0.1), subtle (0.25), half (0.5), strong (0.75), full (1.0).",
            documentation: r#"Controls element opacity using named levels.

Common values:
- `none` → 0
- `slight` → 0.1
- `subtle` → 0.25
- `half` → 0.5
- `strong` → 0.75
- `full` → 1.0

Generated CSS writes `opacity: ...`.

```frame
card FadedCard {
  opacity subtle
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[FrameScope::Component, FrameScope::State],
            related: &["fade", "visibility"],
            values: &["none", "slight", "subtle", "half", "strong", "full"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "outline",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "outline",
            summary: "outline",
            description: "",
            documentation: r#"Sets outline intent.

Use `outline none`, a semantic color such as `outline accent`, or `outline offset small`.

Generated CSS writes `outline` or `outline-offset`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "overflow",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "overflow",
            summary: "overflow",
            description: "",
            documentation: "Controls overflow intent for panels and app shells. Use `overflow hidden` for clipped regions.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "padding",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "padding",
            summary: "padding",
            description: "",
            documentation: "Adds inner spacing using Frame spacing tokens.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "palette",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "palette",
            summary: "palette",
            description: "",
            documentation: "Selects a color palette intent for theming.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "place",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "place",
            summary: "Places an area into a named grid section.",
            description: "Places an `area` into a named grid section from its parent `grid`. Use this when the parent grid uses named columns or areas.",
            documentation: "",
            generated_css: Some("Emits `grid-area` or equivalent placement for the generated class."),
            frame_examples: &[r#"grid Dashboard {
  columns sidebar content inspector
}

area Sidebar {
  in Dashboard
  place sidebar
}"#],
            svelte_examples: &[r#"<aside class="fr-Sidebar">Channels</aside>"#],
            allowed_in: &[FrameScope::Area],
            related: &["in", "columns", "col", "row"],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/grid.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "play-state",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "play-state",
            summary: "play-state",
            description: "",
            documentation: "Controls whether an animation is `running` or `paused`.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "position",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "position",
            summary: "position",
            description: "",
            documentation: "Sets positioning intent such as relative, absolute, sticky, or fixed.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "radius",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "radius",
            summary: "radius",
            description: "",
            documentation: "Sets corner shape with named values like small, large, pill, or none.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "rows",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "rows",
            summary: "rows",
            description: "",
            documentation: r#"Defines the vertical sections of a grid.
Use rows for NavBars, page headers, content bands, and footers.

Generated CSS creates `grid-template-rows`.

Example:

```frame
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
}
```"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "scrollbar",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "scrollbar",
            summary: "scrollbar",
            description: "",
            documentation: "Sets scrollbar density for app panels. Use `scrollbar dense` for compact terminal-inspired surfaces.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "self",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "self",
            summary: "self",
            description: "",
            documentation: "Aligns this item within its parent in both axes. Use `self center` for centered modal panels.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "shadow",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "shadow",
            summary: "shadow",
            description: "",
            documentation: "Sets depth using named shadow values like soft, medium, or deep.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "shape",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "shape",
            summary: "shape",
            description: "",
            documentation: "Sets clip-path or shape intent.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "size",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "size",
            summary: "size",
            description: "",
            documentation: "Selects a typography size intent such as heading, body, or caption.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "span",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "span",
            summary: "span",
            description: "",
            documentation: r#"Makes an area span multiple grid tracks.
Use it for headers, footers, or wide content regions."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "square",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "square",
            summary: "square",
            description: "",
            documentation: "Applies a named equal width and height for icons, avatars, server buttons, and presence dots.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "surface",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "surface",
            summary: "Sets the visual surface of a component.",
            description: "Sets the visual background style for panels, pages, cards, sidebars, modals, and tool regions. Choose a named surface by role instead of writing raw background CSS.",
            documentation: "",
            generated_css: Some("Emits background and related surface variables."),
            frame_examples: &[r#"card ProjectCard {
  surface panel
  padding medium
}"#],
            svelte_examples: &[r#"<div class="fr-ProjectCard">Project details</div>"#],
            allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
            related: &["panel", "main", "glass", "shadow", "border"],
            values: &["main", "panel", "glass", "raised", "flat", "overlay", "inset", "sunken", "gradient", "gradient dusk"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/surfaces.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "theme",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "theme",
            summary: "theme",
            description: "",
            documentation: "Applies semantic color intent such as danger, success, or warning.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "tone",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "tone",
            summary: "tone",
            description: "",
            documentation: "Sets a semantic tone for interactive states such as danger, success, or warning.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "tracks",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "tracks",
            summary: "tracks",
            description: "",
            documentation: r#"Defines readable grid track sizes for app shells.
Use `tracks columns rail panel fill side` or `tracks rows header fill composer` instead of raw `grid-template` CSS."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "transform",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "transform",
            summary: "transform",
            description: "",
            documentation: r#"Animates transform functions in keyframes.

Generated CSS writes `transform: ...`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "truncate",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "truncate",
            summary: "truncate",
            description: "",
            documentation: "Keeps text on one line and adds ellipsis when it overflows. Use it for dense labels in sidebars and headers.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "type",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Motion,
            detail: "type",
            summary: "type",
            description: "",
            documentation: "Sets animation type or easing category.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::MotionProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "visibility",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Style,
            detail: "visibility",
            summary: "visibility",
            description: "",
            documentation: r#"Sets CSS visibility through structured values.

Use `visibility hidden` when the element should keep its layout slot but not render visibly."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::VisualProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "weight",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "weight",
            summary: "weight",
            description: "",
            documentation: "Selects type emphasis such as normal, semibold, or bold.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "whitespace",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "whitespace",
            summary: "whitespace",
            description: "",
            documentation: r#"Controls white-space preservation and wrapping.

Use `whitespace pre-wrap` for user-entered multiline text or `whitespace break-spaces` when spaces should be preserved.

Generated CSS writes `white-space`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "width",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "width",
            summary: "width",
            description: "",
            documentation: r#"Sets width intent with values such as fill, content, screen, sidebar, or percentages.
Generated CSS writes `width`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "word-break",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "word-break",
            summary: "word-break",
            description: "",
            documentation: r#"Controls how words break in narrow layouts.

Use `word-break break-word` for long unspaced content.

Generated CSS writes `word-break`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "wrap",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Typography,
            detail: "wrap",
            summary: "wrap",
            description: "",
            documentation: "Controls text wrapping. Use `wrap anywhere` for chat message bodies and narrow content.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::TypographyProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "z",
            kind: LanguageItemKind::Property,
            layer: LanguageLayer::Layout,
            detail: "z",
            summary: "z",
            description: "",
            documentation: "Sets stacking layer intent using named z-layers.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::LayoutProperty,
            semantic_class: SemanticTokenClass::Property,
        },
    LanguageItem {
            name: "33%",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "33%",
            summary: "33%",
            description: "",
            documentation: "Grid track or sizing value of 33%.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "66%",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "66%",
            summary: "66%",
            description: "",
            documentation: "Grid track or sizing value of 66%.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("above", Value, Style, Value, EnumMember),
    item!("absolute", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "accent",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "accent",
            summary: "accent",
            description: "",
            documentation: r#"Use accent for important interactive UI:
- primary buttons
- active nav items
- focus rings
- highlighted cards"#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("alternate", Value, Style, Value, EnumMember),
    item!("alternate-reverse", Value, Style, Value, EnumMember),
    item!("anywhere", Value, Style, Value, EnumMember),
    item!("around", Value, Style, Value, EnumMember),
    item!("aurora", Value, Style, Value, EnumMember),
    item!("auto", Value, Style, Value, EnumMember),
    item!("avatar-content", Value, Style, Value, EnumMember),
    item!("backwards", Value, Style, Value, EnumMember),
    item!("base", Value, Style, Value, EnumMember),
    item!("between", Value, Style, Value, EnumMember),
    item!("black", Value, Style, Value, EnumMember),
    item!("block", Value, Style, Value, EnumMember),
    item!("blue", Value, Style, Value, EnumMember),
    item!("body", Value, Style, Value, EnumMember),
    item!("bold", Value, Style, Value, EnumMember),
    item!("both", Value, Style, Value, EnumMember),
    item!("bottom", Value, Style, Value, EnumMember),
    item!("bottom-left", Value, Style, Value, EnumMember),
    item!("bottom-right", Value, Style, Value, EnumMember),
    item!("bounce", Value, Style, Value, EnumMember),
    item!("break-all", Value, Style, Value, EnumMember),
    item!("break-spaces", Value, Style, Value, EnumMember),
    item!("break-word", Value, Style, Value, EnumMember),
    item!("bright", Value, Style, Value, EnumMember),
    item!("capitalize", Value, Style, Value, EnumMember),
    item!("caption", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "cards",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "cards",
            summary: "cards",
            description: "",
            documentation: "Used with `columns responsive` to create an auto-fitting card grid.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("clip", Value, Style, Value, EnumMember),
    item!("collapse", Value, Style, Value, EnumMember),
    item!("column", Value, Style, Value, EnumMember),
    item!("column-reverse", Value, Style, Value, EnumMember),
    item!("conic", Value, Style, Value, EnumMember),
    item!("content", Value, Style, Value, EnumMember),
    item!("contents", Value, Style, Value, EnumMember),
    item!("cyan", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "danger",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "danger",
            summary: "danger",
            description: "",
            documentation: r#"Semantic color intent for destructive actions, errors, and dangerous status.
Use it for delete buttons, invalid states, and error badges."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("dashed", Value, Style, Value, EnumMember),
    item!("deep", Value, Style, Value, EnumMember),
    item!("dense", Value, Style, Value, EnumMember),
    item!("desktop", Value, Style, Value, EnumMember),
    item!("dotted", Value, Style, Value, EnumMember),
    item!("double", Value, Style, Value, EnumMember),
    item!("dramatic", Value, Style, Value, EnumMember),
    item!("dropdown", Value, Style, Value, EnumMember),
    item!("dusk", Value, Style, Value, EnumMember),
    item!("ember", Value, Style, Value, EnumMember),
    item!("end", Value, Style, Value, EnumMember),
    item!("evenly", Value, Style, Value, EnumMember),
    item!("fade-in", Value, Style, Value, EnumMember),
    item!("fast", Value, Style, Value, EnumMember),
    item!("fixed", Value, Style, Value, EnumMember),
    item!("flat", Value, Style, Value, EnumMember),
    item!("floating", Value, Style, Value, EnumMember),
    item!("footer", Value, Style, Value, EnumMember),
    item!("forest", Value, Style, Value, EnumMember),
    item!("forwards", Value, Style, Value, EnumMember),
    item!("full", Value, Style, Value, EnumMember),
    item!("glass", Value, Style, Value, EnumMember),
    item!("gradient aurora", Value, Style, Value, EnumMember),
    item!("gradient dusk", Value, Style, Value, EnumMember),
    item!("gradient ember", Value, Style, Value, EnumMember),
    item!("gradient forest", Value, Style, Value, EnumMember),
    item!("gradient midnight", Value, Style, Value, EnumMember),
    item!("gradient ocean", Value, Style, Value, EnumMember),
    item!("gray", Value, Style, Value, EnumMember),
    item!("green", Value, Style, Value, EnumMember),
    item!("groove", Value, Style, Value, EnumMember),
    item!("header", Value, Style, Value, EnumMember),
    item!("heading", Value, Style, Value, EnumMember),
    item!("horizontal", Value, Style, Value, EnumMember),
    item!("huge", Value, Style, Value, EnumMember),
    item!("icon-content-action", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "info",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "info",
            summary: "info",
            description: "",
            documentation: "Informational color intent for neutral notices, tips, and status messages.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("inline", Value, Style, Value, EnumMember),
    item!("inline-block", Value, Style, Value, EnumMember),
    item!("inline-flex", Value, Style, Value, EnumMember),
    item!("inline-grid", Value, Style, Value, EnumMember),
    item!("inset", Value, Style, Value, EnumMember),
    item!("inspector", Value, Style, Value, EnumMember),
    item!("keep-all", Value, Style, Value, EnumMember),
    item!("large", Value, Style, Value, EnumMember),
    item!("layered", Value, Style, Value, EnumMember),
    item!("line-through", Value, Style, Value, EnumMember),
    item!("linear", Value, Style, Value, EnumMember),
    item!("lowercase", Value, Style, Value, EnumMember),
    item!("main", Value, Style, Value, EnumMember),
    item!("manual", Value, Style, Value, EnumMember),
    item!("match-parent", Value, Style, Value, EnumMember),
    item!("medium", Value, Style, Value, EnumMember),
    item!("midnight", Value, Style, Value, EnumMember),
    item!("mobile", Value, Style, Value, EnumMember),
    item!("modal", Value, Style, Value, EnumMember),
    item!("mono", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "muted",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "muted",
            summary: "muted",
            description: "",
            documentation: r#"Semantic color intent for secondary text or subdued UI.
Use it for captions, helper text, and lower-priority metadata."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("narrow", Value, Style, Value, EnumMember),
    item!("none", Value, Style, Value, EnumMember),
    item!("normal", Value, Style, Value, EnumMember),
    item!("nowrap", Value, Style, Value, EnumMember),
    item!("ocean", Value, Style, Value, EnumMember),
    item!("orange", Value, Style, Value, EnumMember),
    item!("outset", Value, Style, Value, EnumMember),
    item!("overline", Value, Style, Value, EnumMember),
    item!("paused", Value, Style, Value, EnumMember),
    item!("pill", Value, Style, Value, EnumMember),
    item!("pink", Value, Style, Value, EnumMember),
    item!("pop-in", Value, Style, Value, EnumMember),
    item!("pre", Value, Style, Value, EnumMember),
    item!("pre-line", Value, Style, Value, EnumMember),
    item!("pre-wrap", Value, Style, Value, EnumMember),
    item!("presence", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "primary",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "primary",
            summary: "primary",
            description: "",
            documentation: "Primary color intent for the most important interactive elements and highlighted content.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("pulse", Value, Style, Value, EnumMember),
    item!("purple", Value, Style, Value, EnumMember),
    item!("radial", Value, Style, Value, EnumMember),
    item!("rail", Value, Style, Value, EnumMember),
    item!("raised", Value, Style, Value, EnumMember),
    item!("red", Value, Style, Value, EnumMember),
    item!("relative", Value, Style, Value, EnumMember),
    item!("relaxed", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "responsive",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "responsive",
            summary: "responsive",
            description: "",
            documentation: "Requests viewport-aware behavior, such as responsive card grids.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("reverse", Value, Style, Value, EnumMember),
    item!("ridge", Value, Style, Value, EnumMember),
    item!("row-reverse", Value, Style, Value, EnumMember),
    item!("running", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "secondary",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "secondary",
            summary: "secondary",
            description: "",
            documentation: "Secondary color intent for supporting actions and secondary emphasis.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("semibold", Value, Style, Value, EnumMember),
    item!("server", Value, Style, Value, EnumMember),
    item!("sharp", Value, Style, Value, EnumMember),
    item!("side", Value, Style, Value, EnumMember),
    item!("sidebar", Value, Style, Value, EnumMember),
    item!("slate", Value, Style, Value, EnumMember),
    item!("slide-up", Value, Style, Value, EnumMember),
    item!("slight", Value, Style, Value, EnumMember),
    item!("slow", Value, Style, Value, EnumMember),
    item!("small", Value, Style, Value, EnumMember),
    item!("soft", Value, Style, Value, EnumMember),
    item!("solid", Value, Style, Value, EnumMember),
    item!("start", Value, Style, Value, EnumMember),
    item!("sticky", Value, Style, Value, EnumMember),
    item!("stretch", Value, Style, Value, EnumMember),
    item!("strong", Value, Style, Value, EnumMember),
    item!("subtle", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "success",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "success",
            summary: "success",
            description: "",
            documentation: r#"Semantic color intent for successful or positive states.
Use it for completed tasks, saved states, and positive status."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("sunken", Value, Style, Value, EnumMember),
    item!("tablet", Value, Style, Value, EnumMember),
    item!("thin", Value, Style, Value, EnumMember),
    item!("tight", Value, Style, Value, EnumMember),
    item!("tiny", Value, Style, Value, EnumMember),
    item!("toast", Value, Style, Value, EnumMember),
    item!("top", Value, Style, Value, EnumMember),
    item!("top-left", Value, Style, Value, EnumMember),
    item!("top-right", Value, Style, Value, EnumMember),
    item!("translate", Value, Style, Value, EnumMember),
    item!("transparent", Value, Style, Value, EnumMember),
    item!("underline", Value, Style, Value, EnumMember),
    item!("unread", Value, Style, Value, EnumMember),
    item!("uppercase", Value, Style, Value, EnumMember),
    item!("vertical", Value, Style, Value, EnumMember),
    item!("visible", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "warning",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "warning",
            summary: "warning",
            description: "",
            documentation: r#"Semantic color intent for cautionary states.
Use it for warnings, pending work, and attention states."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    item!("white", Value, Style, Value, EnumMember),
    item!("wide", Value, Style, Value, EnumMember),
    item!("wrap-reverse", Value, Style, Value, EnumMember),
    item!("x", Value, Style, Value, EnumMember),
    item!("xlarge", Value, Style, Value, EnumMember),
    item!("y", Value, Style, Value, EnumMember),
    item!("yellow", Value, Style, Value, EnumMember),
    item!("zero", Value, Style, Value, EnumMember),
    LanguageItem {
            name: "0%",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "0%",
            summary: "0%",
            description: "",
            documentation: "Keyframe selector for 0% of the animation timeline.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "100%",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "100%",
            summary: "100%",
            description: "",
            documentation: "Keyframe selector for 100% of the animation timeline.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "25%",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "25%",
            summary: "25%",
            description: "",
            documentation: "Keyframe selector for 25% of the animation timeline.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "50%",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "50%",
            summary: "50%",
            description: "",
            documentation: "Keyframe selector for 50% of the animation timeline.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "75%",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "75%",
            summary: "75%",
            description: "",
            documentation: "Keyframe selector for 75% of the animation timeline.",
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "from",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "from",
            summary: "from",
            description: "",
            documentation: r#"Marks the initial state of an animation timeline.

Use `from` inside `@keyframes` blocks.

Generated CSS keeps the selector inside `@keyframes frame-Name`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "to",
            kind: LanguageItemKind::Special,
            layer: LanguageLayer::Motion,
            detail: "to",
            summary: "to",
            description: "",
            documentation: r#"Marks the final state of an animation timeline.

Use `to` inside `@keyframes` blocks.

Generated CSS keeps the selector inside `@keyframes frame-Name`."#,
            generated_css: None,
            frame_examples: &[],
            svelte_examples: &[],
            allowed_in: &[],
            related: &[],
            values: &[],
            aliases: &[],
            lowers_to: None,
            docs_anchor: None,
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::KeyframeSelector,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "below",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "below",
            summary: "Starts a max-width responsive override.",
            description: "Starts a responsive override for viewports below a named breakpoint. Use it when a layout should simplify on smaller screens.",
            documentation: "",
            generated_css: Some("Emits an `@media (max-width: ...)` rule for the current class."),
            frame_examples: &[r#"grid AppShell {
  columns sidebar content inspector

  below tablet {
    columns content
    rows sidebar content inspector
  }
}"#],
            svelte_examples: &[r#"<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>"#],
            allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
            related: &["above", "between", "columns", "rows"],
            values: &["mobile", "tablet", "desktop", "wide"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/grid.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
    LanguageItem {
            name: "container",
            kind: LanguageItemKind::Value,
            layer: LanguageLayer::Style,
            detail: "container",
            summary: "Starts a container query override.",
            description: "Starts a container query override so a component can adapt to the space it receives instead of the whole viewport.",
            documentation: "",
            generated_css: Some("Emits an `@container` rule for the current class."),
            frame_examples: &[r#"grid Cards {
  columns responsive cards

  container narrow {
    columns content
  }
}"#],
            svelte_examples: &[r#"<section class="fr-Cards">...</section>"#],
            allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
            related: &["below", "above", "columns"],
            values: &["narrow", "content", "wide"],
            aliases: &[],
            lowers_to: None,
            docs_anchor: Some("docs/grid.md"),
            status: LanguageItemStatus::Stable,
            completion_category: CompletionCategory::Value,
            semantic_class: SemanticTokenClass::EnumMember,
        },
];

static INDEX: OnceLock<HashMap<&'static str, &'static LanguageItem>> = OnceLock::new();

fn index() -> &'static HashMap<&'static str, &'static LanguageItem> {
    INDEX.get_or_init(|| {
        let mut map = HashMap::new();
        for item in REGISTRY {
            map.insert(item.name, item);
        }
        map
    })
}

pub fn item(name: &str) -> Option<&'static LanguageItem> {
    index().get(name).copied()
}

pub fn items_by_kind(kind: LanguageItemKind) -> &'static [LanguageItem] {
    static CACHE: OnceLock<HashMap<LanguageItemKind, &'static [LanguageItem]>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| {
        let mut map = HashMap::new();
        for k in [
            LanguageItemKind::Primitive,
            LanguageItemKind::Declaration,
            LanguageItemKind::Property,
            LanguageItemKind::Value,
            LanguageItemKind::Event,
            LanguageItemKind::EventModifier,
            LanguageItemKind::StateKeyword,
            LanguageItemKind::BindingKeyword,
            LanguageItemKind::UiKeyword,
            LanguageItemKind::IncludeKeyword,
            LanguageItemKind::Effect,
            LanguageItemKind::Special,
        ] {
            let items: Vec<LanguageItem> =
                REGISTRY.iter().filter(|i| i.kind == k).cloned().collect();
            let leaked = Box::leak(items.into_boxed_slice());
            map.insert(k, &*leaked);
        }
        map
    });
    cache.get(&kind).copied().unwrap_or(&[])
}

pub fn items_by_layer(layer: LanguageLayer) -> &'static [LanguageItem] {
    static CACHE: OnceLock<HashMap<LanguageLayer, &'static [LanguageItem]>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| {
        let mut map = HashMap::new();
        for l in [
            LanguageLayer::Ui,
            LanguageLayer::Style,
            LanguageLayer::Motion,
            LanguageLayer::Layout,
            LanguageLayer::Typography,
            LanguageLayer::Advanced,
            LanguageLayer::EscapeHatch,
        ] {
            let items: Vec<LanguageItem> =
                REGISTRY.iter().filter(|i| i.layer == l).cloned().collect();
            let leaked = Box::leak(items.into_boxed_slice());
            map.insert(l, &*leaked);
        }
        map
    });
    cache.get(&layer).copied().unwrap_or(&[])
}

pub fn items_by_status(status: LanguageItemStatus) -> &'static [LanguageItem] {
    static CACHE: OnceLock<HashMap<LanguageItemStatus, &'static [LanguageItem]>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| {
        let mut map = HashMap::new();
        for s in [
            LanguageItemStatus::Stable,
            LanguageItemStatus::Experimental,
            LanguageItemStatus::Deprecated,
            LanguageItemStatus::Advanced,
            LanguageItemStatus::EscapeHatch,
        ] {
            let items: Vec<LanguageItem> =
                REGISTRY.iter().filter(|i| i.status == s).cloned().collect();
            let leaked = Box::leak(items.into_boxed_slice());
            map.insert(s, &*leaked);
        }
        map
    });
    cache.get(&status).copied().unwrap_or(&[])
}

pub fn completion_doc(name: &str) -> Option<String> {
    item(name).and_then(|i| {
        let doc = i.markdown();
        if doc.is_empty() {
            None
        } else {
            Some(doc)
        }
    })
}

pub fn hover_doc_for(name: &str) -> Option<String> {
    item(name).and_then(|i| {
        let doc = i.markdown();
        if doc.is_empty() {
            None
        } else {
            Some(doc)
        }
    })
}

pub fn semantic_class_for(name: &str) -> Option<SemanticTokenClass> {
    item(name).map(|i| i.semantic_class)
}

pub fn declaration_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| {
                i.kind == LanguageItemKind::Declaration || i.kind == LanguageItemKind::Primitive
            })
            .map(|i| i.name)
            .collect();
        names.sort();
        names.dedup();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn property_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| {
                matches!(
                    i.kind,
                    LanguageItemKind::Property
                        | LanguageItemKind::Effect
                        | LanguageItemKind::Primitive
                        | LanguageItemKind::Declaration
                        | LanguageItemKind::Special
                ) || i.name == "in"
            })
            .map(|i| i.name)
            .collect();
        names.sort();
        names.dedup();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn ui_primitive_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::Primitive)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn event_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::Event)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn event_modifiers() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::EventModifier)
            .map(|i| i.name)
            .collect();
        // "shift" is also a valid event modifier (e.g. `keydown.shift.enter`)
        // even though its primary registry classification is Effect.
        names.push("shift");
        names.sort();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn state_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::StateKeyword)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn ui_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::UiKeyword)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn binding_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::BindingKeyword)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn value_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::Value)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn effect_keywords() -> &'static [&'static str] {
    static CACHE: OnceLock<&'static [&'static str]> = OnceLock::new();
    CACHE.get_or_init(|| {
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::Effect)
            .map(|i| i.name)
            .collect();
        Box::leak(names.into_boxed_slice())
    })
}

pub fn is_ui_primitive(name: &str) -> bool {
    item(name).is_some_and(|i| i.kind == LanguageItemKind::Primitive)
}

pub fn is_known_value(name: &str) -> bool {
    item(name)
        .is_some_and(|i| i.kind == LanguageItemKind::Value || i.kind == LanguageItemKind::Effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_lookup_by_name() {
        assert!(item("grid").is_some());
        assert!(item("card").is_some());
        assert!(item("hover").is_some());
        assert!(item("lift").is_some());
        assert!(item("small").is_some());
        assert!(item("not-a-real-item").is_none());
    }

    #[test]
    fn lookup_by_kind() {
        let decls = items_by_kind(LanguageItemKind::Declaration);
        assert!(!decls.is_empty());
        assert!(decls.iter().any(|i| i.name == "grid"));
        let props = items_by_kind(LanguageItemKind::Property);
        assert!(!props.is_empty());
        assert!(props.iter().any(|i| i.name == "surface"));
    }

    #[test]
    fn no_duplicate_canonical_names() {
        let mut seen = std::collections::HashSet::new();
        for item in REGISTRY {
            assert!(
                seen.insert(item.name),
                "duplicate canonical name: {}",
                item.name
            );
        }
    }

    #[test]
    fn coverage_check_old_tokens() {
        // Verify every old tokens.rs constant is represented in the registry
        for &name in SPACING {
            assert!(item(name).is_some(), "missing spacing: {name}");
        }
        for &name in COLORS {
            assert!(item(name).is_some(), "missing color: {name}");
        }
        for &name in EFFECTS {
            assert!(item(name).is_some(), "missing effect: {name}");
        }
        for &name in SURFACES {
            assert!(item(name).is_some(), "missing surface: {name}");
        }
        for &name in RADII {
            assert!(item(name).is_some(), "missing radius: {name}");
        }
        for &name in SHADOWS {
            assert!(item(name).is_some(), "missing shadow: {name}");
        }
        for &name in BREAKPOINTS {
            assert!(item(name).is_some(), "missing breakpoint: {name}");
        }
        for &name in DISPLAY {
            assert!(item(name).is_some(), "missing display: {name}");
        }
    }

    #[test]
    fn helper_functions_work() {
        assert!(is_ui_primitive("action"));
        assert!(!is_ui_primitive("grid"));
        assert!(is_known_value("small"));
        assert!(declaration_keywords().contains(&"grid"));
        assert!(property_keywords().contains(&"surface"));
        assert!(state_keywords().contains(&"hover"));
        assert!(effect_keywords().contains(&"press"));
        assert!(event_modifiers().contains(&"enter"));
        assert!(binding_keywords().contains(&"bind"));
        assert!(effect_keywords().contains(&"lift"));
    }
}

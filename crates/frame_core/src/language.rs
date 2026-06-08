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
    pub status: LanguageItemStatus,
    pub completion_category: CompletionCategory,
    pub semantic_class: SemanticTokenClass,
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
    "screen", "fill", "content", "auto", "sidebar", "narrow", "wide", "small", "medium", "large",
    "xlarge", "zero", "modal", "icon",
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

pub static REGISTRY: &[LanguageItem] = &[
    item!("area", Declaration, Layout, Declaration, Keyword),
    item!("button", Declaration, Layout, Declaration, Keyword),
    item!("card", Declaration, Layout, Declaration, Keyword),
    item!("center", Declaration, Layout, Declaration, Keyword),
    item!("component", Declaration, Layout, Declaration, Keyword),
    item!("dock", Declaration, Layout, Declaration, Keyword),
    item!("grid", Declaration, Layout, Declaration, Keyword),
    item!("keyframes", Declaration, Layout, Declaration, Keyword),
    item!("overlay", Declaration, Layout, Declaration, Keyword),
    item!("row", Declaration, Layout, Declaration, Keyword),
    item!("split", Declaration, Layout, Declaration, Keyword),
    item!("stack", Declaration, Layout, Declaration, Keyword),
    item!("style-group", Declaration, Layout, Declaration, Keyword),
    item!("style-order", Declaration, Layout, Declaration, Keyword),
    item!("supports", Declaration, Layout, Declaration, Keyword),
    item!("text", Declaration, Layout, Declaration, Keyword),
    item!("tokens", Declaration, Layout, Declaration, Keyword),
    item!("action", Primitive, Ui, Declaration, Keyword),
    item!("avatar", Primitive, Ui, Declaration, Keyword),
    item!("badge", Primitive, Ui, Declaration, Keyword),
    item!("choice", Primitive, Ui, Declaration, Keyword),
    item!("composer", Primitive, Ui, Declaration, Keyword),
    item!("data", Primitive, Ui, Declaration, Keyword),
    item!("dialog", Primitive, Ui, Declaration, Keyword),
    item!("editor", Primitive, Ui, Declaration, Keyword),
    item!("empty", Primitive, Ui, Declaration, Keyword),
    item!("feed", Primitive, Ui, Declaration, Keyword),
    item!("field", Primitive, Ui, Declaration, Keyword),
    item!("icon", Primitive, Ui, Declaration, Keyword),
    item!("image", Primitive, Ui, Declaration, Keyword),
    item!("input", Primitive, Ui, Declaration, Keyword),
    item!("item", Primitive, Ui, Declaration, Keyword),
    item!("label", Primitive, Ui, Declaration, Keyword),
    item!("link", Primitive, Ui, Declaration, Keyword),
    item!("list", Primitive, Ui, Declaration, Keyword),
    item!("media", Primitive, Ui, Declaration, Keyword),
    item!("menu", Primitive, Ui, Declaration, Keyword),
    item!("panel", Primitive, Ui, Declaration, Keyword),
    item!("popover", Primitive, Ui, Declaration, Keyword),
    item!("screen", Primitive, Ui, Declaration, Keyword),
    item!("scroll", Primitive, Ui, Declaration, Keyword),
    item!("section", Primitive, Ui, Declaration, Keyword),
    item!("select", Primitive, Ui, Declaration, Keyword),
    item!("tabs", Primitive, Ui, Declaration, Keyword),
    item!("title", Primitive, Ui, Declaration, Keyword),
    item!("toggle", Primitive, Ui, Declaration, Keyword),
    item!("toolbar", Primitive, Ui, Declaration, Keyword),
    item!("active", StateKeyword, Ui, StateBlock, Keyword),
    item!("checked", StateKeyword, Ui, StateBlock, Keyword),
    item!("disabled", StateKeyword, Ui, StateBlock, Keyword),
    item!("focus", StateKeyword, Ui, StateBlock, Keyword),
    item!("focus-visible", StateKeyword, Ui, StateBlock, Keyword),
    item!("focus-within", StateKeyword, Ui, StateBlock, Keyword),
    item!("hover", StateKeyword, Ui, StateBlock, Keyword),
    item!("invalid", StateKeyword, Ui, StateBlock, Keyword),
    item!("required", StateKeyword, Ui, StateBlock, Keyword),
    item!("target", StateKeyword, Ui, StateBlock, Keyword),
    item!("blur", Effect, Motion, MotionProperty, Property),
    item!("change", Event, Ui, Value, EnumMember),
    item!("click", Event, Ui, Value, EnumMember),
    item!("close", Event, Ui, Value, EnumMember),
    item!("keydown", Event, Ui, Value, EnumMember),
    item!("keyup", Event, Ui, Value, EnumMember),
    item!("mouseenter", Event, Ui, Value, EnumMember),
    item!("mouseleave", Event, Ui, Value, EnumMember),
    item!("open", Event, Ui, Value, EnumMember),
    item!("pointerdown", Event, Ui, Value, EnumMember),
    item!("pointermove", Event, Ui, Value, EnumMember),
    item!("pointerup", Event, Ui, Value, EnumMember),
    item!("press", Effect, Motion, MotionProperty, Property),
    item!("reset", Event, Ui, Value, EnumMember),
    item!("send", Event, Ui, Value, EnumMember),
    item!("submit", Event, Ui, Value, EnumMember),
    item!("alt", EventModifier, Ui, Value, EnumMember),
    item!("capture", EventModifier, Ui, Value, EnumMember),
    item!("ctrl", EventModifier, Ui, Value, EnumMember),
    item!("down", EventModifier, Ui, Value, EnumMember),
    item!("enter", EventModifier, Ui, Value, EnumMember),
    item!("escape", EventModifier, Ui, Value, EnumMember),
    item!("left", EventModifier, Ui, Value, EnumMember),
    item!("meta", EventModifier, Ui, Value, EnumMember),
    item!("once", EventModifier, Ui, Value, EnumMember),
    item!("passive", EventModifier, Ui, Value, EnumMember),
    item!("prevent", EventModifier, Ui, Value, EnumMember),
    item!("right", EventModifier, Ui, Value, EnumMember),
    item!("shift", Effect, Motion, MotionProperty, Property),
    item!("space", EventModifier, Ui, Value, EnumMember),
    item!("stop", EventModifier, Ui, Value, EnumMember),
    item!("tab", EventModifier, Ui, Value, EnumMember),
    item!("up", EventModifier, Ui, Value, EnumMember),
    item!("class", UiKeyword, Ui, Value, EnumMember),
    item!("data-test-id", UiKeyword, Ui, Value, EnumMember),
    item!("decorative", UiKeyword, Ui, Value, EnumMember),
    item!("description", UiKeyword, Ui, Value, EnumMember),
    item!("download", UiKeyword, Ui, Value, EnumMember),
    item!("draft", UiKeyword, Ui, Value, EnumMember),
    item!("for", UiKeyword, Ui, Value, EnumMember),
    item!("goto", UiKeyword, Ui, Value, EnumMember),
    item!("hidden", UiKeyword, Ui, Value, EnumMember),
    item!("hint", UiKeyword, Ui, Value, EnumMember),
    item!("id", UiKeyword, Ui, Value, EnumMember),
    item!("in", UiKeyword, Ui, Value, EnumMember),
    item!("key", UiKeyword, Ui, Value, EnumMember),
    item!("kind", UiKeyword, Ui, Value, EnumMember),
    item!("new-window", UiKeyword, Ui, Value, EnumMember),
    item!("on", UiKeyword, Ui, Value, EnumMember),
    item!("options", UiKeyword, Ui, Value, EnumMember),
    item!("placeholder", UiKeyword, Ui, Value, EnumMember),
    item!("poster", UiKeyword, Ui, Value, EnumMember),
    item!("props", UiKeyword, Ui, Value, EnumMember),
    item!("readonly", UiKeyword, Ui, Value, EnumMember),
    item!("selected", UiKeyword, Ui, Value, EnumMember),
    item!("show", UiKeyword, Ui, Value, EnumMember),
    item!("slot", UiKeyword, Ui, Value, EnumMember),
    item!("source", UiKeyword, Ui, Value, EnumMember),
    item!("sources", UiKeyword, Ui, Value, EnumMember),
    item!("state", UiKeyword, Ui, Value, EnumMember),
    item!("style", UiKeyword, Ui, Value, EnumMember),
    item!("value", UiKeyword, Ui, Value, EnumMember),
    item!("view", UiKeyword, Ui, Value, EnumMember),
    item!("bind", BindingKeyword, Ui, Value, Keyword),
    item!("when", BindingKeyword, Ui, Value, Keyword),
    item!("animate", Effect, Motion, MotionProperty, Property),
    item!("animation", Effect, Motion, MotionProperty, Property),
    item!("brighten", Effect, Motion, MotionProperty, Property),
    item!("dim", Effect, Motion, MotionProperty, Property),
    item!("duration", Effect, Motion, MotionProperty, Property),
    item!("ease", Effect, Motion, MotionProperty, Property),
    item!("fade", Effect, Motion, MotionProperty, Property),
    item!("glow", Effect, Motion, MotionProperty, Property),
    item!("grow", Effect, Motion, MotionProperty, Property),
    item!("lift", Effect, Motion, MotionProperty, Property),
    item!("pop", Effect, Motion, MotionProperty, Property),
    item!("ring", Effect, Motion, MotionProperty, Property),
    item!("rotate", Effect, Motion, MotionProperty, Property),
    item!("scale", Effect, Motion, MotionProperty, Property),
    item!("shrink", Effect, Motion, MotionProperty, Property),
    item!("sink", Effect, Motion, MotionProperty, Property),
    item!("slide", Effect, Motion, MotionProperty, Property),
    item!("smooth", Effect, Motion, MotionProperty, Property),
    item!("tilt", Effect, Motion, MotionProperty, Property),
    item!("transition", Effect, Motion, MotionProperty, Property),
    item!("advanced", Property, Advanced, Value, Property),
    item!("align", Property, Layout, LayoutProperty, Property),
    item!(
        "align-text",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("anchor", Property, Layout, LayoutProperty, Property),
    item!("angle", Property, Motion, MotionProperty, Property),
    item!("areas", Property, Layout, LayoutProperty, Property),
    item!("at", Property, Motion, MotionProperty, Property),
    item!("background", Property, Style, VisualProperty, Property),
    item!("block-size", Property, Layout, LayoutProperty, Property),
    item!("border", Property, Style, VisualProperty, Property),
    item!("box", Property, Layout, LayoutProperty, Property),
    item!("case", Property, Typography, TypographyProperty, Property),
    item!("col", Property, Layout, LayoutProperty, Property),
    item!("color", Property, Style, VisualProperty, Property),
    item!("columns", Property, Layout, LayoutProperty, Property),
    item!("control", Property, Style, VisualProperty, Property),
    item!("corner", Property, Motion, MotionProperty, Property),
    item!("css", Property, Advanced, AdvancedProperty, Property),
    item!(
        "decoration",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("delay", Property, Motion, MotionProperty, Property),
    item!("direction", Property, Motion, MotionProperty, Property),
    item!("display", Property, Layout, LayoutProperty, Property),
    item!("fill", Property, Motion, MotionProperty, Property),
    item!("filter", Property, Motion, MotionProperty, Property),
    item!("flex", Property, Layout, LayoutProperty, Property),
    item!("flow", Property, Layout, LayoutProperty, Property),
    item!("font", Property, Typography, TypographyProperty, Property),
    item!("gap", Property, Layout, LayoutProperty, Property),
    item!("gradient", Property, Style, VisualProperty, Property),
    item!("height", Property, Layout, LayoutProperty, Property),
    item!(
        "hyphenate",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("inline-size", Property, Layout, LayoutProperty, Property),
    item!("interactive", Property, Style, VisualProperty, Property),
    item!("iteration", Property, Motion, MotionProperty, Property),
    item!("justify", Property, Layout, LayoutProperty, Property),
    item!("layout", Property, Layout, LayoutProperty, Property),
    item!("letter", Property, Typography, TypographyProperty, Property),
    item!("line", Property, Typography, TypographyProperty, Property),
    item!("margin", Property, Layout, LayoutProperty, Property),
    item!("max-block-size", Property, Layout, LayoutProperty, Property),
    item!("max-height", Property, Layout, LayoutProperty, Property),
    item!(
        "max-inline-size",
        Property,
        Layout,
        LayoutProperty,
        Property
    ),
    item!("max-width", Property, Layout, LayoutProperty, Property),
    item!("min-block-size", Property, Layout, LayoutProperty, Property),
    item!("min-height", Property, Layout, LayoutProperty, Property),
    item!(
        "min-inline-size",
        Property,
        Layout,
        LayoutProperty,
        Property
    ),
    item!("min-width", Property, Layout, LayoutProperty, Property),
    item!("nudge", Property, Layout, LayoutProperty, Property),
    item!("offset", Property, Layout, LayoutProperty, Property),
    item!("opacity", Property, Style, VisualProperty, Property),
    item!("outline", Property, Style, VisualProperty, Property),
    item!("overflow", Property, Layout, LayoutProperty, Property),
    item!("padding", Property, Layout, LayoutProperty, Property),
    item!("palette", Property, Style, VisualProperty, Property),
    item!("place", Property, Layout, LayoutProperty, Property),
    item!("play-state", Property, Motion, MotionProperty, Property),
    item!("position", Property, Layout, LayoutProperty, Property),
    item!("radius", Property, Style, VisualProperty, Property),
    item!("rows", Property, Layout, LayoutProperty, Property),
    item!("scrollbar", Property, Layout, LayoutProperty, Property),
    item!("self", Property, Layout, LayoutProperty, Property),
    item!("shadow", Property, Style, VisualProperty, Property),
    item!("shape", Property, Motion, MotionProperty, Property),
    item!("size", Property, Typography, TypographyProperty, Property),
    item!("span", Property, Layout, LayoutProperty, Property),
    item!("square", Property, Layout, LayoutProperty, Property),
    item!("surface", Property, Style, VisualProperty, Property),
    item!("theme", Property, Style, VisualProperty, Property),
    item!("tone", Property, Style, VisualProperty, Property),
    item!("tracks", Property, Layout, LayoutProperty, Property),
    item!("transform", Property, Motion, MotionProperty, Property),
    item!(
        "truncate",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("type", Property, Motion, MotionProperty, Property),
    item!("visibility", Property, Style, VisualProperty, Property),
    item!("weight", Property, Typography, TypographyProperty, Property),
    item!(
        "whitespace",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("width", Property, Layout, LayoutProperty, Property),
    item!(
        "word-break",
        Property,
        Typography,
        TypographyProperty,
        Property
    ),
    item!("wrap", Property, Typography, TypographyProperty, Property),
    item!("z", Property, Layout, LayoutProperty, Property),
    item!("33%", Value, Style, Value, EnumMember),
    item!("66%", Value, Style, Value, EnumMember),
    item!("above", Value, Style, Value, EnumMember),
    item!("absolute", Value, Style, Value, EnumMember),
    item!("accent", Value, Style, Value, EnumMember),
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
    item!("cards", Value, Style, Value, EnumMember),
    item!("clip", Value, Style, Value, EnumMember),
    item!("collapse", Value, Style, Value, EnumMember),
    item!("column", Value, Style, Value, EnumMember),
    item!("column-reverse", Value, Style, Value, EnumMember),
    item!("conic", Value, Style, Value, EnumMember),
    item!("content", Value, Style, Value, EnumMember),
    item!("contents", Value, Style, Value, EnumMember),
    item!("cyan", Value, Style, Value, EnumMember),
    item!("danger", Value, Style, Value, EnumMember),
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
    item!("info", Value, Style, Value, EnumMember),
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
    item!("muted", Value, Style, Value, EnumMember),
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
    item!("primary", Value, Style, Value, EnumMember),
    item!("pulse", Value, Style, Value, EnumMember),
    item!("purple", Value, Style, Value, EnumMember),
    item!("radial", Value, Style, Value, EnumMember),
    item!("rail", Value, Style, Value, EnumMember),
    item!("raised", Value, Style, Value, EnumMember),
    item!("red", Value, Style, Value, EnumMember),
    item!("relative", Value, Style, Value, EnumMember),
    item!("relaxed", Value, Style, Value, EnumMember),
    item!("responsive", Value, Style, Value, EnumMember),
    item!("reverse", Value, Style, Value, EnumMember),
    item!("ridge", Value, Style, Value, EnumMember),
    item!("row-reverse", Value, Style, Value, EnumMember),
    item!("running", Value, Style, Value, EnumMember),
    item!("secondary", Value, Style, Value, EnumMember),
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
    item!("success", Value, Style, Value, EnumMember),
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
    item!("warning", Value, Style, Value, EnumMember),
    item!("white", Value, Style, Value, EnumMember),
    item!("wide", Value, Style, Value, EnumMember),
    item!("wrap-reverse", Value, Style, Value, EnumMember),
    item!("x", Value, Style, Value, EnumMember),
    item!("xlarge", Value, Style, Value, EnumMember),
    item!("y", Value, Style, Value, EnumMember),
    item!("yellow", Value, Style, Value, EnumMember),
    item!("zero", Value, Style, Value, EnumMember),
    item!("0%", Special, Motion, KeyframeSelector, EnumMember),
    item!("100%", Special, Motion, KeyframeSelector, EnumMember),
    item!("25%", Special, Motion, KeyframeSelector, EnumMember),
    item!("50%", Special, Motion, KeyframeSelector, EnumMember),
    item!("75%", Special, Motion, KeyframeSelector, EnumMember),
    item!("from", Special, Motion, KeyframeSelector, EnumMember),
    item!("to", Special, Motion, KeyframeSelector, EnumMember),
    item!("below", Value, Style, Value, EnumMember),
    item!("container", Value, Style, Value, EnumMember),
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
        let doc = i.documentation;
        if doc.is_empty() {
            None
        } else {
            Some(doc.to_string())
        }
    })
}

pub fn hover_doc_for(name: &str) -> Option<String> {
    item(name).and_then(|i| {
        let doc = i.documentation;
        if doc.is_empty() {
            None
        } else {
            Some(doc.to_string())
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
        let names: Vec<&str> = REGISTRY
            .iter()
            .filter(|i| i.kind == LanguageItemKind::EventModifier)
            .map(|i| i.name)
            .collect();
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

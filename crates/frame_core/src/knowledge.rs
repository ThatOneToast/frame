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

pub fn concept(name: &str) -> Option<&'static FrameConcept> {
    CONCEPTS.iter().find(|concept| concept.name == name)
}

pub fn completion_doc(name: &str) -> Option<String> {
    concept(name).map(FrameConcept::markdown)
}

pub fn declaration_keywords() -> &'static [&'static str] {
    &[
        "tokens",
        "grid",
        "area",
        "card",
        "stack",
        "row",
        "button",
        "text",
        "center",
        "split",
        "overlay",
        "dock",
        "keyframes",
    ]
}

pub fn property_keywords() -> &'static [&'static str] {
    &[
        "columns",
        "rows",
        "tracks",
        "areas",
        "flow",
        "section",
        "layout",
        "display",
        "gap",
        "place",
        "in",
        "col",
        "row",
        "span",
        "surface",
        "background",
        "theme",
        "text",
        "color",
        "palette",
        "tone",
        "opacity",
        "padding",
        "anchor",
        "margin",
        "radius",
        "border",
        "overflow",
        "scroll",
        "scrollbar",
        "shadow",
        "outline",
        "box",
        "visibility",
        "flex",
        "square",
        "width",
        "height",
        "inline-size",
        "block-size",
        "min-inline-size",
        "max-inline-size",
        "min-block-size",
        "max-block-size",
        "min-width",
        "max-width",
        "min-height",
        "max-height",
        "align",
        "justify",
        "self",
        "nudge",
        "truncate",
        "wrap",
        "case",
        "align-text",
        "control",
        "interactive",
        "text",
        "color",
        "theme",
        "background",
        "position",
        "z",
        "position",
        "offset",
        "z",
        "font",
        "size",
        "weight",
        "line",
        "letter",
        "lift",
        "glow",
        "brighten",
        "dim",
        "blur",
        "press",
        "ring",
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
        "keyframes",
        "below",
        "above",
        "between",
        "container",
        "delay",
        "iteration",
        "direction",
        "fill",
        "play-state",
        "from",
        "to",
        "opacity",
        "transform",
        "filter",
        "type",
        "angle",
        "stop",
        "corner",
        "at",
        "shape",
        "css",
    ]
}

const GRID_FRAME: &str =
    "grid Dashboard {\n  columns sidebar content inspector\n  gap medium\n  height screen\n}";
const GRID_SVELTE: &str = "<div class=\"fr-Dashboard\">\n  <aside class=\"fr-Sidebar\">Channels</aside>\n  <main class=\"fr-Content\">Messages</main>\n  <section class=\"fr-Inspector\">Details</section>\n</div>";
const AREA_FRAME: &str =
    "area Sidebar {\n  in Dashboard\n  place sidebar\n  surface panel\n  padding medium\n}";
const CARD_FRAME: &str = "card HoverCard {\n  surface gradient dusk\n  padding large\n  radius large\n  shadow medium\n\n  hover {\n    lift small\n    glow accent\n  }\n}";
const TOOLBAR_FRAME: &str = "row Toolbar {\n  align center\n  justify between\n  gap small\n  padding medium\n  surface panel\n}";
const TOOLBAR_SVELTE: &str =
    "<div class=\"fr-Toolbar\">\n  <button>Back</button>\n  <button>Save</button>\n</div>";
const KEYFRAMES_FRAME: &str = "keyframes FloatIn {\n  from {\n    opacity 0\n    transform translateY(12px) scale(0.98)\n  }\n\n  to {\n    opacity 1\n    transform translateY(0) scale(1)\n  }\n}";
const RESPONSIVE_FRAME: &str = "grid AppShell {\n  columns sidebar content inspector\n\n  below tablet {\n    columns content\n    rows sidebar content inspector\n  }\n}";
const CONTAINER_FRAME: &str =
    "grid Cards {\n  columns responsive cards\n\n  container narrow {\n    columns content\n  }\n}";

pub static CONCEPTS: &[FrameConcept] = &[
    FrameConcept {
        name: "grid",
        kind: ConceptKind::Declaration,
        summary: "Defines a layout container.",
        description: "Defines a layout container using Frame's grid system. Use `columns`, `rows`, `gap`, and child `area` declarations to create app shells, dashboards, sidebars, inspectors, and card grids. Inline Svelte components can use it inside `<style lang=\"frame\">`.",
        generated_css: Some("Emits `display: grid` with readable grid template rules."),
        frame_examples: &[GRID_FRAME],
        svelte_examples: &[GRID_SVELTE],
        allowed_in: &[FrameScope::Root],
        related: &["area", "columns", "rows", "place", "col"],
        values: &[],
        docs_anchor: Some("docs/grid.md"),
    },
    FrameConcept {
        name: "area",
        kind: ConceptKind::Declaration,
        summary: "Defines a child region inside a grid.",
        description: "Defines a child region inside a named `grid`. Use `in` to reference the parent grid and `place`, `col`, or `row` to claim space.",
        generated_css: Some("Emits grid placement rules for the generated class."),
        frame_examples: &[AREA_FRAME],
        svelte_examples: &["<aside class=\"fr-Sidebar\">Channels</aside>"],
        allowed_in: &[FrameScope::Root],
        related: &["grid", "in", "place", "col", "row"],
        values: &[],
        docs_anchor: Some("docs/areas.md"),
    },
    FrameConcept {
        name: "card",
        kind: ConceptKind::Declaration,
        summary: "Defines a reusable content surface.",
        description: "Defines a reusable content surface for panels, links, tiles, settings sections, and interactive cards. Cards commonly combine `surface`, `padding`, `radius`, `shadow`, and state effects.",
        generated_css: Some("Emits a component class with background, spacing, shape, and interaction rules."),
        frame_examples: &[CARD_FRAME],
        svelte_examples: &["<a class=\"fr-HoverCard\">Open project</a>"],
        allowed_in: &[FrameScope::Root],
        related: &["surface", "padding", "radius", "shadow", "hover"],
        values: &[],
        docs_anchor: Some("docs/cards.md"),
    },
    FrameConcept {
        name: "row",
        kind: ConceptKind::Declaration,
        summary: "Defines a horizontal layout group.",
        description: "As a declaration, creates a horizontal layout for toolbars, NavBars, button groups, and header rows. As an area property, `row` places an area in a numeric grid row.",
        generated_css: Some("Declaration output uses flex row layout; area placement output uses grid row placement."),
        frame_examples: &[TOOLBAR_FRAME],
        svelte_examples: &[TOOLBAR_SVELTE],
        allowed_in: &[FrameScope::Root, FrameScope::Area],
        related: &["stack", "align", "justify", "gap"],
        values: &["1", "2", "3", "all"],
        docs_anchor: Some("docs/layout.md"),
    },
    FrameConcept {
        name: "stack",
        kind: ConceptKind::Declaration,
        summary: "Defines a vertical layout group.",
        description: "Defines a vertical layout group. Use `gap`, `align`, `padding`, and `surface` for panels, forms, and settings layouts.",
        generated_css: Some("Emits a column-oriented layout class."),
        frame_examples: &["stack SettingsPanel {\n  gap medium\n  padding large\n  surface panel\n}"],
        svelte_examples: &["<section class=\"fr-SettingsPanel\">...</section>"],
        allowed_in: &[FrameScope::Root],
        related: &["row", "gap", "align"],
        values: &[],
        docs_anchor: Some("docs/layout.md"),
    },
    FrameConcept {
        name: "columns",
        kind: ConceptKind::Property,
        summary: "Defines horizontal sections of a grid.",
        description: "Defines horizontal sections of a `grid`. Use named sections for readable `place` references, percentages for explicit ratios, or `responsive cards` for card grids.",
        generated_css: Some("Emits `grid-template-columns`; named columns also create named grid areas."),
        frame_examples: &["columns sidebar content inspector\ncolumns 25% 50% 25%\ncolumns responsive cards"],
        svelte_examples: &[GRID_SVELTE],
        allowed_in: &[FrameScope::Grid],
        related: &["grid", "area", "place", "col"],
        values: &["sidebar", "content", "inspector", "25%", "50%", "responsive cards"],
        docs_anchor: Some("docs/grid.md"),
    },
    FrameConcept {
        name: "place",
        kind: ConceptKind::Property,
        summary: "Places an area into a named grid section.",
        description: "Places an `area` into a named grid section from its parent `grid`. Use this when the parent grid uses named columns or areas.",
        generated_css: Some("Emits `grid-area` or equivalent placement for the generated class."),
        frame_examples: &["grid Dashboard {\n  columns sidebar content inspector\n}\n\narea Sidebar {\n  in Dashboard\n  place sidebar\n}"],
        svelte_examples: &["<aside class=\"fr-Sidebar\">Channels</aside>"],
        allowed_in: &[FrameScope::Area],
        related: &["in", "columns", "col", "row"],
        values: &[],
        docs_anchor: Some("docs/grid.md"),
    },
    FrameConcept {
        name: "surface",
        kind: ConceptKind::Property,
        summary: "Sets the visual surface of a component.",
        description: "Sets the visual background style for panels, pages, cards, sidebars, modals, and tool regions. Choose a named surface by role instead of writing raw background CSS.",
        generated_css: Some("Emits background and related surface variables."),
        frame_examples: &["card ProjectCard {\n  surface panel\n  padding medium\n}"],
        svelte_examples: &["<div class=\"fr-ProjectCard\">Project details</div>"],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["panel", "main", "glass", "shadow", "border"],
        values: &["main", "panel", "glass", "raised", "flat", "gradient dusk"],
        docs_anchor: Some("docs/surfaces.md"),
    },
    FrameConcept {
        name: "surface panel",
        kind: ConceptKind::Value,
        summary: "A secondary UI surface.",
        description: "A secondary UI surface for sidebars, inspectors, cards, menus, and tool panels. It usually separates a region from the main page background.",
        generated_css: Some("Uses the panel surface token, currently `var(--frame-surface-panel)`."),
        frame_examples: &[AREA_FRAME],
        svelte_examples: &["<aside class=\"fr-Sidebar\">Channels</aside>"],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["surface main", "surface glass", "shadow", "border"],
        values: &[],
        docs_anchor: Some("docs/surfaces.md"),
    },
    FrameConcept {
        name: "surface main",
        kind: ConceptKind::Value,
        summary: "The primary content surface.",
        description: "The primary page or content background. Use it for main content regions, large pages, and app shell content areas.",
        generated_css: Some("Uses the main surface token, currently `var(--frame-surface-main)`."),
        frame_examples: &["area Content {\n  in Dashboard\n  place content\n  surface main\n  padding large\n}"],
        svelte_examples: &["<main class=\"fr-Content\">Messages</main>"],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["surface panel", "surface glass"],
        values: &[],
        docs_anchor: Some("docs/surfaces.md"),
    },
    FrameConcept {
        name: "surface glass",
        kind: ConceptKind::Value,
        summary: "A translucent elevated surface.",
        description: "A translucent surface for overlays, command palettes, floating panels, and modals.",
        generated_css: Some("Uses the glass surface token and backdrop-oriented styling where supported."),
        frame_examples: &["overlay ModalLayer {\n  surface glass\n  padding large\n}"],
        svelte_examples: &["<div class=\"fr-ModalLayer\">...</div>"],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["overlay", "surface panel", "blur"],
        values: &[],
        docs_anchor: Some("docs/surfaces.md"),
    },
    FrameConcept {
        name: "align",
        kind: ConceptKind::Property,
        summary: "Controls cross-axis placement.",
        description: "Controls cross-axis placement. In a `row`, this usually means vertical alignment. In a `stack`, it usually means horizontal alignment.",
        generated_css: Some("Emits `align-items` or an equivalent placement rule."),
        frame_examples: &[TOOLBAR_FRAME],
        svelte_examples: &[TOOLBAR_SVELTE],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["justify", "row", "stack"],
        values: &["start", "center", "end", "stretch"],
        docs_anchor: Some("docs/layout.md"),
    },
    FrameConcept {
        name: "justify",
        kind: ConceptKind::Property,
        summary: "Controls main-axis placement.",
        description: "Controls main-axis placement and distribution. In a `row`, this usually means horizontal distribution.",
        generated_css: Some("Emits `justify-content` or an equivalent placement rule."),
        frame_examples: &[TOOLBAR_FRAME],
        svelte_examples: &[TOOLBAR_SVELTE],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["align", "row", "stack"],
        values: &["start", "center", "end", "between", "around", "evenly"],
        docs_anchor: Some("docs/layout.md"),
    },
    FrameConcept {
        name: "hover",
        kind: ConceptKind::State,
        summary: "Defines hover effects.",
        description: "Defines effects applied when the pointer hovers this component. Keep this block focused on interaction effects like `lift`, `glow`, and `brighten`.",
        generated_css: Some("Emits `:hover` rules for the generated class."),
        frame_examples: &["hover {\n  lift small\n  glow accent\n}"],
        svelte_examples: &["<a class=\"fr-HoverCard\">Docs</a>"],
        allowed_in: &[FrameScope::Component],
        related: &["focus", "active", "lift", "glow"],
        values: &["lift", "glow", "brighten", "dim"],
        docs_anchor: Some("docs/effects.md"),
    },
    FrameConcept {
        name: "keyframes",
        kind: ConceptKind::Declaration,
        summary: "Defines reusable animation keyframes.",
        description: "Defines reusable animation keyframes with structured timeline selectors. Use `from`, `to`, and percentage blocks to teach Frame how an animation changes over time.",
        generated_css: Some("Emits `@keyframes frame-Name` with supported animatable properties."),
        frame_examples: &[KEYFRAMES_FRAME],
        svelte_examples: &["<section class=\"fr-Panel\">...</section>"],
        allowed_in: &[FrameScope::Root],
        related: &["animation", "duration", "ease", "fill"],
        values: &["from", "to", "50%", "opacity", "transform"],
        docs_anchor: Some("docs/animations.md"),
    },
    FrameConcept {
        name: "animation",
        kind: ConceptKind::Property,
        summary: "Applies preset or custom animation motion.",
        description: "Applies a preset animation or a custom `keyframes` declaration. Use a one-line preset for common motion, or an `animation Name { ... }` block when timing, delay, iteration, direction, fill mode, or play state matter.",
        generated_css: Some("Emits `animation: frame-name ...` and `animation-play-state` when configured."),
        frame_examples: &["card Panel {\n  animation FloatIn {\n    duration 240ms\n    ease smooth\n    fill both\n  }\n}"],
        svelte_examples: &["<section class=\"fr-Panel\">...</section>"],
        allowed_in: &[FrameScope::Component, FrameScope::State],
        related: &["keyframes", "duration", "delay", "ease", "iteration"],
        values: &["fade-in", "slide-up", "pop-in", "pulse"],
        docs_anchor: Some("docs/animations.md"),
    },
    FrameConcept {
        name: "below",
        kind: ConceptKind::Property,
        summary: "Starts a max-width responsive override.",
        description: "Starts a responsive override for viewports below a named breakpoint. Use it when a layout should simplify on smaller screens.",
        generated_css: Some("Emits an `@media (max-width: ...)` rule for the current class."),
        frame_examples: &[RESPONSIVE_FRAME],
        svelte_examples: &[GRID_SVELTE],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["above", "between", "columns", "rows"],
        values: &["mobile", "tablet", "desktop", "wide"],
        docs_anchor: Some("docs/grid.md"),
    },
    FrameConcept {
        name: "container",
        kind: ConceptKind::Property,
        summary: "Starts a container query override.",
        description: "Starts a container query override so a component can adapt to the space it receives instead of the whole viewport.",
        generated_css: Some("Emits an `@container` rule for the current class."),
        frame_examples: &[CONTAINER_FRAME],
        svelte_examples: &["<section class=\"fr-Cards\">...</section>"],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["below", "above", "columns"],
        values: &["narrow", "content", "wide"],
        docs_anchor: Some("docs/grid.md"),
    },
    FrameConcept {
        name: "width 25%",
        kind: ConceptKind::Value,
        summary: "A percentage sizing value.",
        description: "A percentage sizing value. Use percentages for explicit dashboard columns, sidebars, split layouts, proportional areas, and available width calculations.",
        generated_css: Some("Emits percentage CSS sizing such as `width: 25%` or grid track percentages."),
        frame_examples: &["grid Dashboard {\n  columns 25% 50% 25%\n}"],
        svelte_examples: &[GRID_SVELTE],
        allowed_in: &[FrameScope::Grid, FrameScope::Area, FrameScope::Component],
        related: &["columns", "width", "height", "col"],
        values: &["25%", "33%", "50%", "66%", "75%", "100%"],
        docs_anchor: Some("docs/sizing.md"),
    },
];

use frame_core::knowledge;

pub fn doc_for(name: &str, fallback: &str) -> String {
    knowledge::completion_doc(name).unwrap_or_else(|| fallback.to_string())
}

pub const SURFACE_PANEL_DOC: &str = r#"surface panel

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

pub const SURFACE_MAIN_DOC: &str = r#"surface main

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

pub const SURFACE_GLASS_DOC: &str = "surface glass\n\nA translucent surface for overlays, floating panels, and command palettes.\nGenerated CSS uses `background: var(--frame-surface-glass);`.";
pub const SURFACE_GRADIENT_DOC: &str = "surface gradient\n\nApplies a named Frame gradient such as `dusk`, `midnight`, or `aurora`.\nUse gradients for feature cards, callouts, and interactive surfaces that need extra emphasis.";
pub const WIDTH_PERCENT_DOC: &str = "width 25%\n\nMakes this item take a percentage of the available width.\nUseful for sidebars and split layouts.\nGenerated CSS writes values like `width: 25%;` or `height: 50%;`.";

pub const DISPLAY_DOC: &str = r#"display

Sets the element display mode without using the raw CSS escape hatch.

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

card Toolbar {
  display flex
}"#;

pub const FLEX_DOC: &str = r#"flex

Controls flexbox behavior through structured subcommands.

Supported forms:
- `flex direction row`
- `flex direction column`
- `flex wrap wrap`
- `flex grow 1`
- `flex shrink 0`
- `flex basis fill`

Generated CSS writes `flex-direction`, `flex-wrap`, `flex-grow`, `flex-shrink`, or `flex-basis`.

row Toolbar {
  flex wrap wrap
}"#;

pub const INCLUDE_DOC: &str = r#"#include

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

pub const TRANSITION_DOC: &str = r#"transition

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

pub const ANIMATION_DOC: &str = r#"animation

Applies a named entrance or emphasis animation.

Common values: `fade-in`, `slide-up`, `pop-in`, `pulse`, and `none`.

Frame:

card Notice {
  surface panel
  animation pop-in
}

Generated CSS uses deterministic keyframes such as `frame-pop-in`.

Docs: `docs/animations.md`"#;

pub const KEYFRAMES_DOC: &str = r#"keyframes

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

pub const KEYFRAME_SELECTOR_DOC: &str = r#"keyframe selector

Marks a point in an animation timeline.

Use `from` for the initial state, `to` for the final state, and percentages like `50%` for intermediate states.

Generated CSS keeps the selector inside `@keyframes frame-Name`."#;

pub const RESPONSIVE_DOC: &str = r#"responsive block

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

pub const CONTAINER_DOC: &str = r#"container

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

pub const SPLIT_DOC: &str = r#"split

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

pub const OVERLAY_DOC: &str = r#"overlay

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

pub const DOCK_DOC: &str = r#"dock

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

pub const COLUMNS_DOC: &str = r#"columns

Defines the horizontal sections of a grid.

Generated CSS:
- named columns become equal `minmax(0, 1fr)` tracks and named grid areas
- percentage columns become exact `grid-template-columns` percentages
- `responsive cards` becomes an auto-fitting card grid

Examples:

columns sidebar content inspector
columns 25% 50% 25%
columns responsive cards"#;

pub const ROWS_DOC: &str = r#"rows

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

pub const PLACE_DOC: &str = r#"place

Claims a named grid slot from the parent grid.

grid Dashboard {
  columns sidebar content inspector
}

area Sidebar {
  in Dashboard
  place sidebar
}"#;

pub const COL_DOC: &str = r#"col

Places an area in a numeric grid column.
Use this when columns are percentages or explicit tracks.

grid Dashboard {
  columns 25% 50% 25%
}

area Sidebar {
  in Dashboard
  col 1
}"#;

pub const ALIGN_DOC: &str = r#"align

Controls vertical or cross-axis placement.
Generated CSS writes `align-items`.

row Toolbar {
  align center
  justify between
}"#;

pub const JUSTIFY_DOC: &str = r#"justify

Controls horizontal or main-axis placement and distribution.
Generated CSS writes `justify-content`.

row Toolbar {
  align center
  justify between
}"#;

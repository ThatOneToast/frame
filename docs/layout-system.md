# Layout System

Frame layout should describe spatial intent without requiring authors to choose flexbox or CSS grid terminology first.

The canonical language registry in `crates/frame_core/src/language.rs` defines all layout primitives, properties, and values. Parser, LSP, completions, hover, and diagnostics consume this registry.

The styling compiler may still emit CSS grid, flexbox, positioning, and overflow. The language surface should prefer layout primitives that can map to DOM, WebView, or future native renderers.

## Principles

- Layout primitives describe product structure, not CSS algorithms.
- Common app layouts should be one or two declarations, not wrapper-heavy trees.
- Renderer mappings should be deterministic and documented.
- Advanced CSS remains available in the styling layer and escape hatches.
- Data tables and interactive grids are not the same as visual `grid`.

## Page Root Styling

Frame supports `html` and `page-body` declarations for full-page styling. These emit global CSS rules, not class-based styles.

### `html`

Styles the root `<html>` element. Use for page-level background, text color, and font settings.

```frame
html {
  background #0a0f1a
  color #e2e8f0
}
```

Generated CSS:
```css
html {
  background: #0a0f1a;
  color: #e2e8f0;
}
```

### `page-body`

Styles the `<body>` element. Use for page-level layout, margin, min-height, and background. Named `page-body` to avoid confusion with UI/body concepts.

```frame
page-body {
  margin none
  background #0a0f1a
  color #e2e8f0
}
```

Generated CSS:
```css
body {
  min-height: 100vh;
  margin: 0;
  background: #0a0f1a;
  color: #e2e8f0;
}
```

### Full-Page Dark App Pattern

```frame
html {
  background #0a0f1a
  color #e2e8f0
}

page-body {
  margin none
  background #0a0f1a
  color #e2e8f0
}

grid AppShell {
  tracks columns panel fill
  tracks rows auto fill
  gap none
  height screen
}
```

## Proposed Primitives

### `dock`

Meaning: attach content to an edge of the available space.

```frame
dock AppChrome {
  edge left
  size sidebar
  content Sidebar
}
```

Default DOM mapping: positioned or grid-area layout depending on parent. Accessibility comes from contained primitive, such as `menu` or `toolbar`.

### `stack`

Meaning: arrange children in a single ordered direction.

```frame
stack SettingsFields {
  direction down
  gap medium
}
```

Default DOM mapping: flex column or block/grid flow. `direction right` can map to row-like layout. The author should not need `flex-direction`.

### `flow`

Meaning: let items wrap naturally across available space.

```frame
flow TagList {
  gap small
  wrap
}
```

Default DOM mapping: flex wrap or grid auto-placement.

### `grid`

Meaning: place content in named regions or repeated cells.

```frame
grid Dashboard {
  areas sidebar content inspector
  gap medium
}
```

Default DOM mapping: CSS grid. Frame source should prefer `areas`, `slots`, `repeat`, and `min item` over CSS track syntax.

### `overlay`

Meaning: layer content above another surface.

```frame
overlay CommandMenu {
  anchor SearchBox
  align below start
}
```

Default DOM mapping: absolutely positioned layer inside the current rendering tree. Portals are explicitly out of scope for this milestone.

### `scroll`

Meaning: bounded content region with independent scrolling.

```frame
scroll MessageHistory {
  direction y
  restore position
}
```

Default DOM mapping: overflow container. Compiler should track whether the region needs a name, keyboard focus, or scroll restoration hooks.

### `split`

Meaning: two or more resizable or fixed panes.

```frame
split Workspace {
  pane Sidebar size sidebar
  pane Editor fill
  resize between
}
```

Default DOM mapping: grid/flex plus optional resize behavior when implemented. Accessibility should expose resize handles when interactive resizing exists.

## Current Syntax Review

Current Frame style syntax includes:

```frame
grid AppShell {
  columns sidebar content inspector
  rows main
}

row Toolbar {
  align center
  justify between
}

card SidebarItem {
  display flex
  flex direction column
}
```

This is useful and should remain in the styling layer, but it leaks CSS concepts:

- `columns` and `rows` are close to CSS grid tracks
- `display flex` and `display grid` expose implementation
- `flex direction`, `grow`, `shrink`, and `basis` are raw layout mechanics
- `area`, `col`, `row`, and `span` expose grid placement details

## Proposed Authoring Direction

Semantic UI code should use layout primitives:

```frame
component Workspace {
  view {
    split Main {
      sidebar ProjectNav {
        menu Projects
      }

      panel Editor {
        toolbar EditorTools
        scroll DocumentBody
      }
    }
  }
}
```

Styling code can still provide implementation detail:

```frame
split Main {
  gap medium
  min-height screen
}
```

## Mapping Rules

- `stack direction down` maps to vertical flow.
- `stack direction right` maps to horizontal flow.
- `flow wrap` maps to wrapping layout.
- `grid areas a b c` maps to named renderer slots.
- `dock edge left` maps to a stable edge region.
- `scroll direction y` maps to vertical overflow.
- `split` maps to named panes with deterministic order and optional resize metadata.

Renderers may choose CSS grid, flexbox, native stacks, native split views, or custom layout algorithms. The IR should preserve the layout primitive and constraints, not just the generated CSS.

## Diagnostics

The compiler and LSP should report:

- unnamed scroll regions that need accessible names
- `overlay` without anchor or dismissal behavior when interactive
- `split resize` without keyboard-accessible resize controls
- ambiguous `grid` used for data when `table` or `data` is intended
- raw `display flex` suggestions when a `stack` or `flow` primitive is clearer

## Semantic App Shells (`layout`)

For app shells, prefer the intent-based `layout` declaration over raw track
strings:

```frame
layout DashboardShell {
  shell {
    sidebar left fixed 18rem
    main fluid
    inspector right clamp(20rem, 28vw, 28rem)
  }
  gap large
  density comfortable
  below tablet { shell stacked }
}
```

- Region lines read `<name> [left|right] [fixed <size> | fluid | <size-expr>]`.
- Regions order left -> main -> right and lower to `grid-template-columns`
  plus `grid-template-areas`.
- Children attach by source order or `data-frame-section="<region>"`.
- `density compact|comfortable|spacious` sets the shell padding scale.
- Inside responsive conditions, `shell stacked` stacks regions vertically.

Advanced grids (`grid` with `tracks`/`columns`) remain for layouts that need
full track control; treat raw track strings as the escape hatch.

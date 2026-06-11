# Layout

Frame layout declarations cover common app structure without requiring raw CSS.

## Content Areas

Use `area` to section off a grid:

```frame
area Content {
  in Dashboard
  place content
  surface main
  padding large
}
```

Use named placement with named columns, or numeric placement with percentage columns:

```frame
area Sidebar {
  in Dashboard
  col 1
  width 25%
}
```

## Sizing

Frame supports named sizing and percentages:

```frame
width fill
width content
width sidebar
width narrow
width wide
width 25%
width 50%
height screen
height 100%
min-height screen
max-width content
```

Percentages must be `0%` through `100%`.

## Alignment

`align` controls vertical or cross-axis placement. `justify` controls horizontal or main-axis distribution.

```frame
row Toolbar {
  align center
  justify between
  gap small
}
```

Common values:

```frame
align start
align center
align end
align stretch

justify start
justify center
justify end
justify between
justify around
justify evenly
```

## Display and Flex

Use `display` when a declaration needs to override its default layout mode:

```frame
card InlineToolbar {
  display inline-flex
  align center
  gap small
}
```

Supported display values are:

```frame
display block
display inline
display inline-block
display flex
display inline-flex
display grid
display inline-grid
display contents
display none
```

Use `flex` subcommands for flexbox behavior:

```frame
row ActionBar {
  flex wrap wrap
}

card SidebarItem {
  display flex
  flex direction column
  flex grow 1
  flex shrink 0
  flex basis fill
}
```

Generated CSS writes `display`, `flex-direction`, `flex-wrap`, `flex-grow`, `flex-shrink`, and `flex-basis`.

Use `visibility` when an element should keep its layout slot:

```frame
card PendingPreview {
  visibility hidden
}
```

## Dense App Rows

Use `layout` presets when a component needs a repeated internal structure:

```frame
card ChannelButton {
  layout icon-content-action
  gap small
  control reset
  interactive
  align-text left
}

card MessageItem {
  layout avatar-content
  gap medium
}
```

Use `scroll y`, `overflow hidden`, `box border`, directional borders, and `square` sizes for common application panels and controls:

```frame
area ChatPanel {
  scroll y
}

area Sidebar {
  border right terminal-border
}

card ServerButton {
  square server
  layout center
}
```

## Top NavBar

Use `rows` to create a page header band, then put a `row` NavBar in that header.

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
  padding medium
}

area MainContent {
  in AppShell
  row 2
  surface main
  padding large
}

row NavBar {
  align center
  justify between
  gap medium
}

card NavAction {
  surface flat
  text accent
  padding small
  radius pill
}
```

Svelte:

```svelte
<div class="fr-AppShell">
  <header class="fr-Header">
    <nav class="fr-NavBar">
      <a class="fr-NavAction">Home</a>
      <a class="fr-NavAction">Docs</a>
    </nav>
  </header>

  <main class="fr-MainContent">
    Content
  </main>
</div>
```

## Horizontal Split

Use `columns` for left/right splits.

```frame
grid Workspace {
  columns 33% 67%
  gap medium
  height screen
}

area NavigationPane {
  in Workspace
  col 1
  surface panel
  padding medium
}

area DetailPane {
  in Workspace
  col 2
  surface main
  padding large
}
```

Use named columns when exact percentages do not matter:

```frame
grid Workspace {
  columns navigation detail
  gap medium
}

area NavigationPane {
  in Workspace
  place navigation
}

area DetailPane {
  in Workspace
  place detail
}
```

## Vertical Split

Use `rows` for top/bottom splits.

```frame
grid Page {
  rows auto fill auto
  gap medium
  min-height screen
}

area Header {
  in Page
  row 1
}

area Content {
  in Page
  row 2
}

area Footer {
  in Page
  row 3
}
```

## Common Layouts

```frame
stack SettingsPanel {
  gap medium
  align stretch
}

center EmptyState {
  height screen
  surface main
  text muted
}

split AppLayout {
  gap medium
}

overlay ModalLayer {
  position center
  z modal
}

dock AppDock {
  surface glass
  padding medium
}
```

## Layout Primitive Defaults

Frame primitives have sensible CSS defaults:

| Primitive | Default Display | HTML Tag | Notes |
|-----------|----------------|----------|-------|
| `screen` | `div` | `div` | App root container |
| `panel` | `section` | `section` | Section-like region |
| `stack` | `flex column` | `div` | Vertical layout |
| `row` | `flex row` | `div` | Horizontal layout |
| `grid` | `grid` | `div` | CSS grid |
| `card` | `flex column` | `div` | Block container |
| `action` | `inline-flex row` | `button` | Button with reset |
| `input` | `input` | `input` | Input element |
| `text` | `span` | `span` | Inline text |

### Button Reset

`action` elements rendered as `<button>` receive automatic CSS reset:

- `appearance: none`
- `background: none`
- `border: none`
- `cursor: pointer`
- `font: inherit`
- `color: inherit`
- `display: inline-flex`
- `flex-direction: row`
- `gap: var(--frame-space-small)`

This ensures buttons look like Frame-styled controls, not browser defaults.

### Text Node Wrapping

Text nodes inside components are wrapped in `<span class="fr-FrameText">` elements. This allows them to:

- Participate in flex/grid gap spacing
- Receive CSS styling
- Be targeted by selectors

```css
.fr-FrameText {
  display: inline;
  white-space: pre-wrap;
}
```

### When to Use Each Primitive

- **`stack`**: Vertical lists, sidebar groups, form fields
- **`row`**: Horizontal bars, nav bars, table rows
- **`grid`**: Dashboard layouts, card grids, multi-column layouts
- **`card`**: Contained panels, metric cards, list items
- **`action`**: Buttons, links, clickable controls
- **`panel`**: Page sections, sidebar panels
- **`screen`**: App root container

## Table/Grid-Row Pattern

For table-like layouts where headers and cells must align in columns, use `columns` on `row` declarations. This emits `display: grid` with a shared column template:

```frame
row TableHeader {
  columns 2fr 1fr 1fr 1fr 1fr 1fr
  color text-muted
  size caption
  weight semibold
  case uppercase
}

row TableRow {
  columns 2fr 1fr 1fr 1fr 1fr 1fr
  align center
  padding y small
  border bottom soft
}
```

Inherited rows share the same column template:

```frame
row TableRowBase {
  columns 2fr 1fr 1fr 1fr 1fr 1fr
  align center
}

row RunRow1 extends TableRowBase { }
row RunRow2 extends TableRowBase { }
```

Generated CSS:

```css
.fr-TableHeader {
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr);
}

.fr-TableRow {
  display: grid;
  grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr);
  align-items: center;
}
```

Without `columns`, rows emit `display: flex; flex-direction: row;` for general-purpose horizontal layouts.

## Gap-Safe Grid Tracks

When using `grid` with `gap`, prefer fractional (`fr`) columns over percentages to prevent overflow:

```frame
grid PerformanceGrid {
  columns 3fr 2fr
  gap medium
}
```

This emits:

```css
.fr-PerformanceGrid {
  display: grid;
  grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);
  gap: var(--frame-space-medium);
}
```

Percentage columns (`60% 40%`) plus a gap can overflow the container. Fractional columns with `minmax(0, Nfr)` are gap-safe.

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

## Dense App Rows

Use `layout` presets when a component needs a repeated internal structure:

```frame
button ChannelButton {
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

button ServerButton {
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

button NavAction {
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

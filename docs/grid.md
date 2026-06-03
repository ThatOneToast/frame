# Grid

`grid` is Frame's first-class app layout primitive. Use it when a page has named sections, sidebars, inspectors, or responsive card collections.

## Dashboard Layout

```svelte
<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
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
</style>
```

Percentage columns generate direct CSS:

```css
grid-template-columns: 25% 50% 25%;
```

## Named Areas

Named columns are best when you want child areas to claim readable slots:

```frame
grid Dashboard {
  columns sidebar content inspector
  gap medium
}

area Sidebar {
  in Dashboard
  place sidebar
}
```

Generated CSS uses equal flexible columns and `grid-template-areas`.

## Responsive Cards

```frame
grid QuickLinks {
  columns responsive cards
  gap medium
}
```

This generates an auto-fitting card grid using normal CSS grid.

## Vertical Flow

Named `columns` normally lay out left to right. Add `flow vertical` when the named sections should stack top to bottom while keeping the same readable section names:

```frame
grid HoverCardInfo {
  flow vertical
  columns title description
  gap small
  padding small
  height narrow

  section title {
    padding bottom small
  }

  section description {
    padding top none
  }
}
```

Generated CSS uses one column, named row areas, and child assignment rules. Children map by order:

```svelte
<section class="fr-HoverCardInfo">
  <h2>HoverCard</h2>
  <p>description</p>
</section>
```

For more explicit markup, add `data-frame-section`:

```svelte
<section class="fr-HoverCardInfo">
  <h2 data-frame-section="title">HoverCard</h2>
  <p data-frame-section="description">description</p>
</section>
```

`section name { ... }` currently supports section-level spacing, sizing, and alignment such as `padding top small`, `margin bottom medium`, `gap small`, `align center`, `justify between`, `width fill`, and `height content`.

## Rows For Header, Content, Footer

Rows split a grid vertically. This is the clearest way to make a top NavBar area.

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

area Footer {
  in AppShell
  row 3
  surface panel
  padding medium
}
```

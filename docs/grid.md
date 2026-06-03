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

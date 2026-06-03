# Examples

## Dashboard With Sidebar And Inspector

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

## Named Grid Areas

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

## Responsive Card Grid

```frame
grid QuickLinks {
  columns responsive cards
  gap medium
}
```

## Toolbar Alignment

```frame
row Toolbar {
  align center
  justify between
  gap small
}
```

## Gradient Hover Card

```frame
card HoverCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium
  text bright

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

## Centered Empty State

```frame
center EmptyState {
  height screen
  surface main
  text muted
}
```

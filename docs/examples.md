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

## Custom Gradient Card

```frame
#include theme

card HeroCard {
  background hero-gradient
  color brand-text
  padding large
  radius large
  shadow floating
}
```

```frame
// theme.frame
tokens Brand {
  color brand-purple #7c3aed
  color brand-panel #181820
  color brand-text #f8fafc

  gradient hero-gradient {
    type linear
    angle 135deg
    stop brand-purple 0%
    stop brand-panel 100%
  }
}
```

## Advanced Escape Hatch

```frame
card GlassCard {
  surface glass
  padding large
  radius large

  advanced {
    css "backdrop-filter" blur(12px)
  }
}
```

## Four-corner Gradient

```frame
tokens Brand {
  color brand-purple #7c3aed
  color brand-blue #2563eb
  color brand-bg #0f172a

  gradient corner-wash {
    type layered
    corner top-left brand-purple 65%
    corner top-right brand-blue 65%
    corner bottom-left brand-bg 70%
    corner bottom-right #0f172a 70%
  }
}

card HeroCard {
  background corner-wash
  padding top large
  padding x medium
  anchor top
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
# Practical Examples

## Custom Theme

```frame
tokens Brand {
  color brand #7c3aed
  color brand-muted #a78bfa
  color page-bg #0f0f14
  color panel-bg #181820
}

grid AppShell {
  columns sidebar content
  background page-bg
}

area Sidebar {
  in AppShell
  place sidebar
  background panel-bg
  border brand-muted
}

card BrandCard {
  background brand
  color white
  radius large
  shadow medium
}
```

## Border And Glow Card

```frame
card AlertCard {
  surface panel
  border warning
  border width medium
  glow warning
  padding large
  radius large
}
```

## Animated Hover Card

```frame
card AnimatedCard {
  surface gradient aurora
  padding large
  radius large
  shadow soft
  transition smooth

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

## Split Layout With Percentages

```frame
grid Dashboard {
  columns 25% 50% 25%
  gap medium
  height screen
}
```

## Imports

```frame
#include tokens
#include layout
#include cards

card LocalCard {
  surface panel
  padding medium
}
```

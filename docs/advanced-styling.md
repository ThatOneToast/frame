# Advanced Styling

Frame prefers structured design intent first. Use `advanced` only when a feature is not modeled yet.

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

Generated CSS stays scoped to the declaration:

```css
.fr-GlassCard {
  background: var(--frame-surface-glass);
  padding: var(--frame-space-large);
  border-radius: var(--frame-radius-large);
  backdrop-filter: blur(12px);
}
```

The escape hatch is explicit so raw CSS does not become the default authoring path. Prefer native Frame properties such as `surface`, `background`, `border`, `shadow`, `transition`, `animation`, `align`, `justify`, `width`, `height`, `anchor`, and targeted `padding` when they fit.

```frame
card HeroCard {
  anchor top
  padding top large
  padding x medium
  background corner-wash
}
```

# Gradients

Define custom gradients inside `tokens`:

```frame
tokens Brand {
  color brand-purple #7c3aed
  color brand-blue #2563eb
  color brand-bg #0f172a

  gradient hero-gradient {
    type linear
    angle 135deg
    stop brand-purple 0%
    stop brand-blue 45%
    stop brand-bg 100%
  }
}
```

Use the gradient by name:

```frame
card HeroCard {
  background hero-gradient
  color white
  padding large
  radius large
  shadow floating
}
```

Generated CSS uses variables:

```css
:root {
  --frame-gradient-hero-gradient: linear-gradient(135deg, var(--frame-color-brand-purple) 0%, var(--frame-color-brand-blue) 45%, var(--frame-color-brand-bg) 100%);
}

.fr-HeroCard {
  background: var(--frame-gradient-hero-gradient);
}
```

Current validation supports `type linear`, `angle <deg>`, and at least two `stop <color-token-or-hex> <percentage>` entries.

## Layered Corner Gradients

Use `corner` entries to build a multi-layer gradient from multiple corners:

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
  color white
}
```

Generated CSS combines radial gradient layers from each corner. You can also combine `corner` layers with regular `stop` entries; Frame appends the linear layer after the corner layers.

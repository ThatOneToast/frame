# Tokens

Use `tokens` for named design values that should be shared across components.

```frame
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

Use custom colors anywhere a color intent is accepted:

```frame
card BrandCard {
  background brand-panel
  color brand-text
  border brand-purple
}
```

Use custom gradients anywhere a background or surface gradient is accepted:

```frame
card HeroCard {
  background hero-gradient
  color brand-text
}
```

Generated CSS:

```css
:root {
  --frame-color-brand-purple: #7c3aed;
  --frame-gradient-hero-gradient: linear-gradient(135deg, var(--frame-color-brand-purple) 0%, var(--frame-color-brand-panel) 100%);
}
```

Supported color token values today are hex colors: `#fff`, `#ffffff`, and `#ffffffff`.
Function colors such as `rgb(...)` and `hsl(...)` are planned future syntax.

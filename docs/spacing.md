# Spacing

Frame spacing is named by intent and scale:

```txt
none small medium large xlarge
```

Use `padding` for internal spacing, `margin` for external spacing, and `gap` for space between children.

```frame
card QuickLinkCard {
  surface panel
  padding large
  radius large
}

row Toolbar {
  gap small
  padding medium
}
```

Generated CSS maps these values to Frame spacing variables, so the design scale stays consistent across Svelte components.

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

Target one side or axis when a section needs tighter control:

```frame
card HeroCard {
  padding top large
  padding x medium
}

grid HoverCardInfo {
  flow vertical
  columns title description

  section title {
    padding bottom small
  }

  section description {
    padding top none
  }
}
```

Supported targets are `top`, `right`, `bottom`, `left`, `x`, `y`, `inline`, and `block`.

Generated CSS maps these values to Frame spacing variables, so the design scale stays consistent across Svelte components.

# Surfaces

Surfaces describe visual layers with named intent.

## Main And Panel

Use `surface main` for the primary content background:

```frame
area Content {
  in Dashboard
  place content
  surface main
  padding large
}
```

Use `surface panel` for secondary UI:

```frame
area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}
```

Good panel uses:

- sidebars
- inspectors
- cards
- menus
- tool panels

## Other Surfaces

```frame
card GlassCard {
  surface glass
  border soft
  shadow soft
}

card RaisedCard {
  surface raised
  shadow medium
}

card FlatItem {
  surface flat
}
```

## Gradients

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

Available gradients:

```frame
surface gradient dusk
surface gradient midnight
surface gradient aurora
```
# Surfaces

`surface` sets the visual background role for a component.

```frame
area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}
```

Use:

- `surface main` for primary page/content backgrounds.
- `surface panel` for sidebars, cards, inspectors, menus, and settings panels.
- `surface raised` for cards/buttons that should visually pop forward.
- `surface flat` for minimal or transparent UI.
- `surface glass` for translucent overlays and floating regions.
- `surface overlay` for modals, menus, and popovers.
- `surface inset` for field/input-like regions.
- `surface sunken` for recessed panels.

Gradients:

```frame
card FeatureCard {
  surface gradient aurora
  padding large
  radius large
}
```

Supported gradients: `dusk`, `midnight`, `aurora`, `ember`, `ocean`, and `forest`.

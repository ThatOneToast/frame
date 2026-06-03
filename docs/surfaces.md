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

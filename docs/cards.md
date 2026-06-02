# Cards

Cards are reusable content surfaces. They commonly combine surface, spacing, shape, depth, and interaction states.

```frame
card HoverCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
  }
}
```

Expanded card styling:

```frame
card ProjectCard {
  surface gradient aurora
  padding large
  radius large
  shadow soft
  border accent

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

# Animations

Frame keeps motion intent named and predictable.

```frame
card AnimatedCard {
  surface gradient aurora
  padding large
  radius large
  shadow soft
  transition smooth
  animation fade-in

  hover {
    lift small
    glow accent
    brighten subtle
    transition fast
  }
}
```

Transitions:

- `transition none`
- `transition fast`
- `transition smooth`
- `transition slow`

Timing:

- `duration fast`
- `duration normal`
- `duration slow`
- `ease linear`
- `ease smooth`
- `ease bounce`
- `ease sharp`

Animations:

- `animation fade-in`
- `animation slide-up`
- `animation pop-in`
- `animation pulse`
- `animation none`

Generated CSS uses deterministic names such as `frame-fade-in`.

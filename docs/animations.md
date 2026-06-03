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

## Custom Keyframes

Use `keyframes` when motion needs named timeline states instead of a preset:

```frame
keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px) scale(0.98)
  }

  to {
    opacity 1
    transform translateY(0) scale(1)
  }
}
```

This generates:

```css
@keyframes frame-FloatIn {
  from {
    opacity: 0;
    transform: translateY(12px) scale(0.98);
  }

  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
```

Apply custom keyframes with a structured animation block:

```frame
card Panel {
  animation FloatIn {
    duration 240ms
    delay 0ms
    ease smooth
    iteration 1
    direction normal
    fill both
    play-state running
  }
}
```

Animation blocks support:

- `duration fast`, `duration normal`, `duration slow`, or explicit values like `240ms`
- `delay 120ms`
- `ease linear`, `ease smooth`, `ease bounce`, `ease sharp`
- `iteration 1` or `iteration infinite`
- `direction normal`, `reverse`, `alternate`, `alternate-reverse`
- `fill none`, `forwards`, `backwards`, `both`
- `play-state running` or `paused`

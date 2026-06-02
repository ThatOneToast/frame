# Effects

Effects live inside interaction state blocks. They describe feedback intent instead of raw transforms and filters.

```frame
card QuickLinkCard {
  shadow soft

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

Focus, active, and disabled states:

```frame
button PrimaryButton {
  surface panel
  padding medium
  radius pill

  focus {
    ring accent
  }

  active {
    press
  }

  disabled {
    dim medium
  }
}
```

Supported effect intent includes `lift`, `glow`, `brighten`, `dim`, `blur`, `press`, `ring`, `fade`, and `scale`.

# Effects

Effects describe feedback intent instead of raw transforms and filters. Motion helpers can be used on a style declaration or inside interaction state blocks.

```frame
card QuickLinkCard {
  shadow soft
  lift small

  hover {
    lift small%44
    grow slight
    glow accent
    brighten subtle
  }
}
```

Focus, active, and disabled states:

```frame
card PrimaryButton {
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

Intent motion helpers:

```frame
card FloatingCard {
  lift small
  shift right medium
  tilt right subtle%23
  grow slight%5
}
```

Movement intents use `tiny`, `small`, `medium`, `large`, and `huge`. Visual transform intents use `slight`, `subtle`, `normal`, `strong`, and `dramatic`.

Suffix percentages tune from the named amount toward the next stronger amount. `small%0` equals `small`; `small%100` equals `medium`. When the strongest amount is tuned, Frame extrapolates by the previous step distance, so `huge%50` is stronger than `huge`.

Supported motion intent includes `lift`, `sink`, `shift left`, `shift right`, `shift up`, `shift down`, `grow`, `shrink`, `tilt left`, `tilt right`, `press`, and `pop`.

Supported non-motion effect intent includes `glow`, `brighten`, `dim`, `blur`, `ring`, and `fade`. Existing legacy effect words such as `scale` remain supported for older files, but new code should prefer the intent verbs.

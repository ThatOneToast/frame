# Colors

Frame colors are semantic tokens first. Pick the value that matches the UI role, not a raw color code.

## Text And Color

```frame
text bright
text muted
text accent
text danger
text success
text warning

color bright
color muted
color accent
color primary
color secondary
color danger
color success
color warning
color info
```

`text` and `color` both generate `color: var(--frame-color-...)`.

## Background

```frame
background main
background panel
background accent
background danger
background success
background warning
```

`main` and `panel` use surface tokens. Semantic values like `danger` and `success` use color tokens.

## Palette Guidance

Use `accent` for important interactive UI:

- primary buttons
- active nav items
- focus rings
- highlighted cards

Use `muted` for captions, metadata, and helper text.

Use `danger`, `success`, `warning`, and `info` for status and feedback.

Basic named colors are available for simple cases:

```frame
color white
color black
color gray
color red
color orange
color yellow
color green
color blue
color purple
color pink
```

The current generated root tokens use sensible dark-theme defaults for dashboards and chat-style apps.

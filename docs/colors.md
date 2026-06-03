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
# Colors

Use color intent for text, backgrounds, borders, glows, and themes.

```frame
card StatusCard {
  color bright
  background panel
  border accent

  hover {
    glow accent
  }
}
```

Named colors include:

```txt
white black gray slate red orange yellow green blue purple pink cyan
primary secondary accent muted bright danger success warning info
```

Custom color tokens:

```frame
tokens Brand {
  color brand #7c3aed
  color brand-muted #a78bfa
  color page-bg #0f0f14
  color panel-bg #181820
}

card BrandCard {
  background brand
  color white
  border brand-muted
}
```

Supported token values today are `#fff`, `#ffffff`, and `#ffffffff`.

Custom color tokens from included files are available anywhere built-in colors are valid:

```frame
#include theme

card BrandCard {
  background brand-panel
  color brand-text
  border brand-purple

  hover {
    glow brand-purple
  }
}
```

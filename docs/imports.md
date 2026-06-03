# Imports

Use `#include` at the top level to split large Frame files into smaller files.

```frame
#include tokens
#include layout
#include cards

card LocalCard {
  surface panel
  padding medium
}
```

Bare names resolve as `.frame` files from the current file directory or configured include paths.

```bash
frame check src/lib/frame/app.frame --include src/lib/frame
frame compile src/lib/frame/app.frame --out src/lib/frame --include src/lib/frame
```

Relative paths resolve from the including file.

```frame
#include ./styles/cards.frame
#include ../shared/tokens.frame
```

Frame reports missing includes and include cycles so large projects do not silently compile partial styles.

## Split Theme Files

```frame
// app.frame
#include theme
#include layout
#include cards
```

```frame
// theme.frame
tokens Brand {
  color brand-purple #7c3aed
  color brand-panel #181820
  color brand-text #f8fafc

  gradient hero-gradient {
    type linear
    angle 135deg
    stop brand-purple 0%
    stop brand-panel 100%
  }
}
```

Imported grids, grid sections, colors, and gradients are visible to compiler validation, codegen, and the LSP.

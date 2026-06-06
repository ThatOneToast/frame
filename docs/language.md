# Frame Language

Frame is a structured CSS language for Svelte projects. A `.frame` file describes components and layout regions with concepts like `surface panel`, `columns responsive cards`, `radius large`, and `hover { lift small }`.

Frame is not raw CSS with different punctuation. The goal is to expose CSS power through readable, guided syntax: first-class grids, responsive rules, container queries, gradients, transitions, animations, keyframes, tokens, states, and safe advanced CSS escape hatches.

## Declarations

Top-level declarations create generated classes:

```frame
grid AppShell {
  columns sidebar content inspector
  gap medium
}

card ProjectCard {
  surface gradient dusk
  padding large
  radius large
  shadow soft
}

keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px)
  }

  to {
    opacity 1
    transform translateY(0)
  }
}
```

Generated TypeScript exports stable class names:

```ts
export const ui = {
  AppShell: 'fr-AppShell',
  ProjectCard: 'fr-ProjectCard'
} as const;
```

## Statements

Statements are line-based:

```frame
surface panel
padding medium
width fill
```

Nested state blocks describe interaction intent:

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

The compiler emits readable CSS and a `generated.ts` file for Svelte imports.

## Feature Queries

Use typed `supports` blocks when styles should only emit behind a browser feature query:

```frame
supports display grid {
  grid AppShell {
    columns sidebar content
  }
}

supports subgrid {
  grid NestedGrid {
    columns subgrid
  }
}
```

Supported predicates are `display grid`, `display flex`, `backdrop blur`, `color oklch`, `selector has`, `container queries`, and `subgrid`.

Generated CSS uses `@supports`, for example `supports display grid` emits `@supports (display: grid)`.
# Frame Language

Frame is a design-intent CSS DSL. It compiles declarations such as `grid`, `area`, `card`, `row`, `stack`, `dock`, and `text` into normal CSS classes and stable TypeScript exports.

Top-level declarations:

```frame
#include tokens

tokens Brand {
  color brand #7c3aed
}

grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

card DemoCard {
  surface gradient dusk
  border accent
  shadow medium
  transition smooth

  hover {
    lift small
    glow accent
  }
}
```

Core docs:

- `docs/app-vocabulary.md`
- `docs/imports.md`
- `docs/tokens.md`
- `docs/colors.md`
- `docs/surfaces.md`
- `docs/borders.md`
- `docs/effects.md`
- `docs/animations.md`
- `docs/layout.md`
- `docs/sizing.md`
- `docs/grid.md`
- `docs/svelte.md`

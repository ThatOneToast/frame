# Frame Language

Frame is a design-intent CSS DSL for Svelte projects. A `.frame` file describes components and layout regions with concepts like `surface panel`, `columns responsive cards`, `radius large`, and `hover { lift small }`.

Frame is not raw CSS with different punctuation. When a concept is common, Frame should name the UI intent directly. Lower-level CSS escape hatches can come later, but the core language stays readable first.

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

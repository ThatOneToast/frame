# Frame Agent Guide

This folder is written for LLMs and coding agents that need to generate correct Frame code without guessing.

Frame is a design-intent CSS DSL for Svelte projects. It compiles Frame declarations into normal CSS classes and, for external `.frame` files, a TypeScript `ui` class map.

## Agent Decision Tree

Use external `.frame` files when the user wants:

- typed class names
- shared app layout styles
- generated `src/lib/frame/generated.ts`
- generated `src/lib/frame/generated.css`

Use Svelte `<style lang="frame">` blocks when the user wants:

- component-local styles
- quick examples
- no generated TypeScript requirement

Do not generate raw CSS-like Frame. Prefer intent words:

```frame
surface panel
padding medium
radius large
hover {
  lift small
  glow accent
}
```

Avoid this style:

```frame
background-color #111
display grid
border-radius 12px
```

Inside `view` blocks, use UI primitives instead of browser tags:

```frame
action Save { on press @save }
input Email { label "Email" }
editor Message { label "Message" }
```

Avoid browser tags in `view`:

```frame
button Save { on click @save }      // not valid in view
input type="email"                 // not valid in view
```

## Setup Commands

Inside an existing Svelte or SvelteKit project:

```bash
frame init svelte
```

Preview setup:

```bash
frame init svelte --dry-run
```

Local development from this repository:

```bash
cargo run -p frame_cli -- init svelte
```

Compile an external Frame file:

```bash
frame compile src/lib/frame/app.frame --out src/lib/frame
```

Check a Frame file:

```bash
frame check src/lib/frame/app.frame
```

Format a Frame file:

```bash
frame format src/lib/frame/app.frame
```

## Required Mental Model

Top-level declarations create classes:

```frame
grid Dashboard {
}

area Sidebar {
}

card ProjectCard {
}
```

Each declaration emits a class:

```txt
Dashboard -> fr-Dashboard
Sidebar -> fr-Sidebar
ProjectCard -> fr-ProjectCard
```

External `.frame` files also generate:

```ts
export const ui = {
  Dashboard: 'fr-Dashboard',
  Sidebar: 'fr-Sidebar',
  ProjectCard: 'fr-ProjectCard'
} as const;
```

Inline `<style lang="frame">` blocks emit CSS only. Use raw class names like `fr-ProjectCard` in the Svelte markup.

## Core Agent Rules

1. Use `grid` for page structure.
2. Use `area` for children placed inside a grid.
3. Every `area` should include `in GridName`.
4. Use `place name` when the grid has named columns.
5. Use `col 1`, `col 2`, `col 3` when the grid has percentage columns.
6. Use `surface main` for primary content regions.
7. Use `surface panel` for sidebars, inspectors, tool panels, and cards.
8. Use `text muted` for secondary text and `text bright` for high-emphasis text.
9. Use `align` for vertical/cross-axis placement.
10. Use `justify` for horizontal/main-axis distribution.
11. Use only `hover`, `focus`, `active`, and `disabled` as nested state blocks.
12. Put braces exactly around declarations and state blocks.

## Minimum Correct Example

```svelte
<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
</div>

<style lang="frame">
  grid Dashboard {
    columns sidebar content
    gap medium
    height screen
  }

  area Sidebar {
    in Dashboard
    place sidebar
    surface panel
    padding medium
  }

  area Content {
    in Dashboard
    place content
    surface main
    padding large
  }
</style>
```

## Common Failure Modes

Do not suggest values at the file root. Root scope accepts declarations such as `grid`, `area`, `card`, `stack`, and `row`.

Do not put `grid` or `card` inside `hover`:

```frame
card BadCard {
  hover {
    grid NestedGrid
  }
}
```

Use effects inside states:

```frame
card GoodCard {
  hover {
    lift small
    glow accent
  }
}
```

Do not use `place sidebar` unless the referenced grid defines `sidebar`:

```frame
grid Dashboard {
  columns sidebar content
}

area Sidebar {
  in Dashboard
  place sidebar
}
```

## Related Agent Docs

- [Language Cheat Sheet](language-cheatsheet.md)
- [Svelte Patterns](svelte-patterns.md)
- [Recipes](recipes.md)

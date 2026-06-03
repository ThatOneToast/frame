# LSP

Frame's language server is the editor-independent teaching layer for `.frame`
files. Tree-sitter handles syntax highlighting. The LSP handles semantic help:
diagnostics, completions, hover docs, formatting, document symbols, links,
navigation, code actions, semantic tokens, and folding.

## Features

- Parser and semantic diagnostics with suggestions and examples.
- Scope-aware completions for root declarations, grid properties, area
  placement, component styling, token blocks, gradient blocks, and state blocks.
- Markdown completion docs with Frame examples, Svelte examples, related
  concepts, and docs paths.
- Hover docs that explain intent, generated CSS behavior, common values, and
  when to use a concept.
- Formatting for `.frame` files and embedded `<style lang="frame">` blocks.
- Document symbols for declarations, with nested state blocks as children.
- Go-to-definition for `#include`, `area in GridName`, `place section`, and
  imported color/gradient token references.
- References for grid declarations and grid sections.
- Document links from supported concepts to markdown docs.
- Code actions for common typo fixes and safe layout scaffolds.
- Semantic tokens for declarations, names, properties, values, colors,
  percentages, includes, and comments.
- Folding ranges for declarations and nested state blocks.

## Scope-Aware Completions

At file root, Frame suggests declarations and imports only:

```txt
#include
tokens
grid
area
card
stack
row
button
text
center
split
overlay
dock
```

Inside a grid, suggestions focus on layout:

```frame
grid Dashboard {
  columns sidebar content inspector
  rows header main footer
  gap medium
  height screen
  surface main
  align stretch
  justify between
}
```

After `columns`, values include common named sections and layout helpers:

```txt
responsive cards sidebar content inspector header footer main auto fill
25% 33% 50% 66% 75% 100%
```

Inside an area, `in` suggests known grids and `place` suggests sections from
the referenced grid:

```frame
area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}
```

Inside `hover`, `focus`, `active`, and `disabled`, Frame suggests effects only:

```frame
card ProjectCard {
  hover {
    lift small
    glow accent
    transition smooth
  }
}
```

## Hover Docs

Hover docs teach the concept before describing syntax. For example, hovering
`surface panel` explains that panels are secondary UI regions, shows generated
background behavior, gives a Frame example, gives a Svelte class usage example,
and links to `docs/surfaces.md`.

Hover docs are available for declarations, grid placement, surfaces, colors,
spacing, sizing, alignment, effects, transitions, animations, includes, and
custom color/gradient tokens.

## Diagnostics

Diagnostics come from the parser and semantic validator, so the CLI and LSP
share the same messages.

```frame
card Demo {
  surface pannel
}
```

Reports an unknown surface and suggests `panel`.

```frame
area Sidebar {
  in Dashbord
}
```

Reports an unknown grid and suggests the closest grid name when one exists.

```frame
grid Dashboard {
  columns 25%% 50% 25%
}
```

Reports an invalid percentage and suggests values like `25%`, `50%`, and
`100%`.

More examples are in `docs/diagnostics.md`.

## Navigation

Go-to-definition supports:

- `#include theme` to `theme.frame`
- `in Dashboard` to `grid Dashboard`
- `place sidebar` to the matching `columns sidebar ...` section
- imported `background brand-panel` to a color token
- imported `background hero-gradient` to a gradient token

References include grid declarations, `in GridName` usages, named grid columns,
and matching `place` usages.

## Svelte

External `.frame` files are the best-supported workflow for editors:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.Dashboard}>
  <aside class={ui.Sidebar}>Sidebar</aside>
</div>
```

Inline Svelte `<style lang="frame">` blocks compile through the Svelte
preprocessor. The server can map diagnostics, completion, hover, and formatting
inside embedded Frame blocks when an editor routes `.svelte` buffers to
`frame_lsp`. The Zed extension intentionally registers the LSP only for
`.frame` files so it does not conflict with Svelte and CSS tooling.

## Zed Setup

Install the extension from `editors/zed`, then build the server:

```bash
cargo build -p frame_lsp
```

The extension resolves the LSP command in this order:

1. `FRAME_LSP`
2. `frame_lsp` on `PATH`
3. `target/debug/frame_lsp` from this repository checkout when available

## Known Limitations

- Workspace symbols, rename, inlay hints, and code lens are not implemented yet.
- Token references are strongest across direct includes; full workspace-wide
  indexing is planned.
- The formatter is line-oriented because the parser does not preserve comments
  in the AST yet.

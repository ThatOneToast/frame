# LSP

The Frame LSP provides editor intelligence for `.frame` files.

Current support:

- Parser and semantic diagnostics.
- Scope-aware completions.
- Completion snippets for dashboards, percentage dashboards, hover cards, toolbars, and empty states.
- Completion menu documentation for declarations, properties, values, surfaces, colors, and effects.
- Hover docs with Frame intent, generated CSS behavior, and Svelte examples.
- Document formatting for `.frame` documents.
- Document symbols.
- Go-to-definition for grid references and named grid sections.
- References for grid declarations and grid sections.
- Document links to Frame markdown docs.
- Code actions for common fixes and layout scaffolds.
- Semantic tokens.
- Folding ranges.

## Scope-aware Completions

At file root, completions suggest declarations only:

```txt
tokens grid area card stack row button text center split overlay dock
```

Inside a `grid`, completions focus on grid layout:

```frame
grid Dashboard {
  columns
  rows
  gap
  height
  width
  padding
  surface
  align
  justify
}
```

Inside an `area`, `in` suggests grid declarations from the current document, and `place` suggests known columns from the referenced grid.

Inside `hover`, `focus`, `active`, and `disabled`, completions suggest effects like `lift`, `glow`, `brighten`, `dim`, `blur`, `press`, `ring`, `scale`, `fade`, and `slide`.

Property values are contextual:

```frame
surface panel
surface gradient dusk
width 50%
rows auto
align center
justify between
color accent
```

Chained values narrow as you type:

```frame
columns responsive
```

then suggests:

```txt
cards
```

```frame
surface gradient
```

then suggests:

```txt
dusk midnight aurora
```

## Svelte Style Blocks

Inline Svelte `<style lang="frame">` blocks compile through the Svelte preprocessor and can be routed through the Frame LSP when the editor sends the `.svelte` buffer to `frame_lsp`.

The server detects Frame style blocks, maps diagnostics back to the Svelte buffer, and only serves completion/hover/formatting inside the Frame block. Outside the block, Frame completions return empty results so they do not dominate normal Svelte or CSS editing.

Shared app styles can still live in external `.frame` files when generated `generated.css` and `generated.ts` exports are preferred.

## Code Actions

The LSP can:

- Replace close typos like `pannel` with `panel`.
- Create a missing grid referenced by an area.
- Add a missing `place` line.
- Create matching areas from a named grid.
- Convert three named columns to `columns 25% 50% 25%`.
- Add hover lift/glow effects to cards.

See `docs/code-actions.md`.

## Navigation

Go-to-definition supports:

- `in Dashboard` to `grid Dashboard`.
- `place sidebar` to the matching `columns sidebar ...` token.

References include grid declarations, `in GridName` usages, named grid columns, and matching `place` usages.

## Zed Setup

Install the local extension from `editors/zed`, then build the server:

```bash
cargo build -p frame_lsp
```

The extension associates `.frame` files with Frame and registers `frame_lsp` only for the Frame language.

The extension resolves the LSP command in this order:

1. `FRAME_LSP`
2. `frame_lsp` on `PATH`
3. `/Users/whitebread/projects/svelte/frame/target/debug/frame_lsp`

## Known Limitations

- The current formatter is line-oriented because the parser does not preserve comments in the AST yet.
- Workspace symbols, rename, inlay hints, and code lens are not implemented yet.

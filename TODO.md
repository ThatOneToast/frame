# TODO.md

## Current Goal

Finish editor intelligence, Svelte integration, and expand Frame into a practical design-intent styling language.

Current pipeline:

```txt
.frame file
  -> parse
  -> validate
  -> generate CSS + TS
  -> use in Svelte
  -> edit with Zed syntax highlighting and LSP support
```

## Phase 1 — Workspace Setup

- [x] Run `cargo check --workspace`.
- [x] Ensure all crates compile.
- [x] Add shared crate documentation.
- [ ] Add or verify CI workflow.

## Phase 2 — AST

Required structures:

- [x] `Document`
- [x] `Declaration`
- [x] `DeclarationKind`
- [x] `Block`
- [x] `Statement`
- [x] `Span`
- [x] `Identifier`

Declaration kinds:

- [x] `Grid`
- [x] `Area`
- [x] `Card`
- [x] `Stack`
- [x] `Row`
- [x] `Button`
- [x] `Text`
- [x] `Tokens`
- [x] `Center`
- [x] `Split`
- [x] `Overlay`
- [x] `Dock`

## Phase 3 — Parser

- [x] Ignore comments.
- [x] Parse top-level declarations.
- [x] Parse declaration names.
- [x] Parse `{ ... }` blocks.
- [x] Parse bare statement lines.
- [x] Parse nested state blocks like `hover { ... }`.
- [x] Return diagnostics instead of panicking.
- [ ] Preserve comments for formatting.
- [ ] Parse or tolerate responsive blocks:
  - [ ] `mobile`
  - [ ] `tablet`
  - [ ] `desktop`
  - [ ] `wide`
- [x] Add tests for expanded declarations:
  - [x] `center`
  - [x] `split`
  - [x] `overlay`
  - [x] `dock`

## Phase 4 — Semantic Validation

Completed diagnostics:

- [x] Unknown declaration kind.
- [x] Duplicate declaration names.
- [x] `area` missing `in`.
- [x] `area` references unknown grid.
- [x] Invalid spacing value.
- [x] Invalid surface value.
- [x] Invalid hover effect.

Add diagnostics:

- [x] Invalid layout values.
- [x] Invalid typography values.
- [x] Invalid positioning values.
- [ ] Invalid responsive values.
- [ ] Invalid state block names.
- [ ] Invalid effect values inside state blocks.
- [ ] Better diagnostics with suggestions.

## Phase 5 — CSS Codegen

Generated output currently supports:

- [x] `grid AppShell`
- [x] named columns
- [x] responsive card grids
- [x] `area Sidebar in AppShell`
- [x] `place sidebar`
- [x] `card`
- [x] `stack`
- [x] `row`
- [x] `surface`
- [x] `padding`
- [x] `gap`
- [x] `radius`
- [x] `shadow`
- [x] `hover lift`
- [x] `hover glow`
- [x] `hover brighten`

Add support for:

- [x] `center`
- [x] `split`
- [x] `overlay`
- [x] `dock`
- [x] `button`
- [x] `text`
- [x] `width`
- [x] `height`
- [x] `min-width`
- [x] `max-width`
- [x] `min-height`
- [x] `max-height`
- [x] `position`
- [x] `offset`
- [x] `z`
- [x] `border`
- [x] `theme`
- [x] `text bright`
- [x] `text muted`
- [x] `text accent`
- [x] `font`
- [x] `size`
- [x] `weight`
- [x] `focus`
- [x] `active`
- [x] `disabled`
- [ ] responsive blocks

## Phase 6 — TypeScript Codegen

- [x] Generate `ui` class export object.
- [x] Generate `UiClass` type.
- [ ] Add optional grouped exports if useful.
- [ ] Ensure generated names remain stable as syntax expands.

## Phase 7 — CLI

Commands:

- [x] `frame check <file>`
- [x] `frame compile <file> --out <dir>`
- [x] `frame format <file>`
- [x] `frame format <file> --check`
- [x] `frame watch <file> --out <dir>`

## Phase 8 — Zed Extension

Syntax highlighting:

- [x] `editors/zed/extension.toml`
- [x] `editors/zed/languages/frame/config.toml`
- [x] Tree-sitter grammar package.
- [x] Highlight queries.

LSP:

- [x] Run `frame_lsp`.
- [x] Diagnostics.
- [x] Completion items for declaration keywords.
- [x] Completion items for property keywords.
- [x] Completion items for token values.
- [x] Completion items for effect keywords.
- [x] Hover docs for common concepts.
- [x] Formatting.
- [x] Zed docs for enabling the LSP.

## Phase 9 — Svelte Integration

- [x] Document current generated CSS/TS usage.
- [x] Add `examples/svelte/README.md`.
- [x] Implement `frame watch`.
- [ ] Add Vite plugin if practical.
- [x] Recompile `.frame` on file changes.
- [x] Print diagnostics during Svelte dev.
- [x] Keep generated files importable from Svelte.
- [x] Add example Svelte component using generated classes.
- [x] Add troubleshooting docs for missing generated files.

## Phase 10 — Docs

Create or expand:

- [x] `docs/language.md`
- [x] `docs/grid.md`
- [x] `docs/layout.md`
- [x] `docs/cards.md`
- [x] `docs/surfaces.md`
- [x] `docs/effects.md`
- [x] `docs/typography.md`
- [x] `docs/svelte.md`
- [x] `docs/lsp.md`
- [x] `docs/examples.md`

Docs should include plentiful examples and explain the design intent behind each keyword.

## Phase 11 — Tests

Completed:

- [x] Parser declaration tests.
- [x] Parser nested block tests.
- [x] CSS codegen behavior tests.
- [x] TS codegen behavior tests.
- [x] CLI integration tests.

Add:

- [x] Formatter tests.
- [ ] Watch command tests where practical.
- [x] LSP completion tests.
- [x] LSP hover tests.
- [x] LSP formatting tests.
- [ ] Svelte integration smoke test.
- [x] Expanded CSS concept codegen tests.
- [ ] Documentation example compile tests where practical.

## Phase 12 — Expanded Frame Concepts

Add native support for common design concepts without turning Frame into raw CSS.

### Layout

- [x] `center`
- [x] `split`
- [x] `overlay`
- [x] `dock`
- [x] `align`
- [x] `justify`

### Sizing

- [x] `width fill`
- [x] `width content`
- [x] `width screen`
- [x] `width sidebar`
- [x] `height screen`
- [x] `height fill`
- [x] `min-height screen`
- [x] `max-width content`

### Positioning

- [x] `position relative`
- [x] `position absolute top-right`
- [x] `position sticky top`
- [x] `offset small`
- [x] `z above`
- [x] `z modal`
- [x] `z overlay`

### Color / Surfaces

- [x] `surface panel`
- [x] `surface main`
- [x] `surface glass`
- [x] `surface raised`
- [x] `surface flat`
- [x] `surface gradient dusk`
- [x] `surface gradient midnight`
- [x] `surface gradient aurora`
- [x] `text bright`
- [x] `text muted`
- [x] `text accent`
- [x] `theme danger`
- [x] `theme success`
- [x] `theme warning`

### Borders

- [x] `border none`
- [x] `border soft`
- [x] `border accent`
- [x] `border danger`
- [x] `border success`
- [x] `border width small`

### Effects

- [x] `shadow none`
- [x] `shadow soft`
- [x] `shadow medium`
- [x] `shadow deep`
- [x] `glow none`
- [x] `glow accent`
- [x] `glow danger`
- [x] `glow success`
- [x] `blur none`
- [x] `blur background`
- [x] `blur heavy`

### Interactions

- [x] `hover { lift small }`
- [x] `hover { glow accent }`
- [x] `hover { brighten subtle }`
- [x] `focus { ring accent }`
- [x] `active { press }`
- [x] `disabled { dim medium }`

### Typography

- [x] `size heading`
- [x] `size body`
- [x] `size caption`
- [x] `weight bold`
- [x] `weight semibold`
- [x] `weight normal`
- [x] `font mono`
- [x] `color bright`
- [x] `color muted`

### Responsive

- [ ] `mobile { stack }`
- [ ] `mobile { hide Inspector }`
- [ ] `tablet { columns content }`
- [ ] `desktop { columns sidebar content inspector }`

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
- [x] `Include`
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
- [x] Parse root `#include` statements.
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
- [x] Invalid custom color token hex values.
- [x] Invalid gradient stop colors, percentages, angles, and stop counts.
- [x] Invalid border, transition, duration, ease, animation, and z-layer values.
- [ ] Invalid responsive values.
- [ ] Invalid state block names.
- [x] Invalid effect values inside state blocks.
- [x] Better diagnostics with suggestions.

## Phase 5 — CSS Codegen

Generated output currently supports:

- [x] `grid AppShell`
- [x] named columns
- [x] responsive card grids
- [x] vertical named grid flow with section spacing controls
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
- [x] custom color tokens
- [x] custom gradient tokens
- [x] layered corner gradient tokens
- [x] `background brand`
- [x] `background hero-gradient`
- [x] `advanced { css "property" value }`
- [x] `padding top medium`, `padding x medium`, and `anchor top`.
- [x] `grid flow vertical` and `section name { padding ... }`.
- [x] `transition`
- [x] `duration`
- [x] `ease`
- [x] `animation`
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
- [x] `frame compile-stdin --css-only`
- [x] `frame format <file>`
- [x] `frame format <file> --check`
- [x] `frame watch <file> --out <dir>`
- [x] `--include <dir>` for `check`, `compile`, and `watch`.
- [x] `frame init svelte`
- [x] `frame init svelte --dry-run`

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
- [x] Scope-aware completions.
- [x] Embedded Svelte `<style lang="frame">` routing.
- [x] Completion snippets for dashboards, hover cards, toolbars, and empty states.
- [x] Go-to-definition for grid and grid section references.
- [x] References for grid declarations and grid sections.
- [x] Document symbols.
- [x] Document links to Frame docs.
- [x] Code actions for typo fixes and common layout scaffolds.
- [x] Semantic tokens.
- [x] Folding ranges.
- [x] Include highlighting, document links, missing include diagnostics, and include completions.
- [x] Imported grid/color/gradient symbols in completions, hover, diagnostics, and go-to-definition.
- [x] Categorized completion groups for snippets, declarations, project symbols, layout, visual, motion, typography, token, value, include, and advanced items.
- [x] Scope-specific snippets for responsive grids, container queries, component states, animation controls, and keyframes.
- [x] Contextual value hover docs for grid sections, breakpoints, spacing tokens, color intent, animation controls, and keyframe percentages.
- [x] Teacher-style diagnostics for raw CSS property aliases and area declarations missing placement.
- [x] Imported keyframes symbols in completions, hover, and go-to-definition.

## Phase 9 — Svelte Integration

- [x] Document current generated CSS/TS usage.
- [x] Add `examples/svelte/README.md`.
- [x] Implement `frame watch`.
- [x] Add Vite plugin.
- [x] Add Svelte preprocessor.
- [x] Support `<style lang="frame">` CSS output.
- [x] Add `frame init svelte` setup command.
- [x] Recompile `.frame` on file changes.
- [x] Pass Vite plugin `include` paths to the CLI.
- [x] Print diagnostics during Svelte dev.
- [x] Keep generated files importable from Svelte.
- [x] Add example Svelte component using generated classes.
- [x] Add example Svelte route using inline Frame styles.
- [x] Add troubleshooting docs for missing generated files.

## Phase 10 — Docs

Create or expand:

- [x] `docs/language.md`
- [x] `docs/grid.md`
- [x] `docs/layout.md`
- [x] `docs/cards.md`
- [x] `docs/surfaces.md`
- [x] `docs/colors.md`
- [x] `docs/effects.md`
- [x] `docs/typography.md`
- [x] `docs/svelte.md`
- [x] `docs/vite.md`
- [x] `docs/style-blocks.md`
- [x] `docs/lsp.md`
- [x] `docs/areas.md`
- [x] `docs/spacing.md`
- [x] `docs/sizing.md`
- [x] `docs/interactions.md`
- [x] `docs/diagnostics.md`
- [x] `docs/code-actions.md`
- [x] `docs/examples.md`
- [x] `docs/setup.md`
- [x] `docs/imports.md`
- [x] `docs/tokens.md`
- [x] `docs/borders.md`
- [x] `docs/animations.md`
- [x] `docs/agents/README.md`
- [x] `docs/agents/language-cheatsheet.md`
- [x] `docs/agents/svelte-patterns.md`
- [x] `docs/agents/recipes.md`
- [x] `docs/agents/troubleshooting.md`

Docs should include plentiful examples and explain the design intent behind each keyword.

## Phase 11 — Tests

Completed:

- [x] Parser declaration tests.
- [x] Parser nested block tests.
- [x] CSS codegen behavior tests.
- [x] TS codegen behavior tests.
- [x] CLI integration tests.
- [x] CLI stdin compile tests.
- [x] CLI init tests.
- [x] Svelte preprocessor helper tests.
- [x] Vite plugin option/helper tests.

Add:

- [x] Formatter tests.
- [ ] Watch command tests where practical.
- [x] LSP completion tests.
- [x] LSP hover tests.
- [x] LSP formatting tests.
- [x] LSP code action tests.
- [x] LSP definition and reference tests.
- [x] LSP semantic token tests.
- [x] LSP folding tests.
- [ ] Full Svelte integration smoke test.
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
- [x] `width 25%`
- [x] `height 50%`
- [x] percentage grid columns
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
- [x] `background main`
- [x] `background panel`
- [x] `background danger`
- [x] Expanded semantic palette tokens.

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
- [x] `below tablet { columns content }`
- [x] `above desktop { columns sidebar content inspector }`
- [x] `between tablet desktop { ... }`
- [x] `container narrow { columns content }`
- [ ] `tablet { columns content }`
- [ ] `desktop { columns sidebar content inspector }`

## Phase 13 — LSP Teacher Experience

- [x] Group completion results into identifiable categories with stable sort order.
- [x] Suggest scoped snippets based on root, grid, component, state, keyframes, and animation-block context.
- [x] Surface available project symbols, including grids, grid sections, colors, gradients, and keyframes, as distinct completion items.
- [x] Teach values through hover docs that depend on the current statement.
- [x] Explain raw CSS property mistakes with Frame equivalents and escape-hatch guidance.
- [x] Explain missing area placement with concrete `place`, `col`, or `row` examples.

# TODO.md

## Current Goal

Implement the first vertical slice:

```txt
.frame file -> parse -> validate -> generate CSS + TS -> use in Svelte
```

## Phase 1 — Workspace Setup

- [x] Run `cargo check --workspace`.
- [x] Ensure all crates compile.
- [x] Add shared crate documentation.
- [ ] Add CI workflow later.

## Phase 2 — AST

Create AST types in `crates/frame_core/src/ast.rs`.

Required structures:
- [x] `Document`
- [x] `Declaration`
- [x] `DeclarationKind`
- [x] `Block`
- [x] `Statement`
- [x] `Span`
- [x] `Identifier`

Initial declaration kinds:
- [x] `Grid`
- [x] `Area`
- [x] `Card`
- [x] `Stack`
- [x] `Row`
- [x] `Button`
- [x] `Text`
- [x] `Tokens`

## Phase 3 — Parser

Implement parser in `crates/frame_parser`.

Start simple:
- [x] Ignore comments.
- [x] Parse top-level declarations.
- [x] Parse declaration names.
- [x] Parse `{ ... }` blocks.
- [x] Parse bare statement lines.
- [x] Parse nested state blocks like `hover { ... }`.
- [x] Return diagnostics instead of panicking.

Example input:

```frame
card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large

  hover {
    lift small
    glow accent
  }
}
```

## Phase 4 — Semantic Validation

Implement validation in `frame_core::semantic`.

Diagnostics:
- [x] Unknown declaration kind.
- [x] Duplicate declaration names.
- [x] `area` missing `in`.
- [x] `area` references unknown grid.
- [x] Invalid spacing value.
- [x] Invalid surface value.
- [x] Invalid hover effect.

## Phase 5 — CSS Codegen

Implement in `crates/frame_codegen/src/css.rs`.

Generated output must support:
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

## Phase 6 — TypeScript Codegen

Implement in `crates/frame_codegen/src/typescript.rs`.

Output: implemented.

```ts
export const ui = {
  AppShell: 'fr-AppShell',
  Sidebar: 'fr-Sidebar'
} as const;

export type UiClass = keyof typeof ui;
```

## Phase 7 — CLI

Implement in `crates/frame_cli`.

Commands:
- [x] `frame check <file>`
- [x] `frame compile <file> --out <dir>`
- [ ] `frame format <file>`
- [ ] `frame watch <file> --out <dir>`

## Phase 8 — Zed Extension

Scaffold:
- [x] `editors/zed/extension.toml`
- [x] `editors/zed/languages/frame/config.toml`
- [x] Tree-sitter grammar package.
- [x] Highlight queries.

LSP later:
- [x] Run `frame_lsp`.
- [x] Diagnostics.
- [ ] Completion.
- [ ] Hover docs.
- [ ] Formatting.

## Phase 9 — Tests

Add tests:
- [x] Parser declaration tests.
- [x] Parser nested block tests.
- [x] CSS codegen behavior tests.
- [x] TS codegen behavior tests.
- [x] CLI integration tests.

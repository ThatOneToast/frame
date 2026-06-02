# AGENTS.md

## Project Mission

Build **Frame**, a CSS DSL for Svelte projects.

Frame should let developers describe UI intent without needing to understand raw CSS syntax first. The compiler should emit normal CSS and TypeScript class exports.

The language should feel like this:

```frame
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

card HoverCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
  }
}
```

## Non-Negotiable Design Goals

1. **Do not recreate CSS with different punctuation.**
   - Frame should be design-intent-first.
   - Avoid CSS property names unless inside an explicit advanced escape hatch.

2. **`grid` is a first-class keyword.**
   - Grid layout must support named columns, named rows, responsive card grids, and child placement.
   - Areas/items must be able to claim their own space.

3. **Svelte is the first integration target.**
   - Compile `.frame` files to:
     - `generated.css`
     - `generated.ts`
   - The generated TypeScript should export stable class names.

4. **Editor tooling matters from day one.**
   - Parser diagnostics must be structured enough for LSP.
   - Zed extension support is part of the roadmap.
   - Syntax highlighting and semantic diagnostics should be planned early.

5. **Rust workspace architecture is required.**
   - Keep crates separated.
   - Do not cram parser, codegen, CLI, and LSP into one crate.

## Repository Architecture

```txt
crates/
  frame_core/
    src/
      ast.rs
      diagnostics.rs
      lib.rs
      semantic.rs
      tokens.rs

  frame_parser/
    src/
      lexer.rs
      lib.rs
      parser.rs
      tests.rs

  frame_codegen/
    src/
      css.rs
      lib.rs
      typescript.rs

  frame_cli/
    src/
      main.rs

  frame_lsp/
    src/
      main.rs

editors/
  zed/
    extension.toml
    languages/frame/config.toml
    tree-sitter-frame/

examples/
  svelte/
    src/lib/frame/app.frame
```

## Implementation Rules

- Prefer small modules with clear responsibility.
- Add tests for every parser and codegen feature.
- Keep compiler output deterministic.
- Use snapshot tests where useful.
- Avoid adding dependencies until needed.
- CLI should fail with useful diagnostics, not panics.
- Generated CSS should be readable enough to debug.
- Generated TS exports should be stable.

## Initial Language Scope

Implement these top-level declarations first:

```frame
tokens AppTheme { ... }
grid AppShell { ... }
area Sidebar { ... }
card QuickLinkCard { ... }
stack SettingsPanel { ... }
row Toolbar { ... }
button PrimaryButton { ... }
text MutedText { ... }
```

## MVP Keywords

### Layout

```txt
grid
area
stack
row
center
split
overlay
dock
```

### Grid

```txt
columns
rows
gap
place
in
col
row
span
responsive
cards
compact
comfortable
```

### Surface / Color

```txt
surface
theme
text
background
gradient
glass
panel
main
accent
muted
danger
success
warning
```

### Shape / Space

```txt
padding
gap
margin
radius
border
shadow
height
width
screen
fill
small
medium
large
xlarge
```

### Effects

```txt
hover
focus
active
lift
glow
brighten
dim
blur
press
ring
smooth
```

## MVP Output

A `.frame` file should compile into:

```txt
generated.css
generated.ts
```

Generated TypeScript shape:

```ts
export const ui = {
  AppShell: 'fr-AppShell',
  Sidebar: 'fr-Sidebar',
  QuickLinkCard: 'fr-QuickLinkCard'
} as const;
```

## Testing Expectations

Add tests for:

- Parsing each top-level declaration.
- Parser errors for unknown blocks.
- Grid columns by name.
- Responsive card grid generation.
- Area placement generation.
- Hover effects generation.
- TypeScript class export generation.
- CLI compile command.

## Suggested Commands

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p frame_cli -- check examples/svelte/src/lib/frame/app.frame
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

## Codex Behavior

When implementing:
- Update `TODO.md` after completing tasks.
- Add tests alongside code.
- Prefer complete working vertical slices over many half-finished features.
- Keep generated examples updated.

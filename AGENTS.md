# AGENTS.md

## Project Mission

Build **Frame**, a CSS DSL for Svelte projects.

Frame lets developers describe UI intent without needing to understand raw CSS syntax first. The compiler emits normal CSS and TypeScript class exports.

Frame should feel like this:

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
   - Avoid raw CSS property thinking unless inside an explicit advanced escape hatch.
   - Prefer concepts like `surface panel`, `hover lift`, `columns responsive cards`, and `radius large`.

2. **`grid` is a first-class keyword.**
   - Grid layout must support named columns, named rows, responsive card grids, and child placement.
   - Areas/items must be able to claim their own space.
   - Grid should be powerful enough to replace most direct CSS grid usage.

3. **Svelte is the first integration target.**
   - Compile `.frame` files to:
     - `generated.css`
     - `generated.ts`
   - Generated TypeScript should export stable class names.
   - Svelte examples should stay working.

4. **Editor tooling matters from day one.**
   - Zed syntax highlighting must work.
   - LSP diagnostics, completions, hover docs, and formatting are first-class features.
   - Parser diagnostics must include enough span information for LSP use.
   - Hover documentation should teach users how to use Frame concepts.

5. **Rust workspace architecture is required.**
   - Keep crates separated.
   - Do not cram parser, codegen, CLI, and LSP into one crate.
   - Prefer reusable logic outside protocol-specific LSP glue.

## Repository Architecture

```txt
crates/
  frame_core/
    src/
      ast.rs
      diagnostics.rs
      knowledge.rs
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
      code_actions.rs
      completions.rs
      context.rs
      document_symbols.rs
      document_links.rs
      embedded.rs
      folding.rs
      hover.rs
      formatting.rs
      navigation.rs
      semantic_tokens.rs
      diagnostics.rs

packages/
  frame-svelte/
    src/
      index.ts
      vite.ts
      preprocess.ts
      compile.ts

editors/
  zed/
    extension.toml
    languages/frame/config.toml
    tree-sitter-frame/

examples/
  svelte/
    src/lib/frame/app.frame

docs/
  language.md
  setup.md
  imports.md
  tokens.md
  grid.md
  layout.md
  cards.md
  surfaces.md
  colors.md
  borders.md
  effects.md
  animations.md
  typography.md
  svelte.md
  vite.md
  style-blocks.md
  lsp.md
  examples.md
  agents/
    README.md
    language-cheatsheet.md
    svelte-patterns.md
    recipes.md
    troubleshooting.md
```

## Implementation Rules

- Prefer small modules with clear responsibility.
- Treat LSP diagnostics, completions, hover docs, and formatting as core compiler features, not optional editor polish.
- Keep Svelte integration working through generated `generated.css` and `generated.ts`; the Vite plugin is the default Svelte development loop, with `frame watch` still available as a CLI fallback.
- Expand common design concepts natively with docs and examples before considering raw CSS-like escape hatches.
- Keep documentation plentiful, example-driven, and accurate with the current compiler behavior.
- Add tests for every parser and codegen feature.
- Add tests for LSP helper logic where practical.
- Keep compiler output deterministic.
- Use snapshot tests where useful.
- Avoid adding dependencies until needed.
- CLI should fail with useful diagnostics, not panics.
- Generated CSS should be readable enough to debug.
- Generated TS exports should be stable.
- Keep examples updated when syntax changes.
- Update `TODO.md` after completing work.
- Update `MILESTONES.md` after completing milestones.
- Continue implementing until an issue arises, a question is needed, or tests cannot be made to pass.

## Current Language Scope

Top-level declarations:

```frame
#include base
tokens AppTheme { ... }
grid AppShell { ... }
area Sidebar { ... }
card QuickLinkCard { ... }
stack SettingsPanel { ... }
row Toolbar { ... }
button PrimaryButton { ... }
text MutedText { ... }
center EmptyState { ... }
split AppLayout { ... }
overlay ModalLayer { ... }
dock AppDock { ... }
```

## Core Keywords

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
layout
position
align
justify
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
auto
fill
screen
```

### Surface / Color

```txt
surface
theme
text
background
color
palette
tone
opacity
gradient
glass
panel
main
accent
muted
danger
success
warning
info
primary
secondary
transparent
bright
white
black
gray
slate
red
orange
yellow
green
blue
purple
pink
cyan
```

### Shape / Space

```txt
padding
gap
margin
radius
border
shadow
outline
height
width
min-height
max-height
min-width
max-width
screen
fill
content
small
medium
large
xlarge
pill
full
none
```

### Effects

```txt
hover
focus
active
disabled
lift
glow
brighten
dim
blur
press
ring
smooth
fade
scale
rotate
slide
transition
duration
ease
animation
animate
```

### Typography

```txt
font
size
weight
line
letter
heading
body
caption
mono
bold
semibold
normal
thin
```

### Responsive

```txt
mobile
tablet
desktop
wide
stack
hide
show
only
```

## Desired Output

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

export type UiClass = keyof typeof ui;
```

## LSP Expectations

The LSP should provide:

- diagnostics from parser errors
- diagnostics from semantic validation
- completions for declaration keywords
- completions for property keywords
- completions for token values
- completions for effect keywords
- hover docs for every major concept
- formatting support

Hover docs should explain Frame intent first.

Example hover for `grid`:

```txt
Defines a layout container using Frame's grid system.
Use `columns`, `rows`, `gap`, and child `area` declarations to place content.
```

Example hover for `surface`:

```txt
Sets the visual surface of a component.
Use named surfaces like `panel`, `main`, `glass`, or gradients like `gradient dusk`.
```

## Svelte Expectations

Frame should be easy to use from Svelte.

Example:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Sidebar</aside>

  <main class={ui.Content}>
    <div class={ui.QuickLinks}>
      <a class={ui.QuickLinkCard}>Docs</a>
      <a class={ui.QuickLinkCard}>GitHub</a>
    </div>
  </main>

  <section class={ui.Inspector}>Inspector</section>
</div>
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
- CLI format command.
- CLI watch logic where practical.
- LSP completions.
- LSP hover docs.
- LSP formatting.
- Svelte integration smoke behavior where practical.

## Suggested Commands

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cargo run -p frame_cli -- check examples/svelte/src/lib/frame/app.frame
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame --include examples/svelte/src/lib/frame
cargo run -p frame_cli -- format examples/svelte/src/lib/frame/app.frame
cargo run -p frame_cli -- watch examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

## Codex Behavior

When implementing:

- Update `TODO.md` after completing tasks.
- Update `MILESTONES.md` after completing milestones.
- Add tests alongside code.
- Prefer complete working vertical slices over many half-finished features.
- Keep generated examples updated.
- Keep docs accurate with current syntax.
- Continue implementing until an issue arises, a question is needed, or tests cannot be made to pass.

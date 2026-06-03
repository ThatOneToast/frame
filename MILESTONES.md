# MILESTONES.md

## Milestone 0 — Repository Scaffolding

Goal: create a clean Rust workspace and documentation foundation.

Deliverables:
- [x] `.gitignore`
- [x] `README.md`
- [x] `AGENTS.md`
- [x] `MILESTONES.md`
- [x] `TODO.md`
- [x] Rust workspace crates
- [x] Example `.frame` file
- [x] Zed extension placeholder

---

## Milestone 1 — Parser MVP

Goal: parse the first useful subset of Frame.

Deliverables:
- [x] AST for declarations:
  - [x] `grid`
  - [x] `area`
  - [x] `card`
  - [x] `stack`
  - [x] `row`
  - [x] `button`
  - [x] `text`
- [x] Nested blocks:
  - [x] `hover`
  - [x] `focus`
  - [x] `active`
- [x] Property-like statements:
  - [x] `columns sidebar content inspector`
  - [x] `gap medium`
  - [x] `surface panel`
  - [x] `padding large`
- [x] Friendly parse errors.
- [x] Parser unit tests.

Success criteria:

```bash
cargo test -p frame_parser
```

---

## Milestone 2 — Semantic Model

Goal: validate Frame files after parsing.

Deliverables:
- [x] Unknown keyword diagnostics.
- [x] Duplicate declaration diagnostics.
- [x] `area ... in GridName` validation.
- [x] `place name` validation against grid columns/areas.
- [x] Allowed value tables for spacing, surfaces, effects, radii, and sizes.
- [x] Diagnostic spans suitable for LSP.

Success criteria:
- Invalid files produce useful errors.
- Valid example file passes `frame_cli check`.

---

## Milestone 3 — CSS Codegen MVP

Goal: compile useful Frame declarations into real CSS.

Deliverables:
- [x] Stable class naming.
- [x] Base design token CSS.
- [x] `grid` codegen.
- [x] `area` placement codegen.
- [x] Responsive card grid codegen.
- [x] `card` codegen.
- [x] `stack` and `row` codegen.
- [x] `hover` effects codegen.
- [x] CSS behavior tests.

Success criteria:
- Example `.frame` compiles to readable CSS.
- Svelte can import and use the generated classes.

---

## Milestone 4 — TypeScript Codegen

Goal: make generated classes ergonomic in Svelte.

Deliverables:
- [x] Generate `generated.ts`.
- [x] Export `ui` object.
- [x] Use stable generated class names.
- [x] TypeScript behavior tests.

Success criteria:

```ts
import { ui } from '$lib/frame/generated';
```

works in a Svelte project.

---

## Milestone 5 — CLI

Goal: provide a usable command-line workflow.

Deliverables:
- [x] `frame check <file>`
- [x] `frame compile <file> --out <dir>`
- [x] `frame format <file>`
- [x] `frame watch <file> --out <dir>`
- [x] Exit codes for CI.
- [x] Human-readable diagnostics.

Success criteria:

```bash
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

---

## Milestone 6 — Zed Syntax Highlighting

Goal: make `.frame` files pleasant to edit in Zed.

Deliverables:
- [x] Tree-sitter grammar scaffold.
- [x] Highlight queries.
- [x] Zed extension metadata.
- [x] File extension association for `.frame`.
- [x] Syntax highlighting for:
  - [x] declarations
  - [x] block names
  - [x] keywords
  - [x] effects
  - [x] strings/comments

Success criteria:
- Zed recognizes `.frame` files.
- Basic highlighting works.

---

## Milestone 7 — LSP MVP

Goal: provide editor intelligence.

Deliverables:
- [x] `frame_lsp` executable.
- [x] Publish diagnostics from parser and semantic model.
- [x] Completion items for known keywords.
- [x] Completion items for known token values.
- [x] Hover docs for common concepts.
- [x] Format document support.

Success criteria:
- Zed can run the LSP.
- Invalid Frame files show diagnostics.

---

## Milestone 8 — Svelte Integration

Goal: make Frame feel native in Svelte projects.

Deliverables:
- [x] Vite plugin.
- [x] Svelte preprocessor.
- [x] `<style lang="frame">` support.
- [x] External `.frame` file support.
- [x] Inline style block documentation.
- [x] Generated CSS/TS path configuration.
- [x] `frame watch` development workflow.
- [x] Example Svelte usage documentation.

Success criteria:
- A Svelte component can use generated Frame classes without manual CSS.
- A Svelte component can compile component-local Frame through `<style lang="frame">`.

---

## Milestone 9 — Svelte Setup And Practical Editor Guidance

Goal: make Frame usable inside real Svelte projects with one setup command and clearer editor guidance.

Deliverables:
- [x] `frame init svelte`.
- [x] `frame init svelte --dry-run`.
- [x] Safe Svelte/Vite config updates with backups.
- [x] Initial `src/lib/frame/app.frame` generation.
- [x] Generated `generated.css` and `generated.ts` from init.
- [x] Scope-aware LSP completions.
- [x] Embedded Svelte `<style lang="frame">` LSP routing.
- [x] Practical hover docs with Svelte examples.
- [x] Percentage-based sizing.
- [x] Expanded color and surface tokens.
- [x] Grid, layout, color, surface, Svelte, and LSP docs.

Success criteria:
- A Svelte project can add Frame in one command.
- Frame style blocks receive Frame completions and hover docs.
- Users can build sidebar/content/inspector layouts with named or percentage columns.

---

## Milestone 10 — Production-Grade LSP Guidance

Goal: make the Frame LSP act like an expert editor guide, not just a keyword autocomplete server.

Deliverables:
- [x] Central Frame knowledge base for completion and hover documentation.
- [x] Rich markdown completion docs with Frame and Svelte examples.
- [x] Completion snippets for dashboards, percentage dashboards, hover cards, toolbars, and empty states.
- [x] Stronger diagnostics with suggestions and concept guidance.
- [x] Code actions for typos, missing grids, missing placements, generated areas, percentage columns, and hover effects.
- [x] Go-to-definition for `area in GridName` and `place section`.
- [x] References for grid declarations and grid sections.
- [x] Document symbols with nested state blocks.
- [x] Document links to markdown docs.
- [x] Semantic tokens for declarations, properties, values, colors, and percentages.
- [x] Folding ranges for declarations and state blocks.
- [x] Embedded Svelte `<style lang="frame">` diagnostics, completion, hover, and formatting routing.
- [x] Additional documentation for areas, spacing, sizing, interactions, diagnostics, and code actions.

Success criteria:
- `cargo test --workspace` passes.
- Frame files and inline Svelte Frame style blocks receive practical editor guidance.

---

## Milestone 11 — Styling Vocabulary, Imports, And Highlighting Hardening

Goal: make Frame feel like a complete styling language for real Svelte projects.

Deliverables:
- [x] Tree-sitter grammar support for `#include` and hex color literals.
- [x] Highlight query coverage for declarations, layout properties, color/surface properties, borders, effects, animation, percentages, numbers, and includes.
- [x] Zed highlighting samples for layout, colors, effects, imports, and broad keyword coverage.
- [x] Custom color token syntax: `color brand #7c3aed`.
- [x] CSS variables for custom colors and use in `background`, `color`, `border`, and state effects.
- [x] Custom gradient token syntax with `type linear`, `angle`, and `stop` entries.
- [x] CSS variables for custom gradients and use in `background` and `surface`.
- [x] Layered corner gradients for multi-corner background washes.
- [x] Targeted padding/margin and sticky `anchor` positioning.
- [x] Vertical grid flow with per-section spacing, sizing, and alignment controls.
- [x] Explicit advanced styling escape hatch with scoped `css "property" value` output.
- [x] Expanded surface, color, border, shadow, glow, transition, animation, alignment, position, z-layer, and sizing vocabulary.
- [x] Deterministic keyframes for named animations.
- [x] `#include` support in parser, CLI, Vite plugin, LSP diagnostics, document links, go-to-definition, and completions.
- [x] Include cycle and missing include diagnostics.
- [x] Cross-file LSP completions, hover, diagnostics, and go-to-definition for imported grids and tokens.
- [x] Practical docs for setup, imports, tokens, borders, animations, and examples.

Success criteria:
- `cargo fmt`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- Tree-sitter parser and highlight checks pass for `highlighting.frame` and `imports.frame`.
- `npm test` passes in `packages/frame-svelte`.
- `npm run build` passes in `examples/svelte`.

---

## Milestone 12 — Structured CSS Teacher Slice

Goal: continue the language/LSP overhaul with first-class motion, responsive overrides, and broader centralized language metadata.

Deliverables:
- [x] `keyframes` top-level declaration in the AST and parser.
- [x] Custom keyframes CSS generation with deterministic `frame-Name` names.
- [x] Structured `animation Name { ... }` blocks with duration, delay, ease, iteration, direction, fill, and play-state.
- [x] Responsive declaration blocks: `below`, `above`, and `between`.
- [x] Container query declaration blocks with named container sizes.
- [x] Symbol indexing for custom keyframes and custom animation completions.
- [x] LSP completions for keyframe selectors, keyframe properties, animation options, and custom keyframe references.
- [x] Hover docs for keyframes, responsive blocks, container queries, and animation controls.
- [x] Semantic token coverage for keyframe selectors and responsive/container block keywords.
- [x] Parser, codegen, LSP completion, and semantic validation tests for the new slice.
- [x] README and docs updated to reflect structured full-CSS coverage rather than intentionally hiding CSS.

Success criteria:
- `cargo fmt`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

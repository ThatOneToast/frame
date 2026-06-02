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
- [ ] `frame format <file>`
- [ ] `frame watch <file> --out <dir>`
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
- [ ] Completion items for known keywords.
- [ ] Completion items for known token values.
- [ ] Hover docs for common concepts.
- [ ] Format document support.

Success criteria:
- Zed can run the LSP.
- Invalid Frame files show diagnostics.

---

## Milestone 8 — Svelte Integration

Goal: make Frame feel native in Svelte projects.

Deliverables:
- [ ] Vite plugin or Svelte preprocessor.
- [ ] External `.frame` file support.
- [ ] Optional `<style lang="frame">` exploration.
- [ ] Generated CSS/TS path configuration.
- [ ] Example Svelte app.

Success criteria:
- A Svelte component can use generated Frame classes without manual CSS.

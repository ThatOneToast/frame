# MILESTONES.md

## Milestone 0 — Overhaul Foundation

Goal: prepare the repository for the move from styling-only output to `Frame -> Frame IR -> DOM runtime`.

Deliverables:

- [x] Rewrite project README.
- [x] Rewrite agent guidance.
- [x] Rewrite TODO tracker.
- [x] Rewrite milestones.
- [x] Add CSS coverage tracker.
- [x] Add DOM coverage tracker.
- [x] Add research notes for architecture and runtime design.

Success criteria:

- The repository clearly communicates that Frame is experimental.
- The new direction is documented without implementing runtime code yet.

---

## Milestone 1 — Language Shape

Goal: define the first complete Frame UI syntax slice.

Deliverables:

- [ ] `component` declarations.
- [ ] `view` blocks.
- [ ] `props` blocks.
- [ ] `state` blocks.
- [ ] DOM element declarations.
- [ ] Component invocation syntax.
- [ ] `$value` references.
- [ ] `@handler` references.
- [ ] `Name:StyleName` style binding syntax.
- [ ] `style when` reactive style syntax.
- [ ] `show when`, `disabled when`, and similar condition helpers.
- [ ] Clear unsafe/raw escape hatch syntax.

Success criteria:

- The planned syntax can express a small chat UI without Svelte or React.
- The syntax remains readable and teachable.

---

## Milestone 2 — Parser and AST

Goal: parse the UI language slice while preserving existing styling syntax.

Deliverables:

- [ ] AST for components, views, props, state, elements, text, events, bindings, conditions, loops, and style bindings.
- [ ] Parser support for `$` data references.
- [ ] Parser support for `@` handler references.
- [ ] Parser support for event filters and modifiers.
- [ ] Parser support for explicit style bindings.
- [ ] Parser support for reactive style rules.
- [ ] Comment preservation for formatter work.
- [ ] Parser tests.

Success criteria:

```bash
cargo test -p frame_parser
```

---

## Milestone 3 — Semantic Model

Goal: resolve names and validate UI meaning.

Deliverables:

- [ ] Component symbol table.
- [ ] State and prop symbol table.
- [ ] Handler reference table.
- [ ] Style reference resolution.
- [ ] DOM element validation.
- [ ] Attribute validation.
- [ ] Event validation.
- [ ] Accessibility diagnostics.
- [ ] Unsafe sink diagnostics.
- [ ] Source spans for every diagnostic.

Success criteria:

- Invalid UI declarations produce actionable diagnostics.
- LSP can reuse semantic information.

---

## Milestone 4 — Frame IR

Goal: introduce a renderer-neutral IR as the real compiler output.

Deliverables:

- [ ] IR node model.
- [ ] IR component model.
- [ ] IR style binding model.
- [ ] IR event binding model.
- [ ] IR state/binding model.
- [ ] IR control-flow model.
- [ ] IR source maps.
- [ ] IR version metadata.
- [ ] JSON serialization.
- [ ] Snapshot tests.

Success criteria:

- A parsed Frame component can be lowered into stable IR.
- Renderers do not need to parse Frame source.

---

## Milestone 5 — TypeScript Contracts

Goal: connect Frame UI declarations to external TypeScript logic without inline scripts.

Deliverables:

- [ ] Generated prop types.
- [ ] Generated state types.
- [ ] Generated handler interfaces.
- [ ] Generated event context types.
- [ ] Generated skeleton files.
- [ ] Non-destructive file update strategy.
- [ ] Tests for generated contracts.

Success criteria:

- A developer can write Frame UI first, then fill in generated TypeScript skeletons.

---

## Milestone 6 — DOM Runtime MVP

Goal: render Frame IR directly to the browser DOM.

Deliverables:

- [ ] Runtime package scaffold.
- [ ] Mount/unmount API.
- [ ] Element creation.
- [ ] Text node creation.
- [ ] Attribute/property application.
- [ ] Event listener binding.
- [ ] Escaped text insertion.
- [ ] Style class application.
- [ ] Basic state updates.
- [ ] Patch application.
- [ ] Runtime tests.

Success criteria:

- A small Frame component renders in a browser without Svelte or React.

---

## Milestone 7 — Reactive Runtime

Goal: support practical UI updates.

Deliverables:

- [ ] State dependency tracking.
- [ ] Conditional rendering.
- [ ] Keyed list rendering.
- [ ] Form bindings.
- [ ] Reactive styles.
- [ ] Batched updates.
- [ ] Cleanup lifecycle.
- [ ] Error boundaries or runtime error reporting plan.

Success criteria:

- A chat input can update text, submit, disable while sending, and render messages.

---

## Milestone 8 — Full DOM Coverage Expansion

Goal: expand toward complete HTML and DOM capability.

Deliverables:

- [ ] Element catalog implementation from `TODO-DOM.md`.
- [ ] Attribute catalog implementation from `TODO-DOM.md`.
- [ ] Event catalog implementation from `TODO-DOM.md`.
- [ ] Form behavior coverage.
- [ ] Media behavior coverage.
- [ ] Accessibility coverage.
- [ ] DOM escape hatches.

Success criteria:

- Unsupported DOM features are rare, documented, and intentionally tracked.

---

## Milestone 9 — CSS Coverage Expansion

Goal: continue evolving Frame styling toward complete CSS capability through structured syntax.

Deliverables:

- [ ] Remaining layout coverage.
- [ ] Remaining typography coverage.
- [ ] Remaining visual effects coverage.
- [ ] Remaining transforms/transitions/animations coverage.
- [ ] Remaining responsive/container query coverage.
- [ ] Remaining advanced CSS coverage.
- [ ] Continued advanced escape hatch support.

Success criteria:

- `TODO-CSS.md` is mostly complete or each missing CSS area has a clear reason.

---

## Milestone 10 — LSP Teacher Experience for UI

Goal: make Frame's editor support teach the full UI language.

Deliverables:

- [ ] UI syntax highlighting.
- [ ] DOM element completions.
- [ ] Attribute completions.
- [ ] Event completions.
- [ ] State/prop/handler completions.
- [ ] Hover docs for UI syntax.
- [ ] Go-to-definition for handlers and styles.
- [ ] Code actions for missing handlers and styles.
- [ ] Diagnostics for unsafe/invalid DOM usage.

Success criteria:

- The LSP can guide a new user through building a small UI.

---

## Milestone 11 — Web App Target

Goal: make Frame usable as a standalone web UI language.

Deliverables:

- [ ] `frame init web`.
- [ ] Dev server or Vite-compatible runtime flow.
- [ ] Browser example app.
- [ ] Build output docs.
- [ ] Debugging docs.

Success criteria:

- A developer can create a small Frame web app without Svelte or React.

---

## Milestone 12 — Tauri/WebView Target

Goal: use the DOM runtime inside desktop apps.

Deliverables:

- [ ] `frame init tauri` plan or scaffold.
- [ ] Tauri example app.
- [ ] Desktop command/event boundary docs.
- [ ] Security docs for desktop APIs.
- [ ] Packaging notes.

Success criteria:

- The same Frame UI model can run in a browser and a Tauri WebView.

---

## Milestone 13 — Native Renderer Research

Goal: evaluate future renderers after the IR has proven itself.

Potential targets:

- [ ] winit/wgpu custom renderer.
- [ ] egui backend.
- [ ] iced backend.
- [ ] slint backend.
- [ ] platform-native widgets.

Success criteria:

- Native target decisions are based on the stable IR, not premature assumptions.

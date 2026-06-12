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

- [x] `component` declarations.
- [x] `view` blocks.
- [x] `props` blocks.
- [x] `state` blocks.
- [x] `slot` declarations.
- [x] Initial UI element declarations.
- [x] Initial component invocation syntax.
- [x] `$value` references (state and props).
- [x] `@handler` references.
- [x] `Name:StyleName` style binding syntax.
- [x] Initial reactive style syntax: `style when $state = StyleName` and `style StyleName when $state`.
- [x] `show when`, `disabled when`, and similar condition helpers.
- [x] Clear unsafe/raw escape hatch syntax.

Success criteria:

- The planned syntax can express a small chat UI without Svelte or React.
- The syntax remains readable and teachable.

---

## Milestone 2 — Parser and AST

Goal: parse the UI language slice while preserving existing styling syntax.

Deliverables:

- [x] AST for components, views, props, state, slots, elements, text, events, bindings, conditions, and style bindings.
- [x] Parser support for `$` data references (state and props).
- [x] Parser support for `@` handler references.
- [x] Parser support for event filters and modifiers.
- [x] Parser support for explicit style bindings.
- [x] Parser support for initial reactive style rules.
- [x] Parser support for keyed and non-keyed list syntax.
- [ ] Comment preservation for formatter work.
- [x] Parser tests for all supported UI syntax.

Success criteria:

```bash
cargo test -p frame_parser
```

---

## Milestone 3 — Semantic Model

Goal: resolve names and validate UI meaning.

Deliverables:

- [x] Component symbol validation.
- [x] State and prop symbol table.
- [x] Handler reference collection.
- [x] Style reference validation.
- [x] UI element validation.
- [ ] Attribute validation.
- [x] Event validation.
- [x] Accessibility diagnostics.
- [x] Unsafe sink diagnostics.
- [x] URL-bearing attribute detection.
- [x] Source spans for every diagnostic.

Success criteria:

- Invalid UI declarations produce actionable diagnostics.
- LSP can reuse semantic information.

---

## Milestone 4 — Frame IR

Goal: introduce a renderer-neutral IR as the real compiler output.

Deliverables:

- [x] IR node model.
- [x] IR component model.
- [x] IR style binding model.
- [x] IR event binding model.
- [x] IR state/binding model.
- [x] IR prop model.
- [x] IR slot model.
- [x] IR capability flags.
- [x] IR control-flow model.
- [x] IR source spans.
- [x] IR version metadata.
- [x] JSON serialization.
- [x] Stable string tests.

Success criteria:

- A parsed Frame component can be lowered into stable IR.
- Renderers do not need to parse Frame source.

---

## Milestone 5 — TypeScript Contracts

Goal: connect Frame UI declarations to external TypeScript logic without inline scripts.

Deliverables:

- [x] Generated prop types.
- [x] Generated state types.
- [x] Generated handler interfaces.
- [x] Generated event context types.
- [x] Generated handler skeleton files with TODO comments.
- [x] Event-specific handler type aliases (FramePressEvent, FrameInputEvent, etc.).
- [x] Non-destructive file update strategy (skeletons only written if missing).
- [x] Tests for generated contracts and skeletons.

Success criteria:

- A developer can write Frame UI first, then fill in generated TypeScript skeletons.

---

## Milestone 6 — Runtime Architecture Preparation

Goal: freeze renderer-neutral runtime architecture before DOM rendering begins.

Status: **Complete when tests pass.**

Deliverables:

- [x] Runtime architecture document.
- [x] Renderer-neutral runtime crate.
- [x] Component runtime model.
- [x] Reactive state store with dependency tracking, dirty tracking, subscriptions, and batching.
- [x] Runtime prop model.
- [x] Slot IR and runtime metadata.
- [x] Conditional rendering metadata.
- [x] Keyed and non-keyed list IR/runtime metadata.
- [x] Runtime-neutral event descriptors and handler lookup metadata.
- [x] Accessibility architecture document and diagnostics.
- [x] Security architecture document and diagnostics.
- [x] IR spec document.
- [x] Runtime tests.
- [x] LSP hover/completion updates for new syntax and diagnostics.
- [x] Zed grammar, highlighting, and sample updates.

Success criteria:

- Renderer work can begin without changing the IR/runtime boundaries.
- No DOM element creation, browser node creation, document access, mounting, patching, or hydration exists.

---

## Milestone 7 — DOM Runtime MVP

Goal: render Frame IR directly to the browser DOM.

Status: **Phase 4 implemented with accessibility, event hardening, input behavior, lifecycle cleanup, and diagnostics maturity.** Broader platform features remain open.

Deliverables:

- [x] Runtime package scaffold.
- [x] Mount/unmount API.
- [x] Element creation.
- [x] Text node creation.
- [x] Attribute/property application.
- [x] Event listener binding.
- [x] Escaped text insertion.
- [x] Style class application.
- [x] Basic state updates.
- [x] Patch application.
- [x] Runtime tests.
- [x] Practical HTML element coverage.
- [x] Global attributes, `data-*`, and `aria-*`.
- [x] URL safety checks and diagnostics.
- [x] Form controls, submit/reset events, and `selected` binding.
- [x] Batched scheduler with deterministic flush order.
- [x] Hardened keyed/non-keyed list reconciliation.
- [x] Nested component and list cleanup accounting.
- [x] Runtime diagnostics with component/source context.
- [x] Accessibility defaults for semantic primitives.
- [x] Keyboard activation for action-like controls.
- [x] Disabled state reflected in DOM for interactive elements.
- [x] `placeholder`, `readonly`, and `disabled` property support.
- [x] `label` → `aria-label` attribute mapping.
- [x] Mount-time handler validation with debug warnings.
- [x] Mount-time prop type validation.
- [x] Debug output explaining queued and flushed patches.
- [x] Expanded runtime tests covering accessibility, events, input behavior, lifecycle, and diagnostics.

Success criteria:

- Frame components render practical browser UI screens without Svelte or React.
- Runtime behaves predictably for accessibility, events, forms, and lifecycle.

---

## Milestone 8 — Reactive Runtime

Goal: support practical UI updates.

Deliverables:

- [x] State dependency tracking.
- [x] Conditional rendering.
- [x] Keyed list rendering.
- [x] Form bindings.
- [x] Reactive styles.
- [x] Batched updates.
- [x] Cleanup lifecycle.
- [x] Error boundaries or runtime error reporting plan.

Success criteria:

- A chat input can update text, submit, disable while sending, and render messages.

---

## Milestone 9 — Semantic Language Redesign

Goal: redesign Frame's authoring model around UI intent, semantics, accessibility, and developer goals instead of browser implementation details.

Status: **Phase 1 implementation complete. Completions, validation, and diagnostics expanded.**

Out of scope:

- SSR
- hydration
- routing
- portals
- suspense
- async components
- further runtime rendering work

Deliverables:

- [x] Existing syntax research across HTML, JSX, React, Vue, Svelte, SwiftUI, Jetpack Compose, Flutter, Slint, and Figma concepts.
- [x] HTML leakage analysis.
- [x] Semantic UI primitive proposal.
- [x] Intent-based API proposal.
- [x] Layout system proposal.
- [x] Form system proposal.
- [x] Accessibility-first model proposal.
- [x] Compiler mapping strategy.
- [x] LSP teaching requirements.
- [x] Finalize first implementation slice.
- [x] Finalize compatibility and migration policy: no legacy author-facing HTML-like UI syntax.
- [x] Parser support for semantic primitives.
- [x] Semantic validation for semantic primitives and removed browser words.
- [x] IR support for `semantic_kind` and `render_kind`.
- [x] LSP hover/completion/semantic token updates.
- [x] Zed grammar/highlighting updates.
- [x] Semantic examples.
- [x] `docs/semantic-lowering.md`.
- [x] Primitive-specific property validation with teacher-like diagnostics.
- [x] Generated IR golden fixtures and cross-language structural tests.
- [x] Canonical language registry in `crates/frame_core/src/language.rs` as the single source of truth for all language concepts.
- [x] LSP, parser, semantic model, and Zed extension consuming the canonical registry.
- [x] AST-backed `SemanticCursor` model in `crates/frame_lsp/src/ide/cursor.rs` as unified cursor context.
- [x] Completions, hover, diagnostics, and references migrated to `SemanticCursor`.
- [x] Internal `ReferenceKind` classification and `includeDeclaration` honored in LSP references.
- [x] Context-aware primitive body completions (id, class, rel, data-*, text, on, bind, when, hidden, for).
- [x] Duplicate property and duplicate event handler validation.
- [x] Empty declaration diagnostics (empty view, empty primitive body).
- [x] Component/primitive name collision warnings.
- [x] Local unused state/prop hints.
- [ ] Update `TODO-DOM.md` after any DOM coverage changes are implemented.
- [ ] Update `TODO-CSS.md` after any styling coverage changes are implemented.

Success criteria:

- Frame has a documented semantic vocabulary that can express common app UI without raw HTML tags as the primary syntax.
- DOM mappings are explicit and renderer-overridable.
- Accessibility defaults are documented before parser, semantic, IR, or runtime changes begin.

---

## Milestone 10 — Full DOM Coverage Expansion

Status: **Paused until the semantic language redesign milestone is resolved.**

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

## Milestone 10B — Page Root Styling

Goal: enable full-page dark-themed apps by supporting `html` and `page-body` declarations.

Deliverables:

- [x] `html` declaration that emits a global `html` CSS rule.
- [x] `page-body` declaration that emits a global `body` CSS rule.
- [x] `page-body` emits default `min-height: 100vh` and `margin: 0`.
- [x] Raw CSS values supported (hex colors, etc.).
- [x] Grid conflict diagnostics for `columns` + `tracks`.
- [x] Grid conflict diagnostics for duplicate column names.
- [x] Unknown explicit style binding diagnostics.
- [x] LSP completions for `html` and `page-body`.
- [x] LSP hover docs for `html` and `page-body`.
- [x] Tree-sitter grammar updated with `html` and `page-body`.
- [x] Dashboard regression test with page root styling.
- [x] Codegen tests for `html`/`page-body` CSS emission.
- [x] Diagnostic tests for grid conflicts and style bindings.

Success criteria:

- Full-page dark-themed apps work without white body backgrounds.
- `html` and `page-body` emit correct global CSS rules.
- Grid conflicts and unknown style bindings produce actionable diagnostics.

---

## Milestone 11 — CSS Coverage Expansion

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

## Milestone 11B — Style Inheritance

Goal: support declaration-level style reuse through `extends` inheritance.

Status: **Complete.**

Deliverables:

- [x] Parser support for `extends` keyword in style declarations.
- [x] Semantic validation of inheritance kind matching (card extends card, grid extends grid).
- [x] Cycle detection and error diagnostics for circular inheritance.
- [x] Diagnostics for unknown base style references.
- [x] Property merging from parent declarations into child declarations.
- [x] CSS output includes both base and overridden properties.
- [x] LSP diagnostics for invalid inheritance.
- [x] Tests covering single-level, multi-level, cycle, and kind mismatch cases.

Success criteria:

- `card Child extends Parent { ... }` compiles with inherited CSS output.
- Invalid inheritance produces actionable diagnostics.
- Multi-level chains resolve correctly.

---

## Milestone 12 — LSP Teacher Experience for UI

Goal: make Frame's editor support teach the semantic UI language.

Status: **Core teacher features implemented. Expanded completions and validation added.**

Deliverables:

- [x] UI syntax highlighting.
- [x] Semantic primitive completions before DOM element completions.
- [x] Intent-based property completions before raw attribute completions.
- [x] Event completions.
- [x] State/prop/handler completions.
- [x] Hover docs for UI intent and accessibility behavior.
- [x] Go-to-definition for handlers and styles.
- [x] Code action for missing same-file style skeletons.
- [x] Code action to map automatic style lookup nodes to existing styles.
- [x] Multi-file code actions for missing handler, state, and prop skeletons.
- [x] Workspace edits that create symbols in included files.
- [x] Find All References across includes.
- [x] Cross-file diagnostics for unresolved components, duplicate symbols, and shadowed imports.
- [x] Diagnostics for unsafe/invalid DOM usage.
- [x] Project theme file (`app-theme.frame`) completions, hover, definitions, and references.
- [x] Code actions can create missing styles in `app-theme.frame`.
- [x] All-token line diagnostics with clearer wording for value/color issues.
- [x] Expanded semantic token coverage (loop vars, time values, slot names, prop/state names, UI keywords).
- [x] Primitive body completions with id, class, rel, data-*, text, on, bind, when, hidden.
- [x] Event and binding completions with keyboard modifier snippets.
- [x] Completion ranking (loop vars before state before props, local handlers first).
- [x] Duplicate property and duplicate exact event handler diagnostics.
- [x] Empty view and empty primitive body diagnostics.
- [x] Component/primitive name collision warnings.
- [x] Local unused state/prop hints.
- [ ] Cross-file import-aware completions and diagnostics.
- [ ] Full unused symbol detection across includes.
- [ ] Advanced style completions from the symbol index.
- [ ] Code actions / quick fixes for duplicate properties and empty declarations.

Success criteria:

- The LSP can guide a new user through building a small UI.

---

## Milestone 13 — Web App Target

Status: **Scaffolding complete with generated types, skeletons, and standalone example app.**

Goal: make Frame usable as a standalone web UI language.

Deliverables:

- [x] `frame init web`.
- [x] `frame new` with web and svelte templates.
- [x] Vite-compatible runtime flow (`npm run dev` / `npm run build`).
- [x] `frame build` generates typed IR, CSS, contracts, and non-destructive handler skeletons.
- [x] Event-specific handler type aliases in generated contracts.
- [x] Browser example app (`implementations/frame-web-app`) demonstrating semantic layout, input binding, actions, lists, and handlers.
- [x] Build output docs in web template README.
- [x] Debugging docs explaining runtime debug mode and generated files.
- [x] Project theme file (`app-theme.frame`) generated with web template.
- [x] Theme file automatically included in `frame check` and `frame build`.
- [x] `frame build --watch` for automatic rebuilds during development.
- [x] Web template `npm run dev` runs Frame watch and Vite in parallel.
- [ ] Hot reload during dev.
- [ ] Production build optimization docs.

Success criteria:

- A developer can create a small Frame web app without Svelte or React.

---

## Milestone 14 — Tauri/WebView Target

Status: **Paused until the semantic language redesign milestone is resolved.**

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

## Milestone 15 — Native Renderer Research

Status: **Paused until the semantic language redesign milestone is resolved.**

Goal: evaluate future renderers after the IR has proven itself.

Potential targets:

- [ ] winit/wgpu custom renderer.
- [ ] egui backend.
- [ ] iced backend.
- [ ] slint backend.
- [ ] platform-native widgets.

Success criteria:

- Native target decisions are based on the stable IR, not premature assumptions.

## Milestone — Semantic Styling Overhaul (2026-06)

Status: shipped (first batch).

- Normalized style layer (`frame_core::style`): schema, tokens, theme,
  properties, normalize, motion, layout, recipes, diagnostics.
- CSS backends consume normalized style facts; the hard-coded `:root` token
  block, first-word inheritance merging, and `[class*="fr-"]` resets are gone.
- New language surface: namespaced `tokens` contracts, `theme ... uses ...`,
  `layout` shells, `motion`, `recipe`/`variant`, `token(kind.name)` references.
- Experimental atomic CSS backend behind `--css-backend atomic`.
- Parser, semantic validation, codegen, LSP, tree-sitter grammar, docs, and
  tests updated together; grammar/registry drift tests keep them aligned.

Remaining follow-ups: recipe TS call-style API, inline Svelte source maps,
broader migration of size keywords (`sidebar`, `chart`, ...) into the token
contract.

# TODO.md

## Current Goal

Prepare Frame for the `Frame -> Frame IR -> DOM runtime` overhaul.

This file tracks the implementation path. Unchecked implementation items should remain unchecked until code and tests exist.

## Phase 0 — Foundation Documentation

- [x] Rewrite `README.md` around Frame as an experimental UI language.
- [x] Rewrite `AGENTS.md` with the new architecture and contributor rules.
- [x] Rewrite `TODO.md` around the overhaul path.
- [x] Rewrite `MILESTONES.md` around staged IR/runtime work.
- [x] Add `TODO-CSS.md` for structured CSS coverage tracking.
- [x] Add `TODO-DOM.md` for HTML, DOM, events, forms, accessibility, and runtime coverage tracking.
- [x] Add `research/` documentation for architecture decisions.

## Phase 1 — Language Model Design

- [x] Define the split between style declarations and UI declarations.
- [ ] Define full `component`, `view`, `state`, `props`, and `slots` syntax.
- [x] Define element syntax such as `button Send` and `button Send:PrimaryButton`.
- [ ] Define full automatic style lookup rules.
- [x] Define explicit style binding rules.
- [x] Define initial style reactivity syntax.
- [x] Define data reference syntax using `$name`.
- [x] Define handler reference syntax using `@name`.
- [x] Define binding syntax such as `value bind $draft`.
- [ ] Define condition syntax such as `show when $loggedIn`.
- [ ] Define loop syntax for list rendering.
- [ ] Define component composition syntax.
- [ ] Define escape hatches and mark unsafe forms explicitly.

## Phase 2 — Parser Upgrade Plan

- [ ] Replace or extend the current line-oriented parser where needed.
- [ ] Preserve existing styling syntax.
- [x] Parse UI declarations without breaking current CSS declarations.
- [x] Parse style binding names after `:`.
- [x] Parse `$state` references.
- [ ] Parse `$prop` references.
- [x] Parse `@handler` references.
- [x] Parse event filters like `keydown.enter` and `keydown.ctrl.enter`.
- [x] Parse initial conditions and reactive style rules.
- [x] Preserve source spans for initial UI syntax.
- [ ] Preserve comments for formatting.
- [x] Add parser tests for initial UI constructs.

## Phase 3 — Semantic Model

- [x] Resolve component names.
- [ ] Resolve props and state symbols.
- [x] Resolve initial state symbols.
- [x] Resolve `$value` references against state.
- [x] Collect `@handler` references.
- [ ] Validate automatic style lookup.
- [x] Validate explicit style references in-file with soft missing-style diagnostics.
- [x] Validate initial UI element names.
- [ ] Validate known attributes per element where practical.
- [x] Validate initial event names and event modifiers.
- [ ] Validate accessibility requirements for common controls.
- [ ] Validate unsafe raw HTML usage.
- [ ] Validate URL-bearing attributes.
- [ ] Generate teachable diagnostics with suggestions.

## Phase 4 — Frame IR

- [ ] Add renderer-neutral IR types.
- [ ] Add IR nodes for elements, text, components, fragments, slots, conditions, and loops.
- [ ] Add IR structures for attributes, properties, events, bindings, style bindings, and reactive rules.
- [ ] Add source mapping from IR nodes back to Frame source.
- [ ] Add capability flags for renderer support.
- [ ] Add JSON serialization for runtime consumption.
- [ ] Add IR snapshot tests.
- [ ] Document IR versioning.

## Phase 5 — TypeScript Contracts

- [ ] Generate component prop types.
- [ ] Generate component state types.
- [ ] Generate handler interfaces.
- [ ] Generate event context types.
- [ ] Generate DOM event-specific handler signatures.
- [ ] Generate skeleton implementation files.
- [ ] Avoid overwriting user implementations without explicit confirmation.
- [ ] Add tests for generated contracts.

## Phase 6 — DOM Runtime

- [ ] Create a DOM runtime package.
- [ ] Mount Frame IR into a DOM container.
- [ ] Create DOM elements from IR nodes.
- [ ] Create text nodes from escaped values.
- [ ] Apply attributes and properties safely.
- [ ] Attach event listeners from handler references.
- [ ] Support event filters and modifiers.
- [ ] Support state updates and patch scheduling.
- [ ] Support style class changes.
- [ ] Support conditional rendering.
- [ ] Support keyed list rendering.
- [ ] Support form bindings.
- [ ] Support cleanup on unmount.
- [ ] Add runtime tests.

## Phase 7 — CSS Integration

- [ ] Keep existing CSS class generation working.
- [ ] Connect UI node style bindings to generated classes.
- [ ] Support automatic style inheritance by node name.
- [ ] Support explicit style override with `Name:StyleName`.
- [ ] Support reactive style patches.
- [ ] Track remaining CSS coverage in `TODO-CSS.md`.
- [ ] Add CSS output tests for new style binding behavior.

## Phase 8 — CLI

- [ ] Add `frame build` for full IR/runtime output.
- [ ] Add `frame check` support for UI declarations.
- [ ] Add `frame emit-ir` for debugging.
- [ ] Add `frame emit-contracts` for TypeScript contracts.
- [ ] Add `frame init web`.
- [ ] Add `frame init tauri` later.
- [ ] Keep existing styling commands compatible during migration.

## Phase 9 — LSP

- [x] Add completions for UI declarations.
- [x] Add completions for initial UI elements.
- [ ] Add completions for attributes by element.
- [x] Add completions for initial events and modifiers.
- [ ] Add completions for `$state`, `$props`, and `@handlers`.
- [x] Add hover docs for initial UI concepts.
- [ ] Add diagnostics for unresolved style bindings.
- [ ] Add diagnostics for unresolved handlers.
- [ ] Add diagnostics for unsafe DOM sinks.
- [ ] Add code actions to create missing handler skeletons.
- [ ] Add code actions to create missing style declarations.
- [ ] Add go-to-definition for styles, state, props, and handlers.

## Phase 10 — Web/Tauri Target

- [ ] Build a minimal browser app with the DOM runtime.
- [ ] Build a minimal Tauri/WebView app using the same runtime.
- [ ] Document packaging expectations.
- [ ] Keep desktop-specific APIs outside the core Frame language.
- [ ] Add a small example app under `implementations/` when runtime exists.

## Phase 11 — Compatibility and Migration

- [ ] Decide how current Svelte integration survives the transition.
- [ ] Mark Svelte integration as legacy, bridge, or compatibility layer.
- [ ] Provide migration docs from generated CSS/TS usage to UI/IR/runtime usage.
- [ ] Keep existing examples compiling until intentionally replaced.

## Phase 12 — Quality Bar

- [ ] `cargo fmt`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Runtime package tests when package exists.
- [ ] Documentation examples compile or are clearly marked conceptual.

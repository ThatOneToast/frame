# TODO.md

## Current Goal

Design Frame's next language layer around semantic UI intent, accessibility, and renderer-independent primitives instead of exposing HTML and DOM concepts as the default authoring model.

Runtime work is paused. Do not implement SSR, hydration, routing, portals, suspense, async components, or further runtime rendering features during this milestone.

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
- [x] Define full `component`, `view`, `state`, `props`, and `slots` syntax.
- [x] Define element syntax such as `button Send` and `button Send:PrimaryButton`.
- [x] Define full automatic style lookup rules.
- [x] Define explicit style binding rules.
- [x] Define initial style reactivity syntax.
- [x] Define data reference syntax using `$name`.
- [x] Define handler reference syntax using `@name`.
- [x] Define binding syntax such as `value bind $draft`.
- [x] Define condition syntax such as `show when $loggedIn`.
- [x] Define loop syntax for list rendering.
- [x] Define initial zero-arg and named-argument component invocation syntax.
- [x] Define escape hatches and mark unsafe forms explicitly.

## Phase 1B — Language Redesign Milestone

- [x] Research lessons from HTML, JSX, React, Vue, Svelte, SwiftUI, Jetpack Compose, Flutter, Slint, and Figma concepts.
- [x] Document HTML leakage in the current Frame language.
- [x] Categorize leaked browser concepts as must remain, can be abstracted, or should be redesigned.
- [x] Create `docs/language-redesign.md`.
- [x] Create `docs/ui-primitives.md`.
- [x] Create `docs/layout-system.md`.
- [x] Create `docs/forms.md`.
- [x] Create `docs/accessibility-model.md`.
- [x] Decide the first implemented semantic primitive set.
- [x] Decide compatibility rules for existing HTML-like element syntax: no author-facing compatibility in `view`; browser words are internal lowering targets only.
- [x] Define parser changes for semantic primitives.
- [x] Define semantic model changes for primitive intent.
- [x] Define IR changes that preserve primitive kind separately from DOM mapping.
- [x] Define codegen and TypeScript contract changes for intent-based events such as `on press`.
- [x] Define LSP hover/completion wording for semantic primitives.
- [x] Add migration diagnostics and initial code actions for common HTML-like syntax.
- [x] Update examples to prefer semantic primitives after implementation exists.
- [x] Add `docs/semantic-lowering.md`.
- [x] Create canonical language registry in `crates/frame_core/src/language.rs` as the single source of truth for all language concepts.
- [x] Update LSP, Zed extension, parser, and semantic model to consume the canonical registry.
- [ ] Update docs and samples to reflect the registry as the source of truth and reinforce UI-native syntax direction.

## Phase 2 — Parser Upgrade Plan

- [x] Replace or extend the current line-oriented parser where needed.
- [x] Preserve existing styling syntax.
- [x] Parse UI declarations without breaking current CSS declarations.
- [x] Parse style binding names after `:`.
- [x] Parse `$state` references.
- [x] Parse `$prop` references.
- [x] Parse `@handler` references.
- [x] Parse event filters like `keydown.enter` and `keydown.ctrl.enter`.
- [x] Parse initial conditions and reactive style rules.
- [x] Preserve source spans for initial UI syntax.
- [ ] Preserve comments for formatting.
- [x] Add parser tests for all supported UI constructs.

## Phase 3 — Semantic Model

- [x] Resolve component names.
- [x] Resolve props and state symbols.
- [x] Resolve initial state symbols.
- [x] Resolve `$value` references against state and props.
- [x] Collect `@handler` references.
- [x] Validate automatic style lookup.
- [x] Validate explicit style references in-file with soft missing-style diagnostics.
- [x] Validate initial UI element names.
- [x] Validate known attributes per element where practical. (Implemented as primitive-specific property validation)
- [x] Validate initial event names and event modifiers.
- [x] Validate accessibility requirements for common controls.
- [x] Validate unsafe raw HTML usage.
- [x] Validate URL-bearing attributes.
- [x] Generate teachable diagnostics with suggestions.

## Phase 4 — Frame IR

- [x] Add initial renderer-neutral IR types.
- [x] Add IR nodes for elements, text, component invocations, conditions, and style rules.
- [x] Add IR nodes for fragments, slots, full conditions, and loops.
- [x] Add IR structures for attributes, properties, events, bindings, style bindings, and reactive rules.
- [x] Add source mapping from initial IR nodes back to Frame source.
- [x] Add capability flags for renderer support.
- [x] Add JSON serialization for runtime consumption.
- [x] Add stable IR serialization tests.
- [x] Document IR versioning.
- [x] Add golden fixture tests for IR structure across common patterns.

## Phase 5 — TypeScript Contracts

- [x] Generate component prop types.
- [x] Generate component state types.
- [x] Generate handler interfaces.
- [x] Generate event context types.
- [ ] Generate DOM event-specific handler signatures. (Deferred to Phase 6 — DOM Runtime)
- [ ] Generate skeleton implementation files. (Deferred to Phase 6 — DOM Runtime)
- [x] Avoid overwriting user implementations without explicit confirmation.
- [x] Add tests for generated contracts.

## Phase 6 — DOM Runtime

- [x] Create a DOM runtime package.
- [x] Mount Frame IR into a DOM container.
- [x] Create DOM elements from IR nodes.
- [x] Create text nodes from escaped values.
- [x] Apply attributes and properties safely.
- [x] Attach event listeners from handler references.
- [x] Support event filters and modifiers.
- [x] Support state updates with simple re-rendering.
- [x] Support style class changes.
- [x] Support conditional rendering.
- [x] Support keyed list rendering.
- [x] Support basic `value` and `checked` bindings.
- [x] Support cleanup on unmount.
- [x] Support scheduled batched updates.
- [x] Support hardened list reconciliation and cleanup.
- [x] Support runtime diagnostics with component/source context.
- [x] Add runtime tests.
- [x] Add accessibility defaults for semantic primitives.
- [x] Add mount-time handler validation with debug warnings.
- [x] Add mount-time prop type validation.
- [x] Add `readonly`, `disabled`, and `placeholder` property support.
- [x] Add `label` → `aria-label` mapping.
- [x] Add debug output for queued and flushed patches with component context.
- [x] Expand runtime tests for accessibility, events, input behavior, lifecycle, and diagnostics.
- [x] Generate skeleton implementation files (non-destructive handler skeletons with TODO comments).
- [x] Generate DOM event-specific handler signatures (FramePressEvent, FrameInputEvent, FrameKeyboardEvent, FrameFormEvent).
- [x] `frame build` emits contracts and skeletons into generated directory.

## Phase 7 — CSS Integration

- [x] Keep existing CSS class generation working.
- [x] Connect UI node style bindings to generated classes.
- [x] Support automatic style inheritance by node name.
- [x] Support explicit style override with `Name:StyleName`.
- [x] Support reactive style patches.
- [x] Track remaining CSS coverage in `TODO-CSS.md`.
- [x] Add CSS output tests for new style binding behavior.

## Phase 8 — CLI

- [x] Add `frame build` for CSS output from config.
- [x] Add `frame check` support for UI declarations.
- [x] Add `frame emit-ir` for debugging.
- [x] Add `frame emit-contracts` for TypeScript contracts.
- [x] Add `frame init web`.
- [x] Add `frame init svelte`.
- [x] Add `frame new` with web and svelte templates.
- [x] Add `frame doctor` for environment checks.
- [x] Add `frame build --watch` for automatic rebuilds during development.
- [x] Add project theme file (`app-theme.frame`) auto-resolution in `frame build` and `frame check`.
- [ ] Add `frame init tauri`. (Deferred to Phase 12 — Tauri/WebView Target)
- [x] Keep existing styling commands compatible during migration.

## Phase 9 — LSP

- [x] Add completions for UI declarations.
- [x] Add completions for initial UI elements.
- [ ] Add completions for attributes by element.
- [x] Add completions for initial events and modifiers.
- [x] Add completions for `$state`, `$props`, and `@handlers`.
- [x] Add completions for same-file component invocations.
- [x] Add completions for imported component invocations through `#include`.
- [x] Add hover docs for initial UI concepts.
- [x] Add hover docs for imported declarations and components.
- [x] Add diagnostics for unresolved style bindings.
- [x] Add diagnostics for unresolved handlers.
- [x] Add diagnostics for unsafe DOM sinks.
- [x] Add diagnostics for unresolved imported components.
- [x] Add diagnostics for duplicate symbols across includes.
- [x] Add diagnostics for imported symbols shadowing local symbols.
- [x] Add code actions to create missing handler skeletons.
- [x] Add code actions to create missing style declarations.
- [x] Add multi-file code actions to create missing state and prop declarations.
- [x] Add migration code actions from common browser words to Frame semantic primitives.
- [x] Add go-to-definition for styles, state, props, and handlers.
- [x] Add go-to-definition for imported styles, components, and grids.
- [x] Add Find All References for styles, handlers, state, props, and declarations across includes.
- [x] Add workspace edits that create missing symbols in included files.
- [x] Add project theme file (`app-theme.frame`) completions, hover, definitions, and references.
- [x] Add code actions that can create missing styles in `app-theme.frame`.

## Phase 10 — Web App Target

- [x] Build a minimal browser app with the DOM runtime.
- [x] Document packaging expectations.
- [x] Keep desktop-specific APIs outside the core Frame language.
- [x] Add a small example app under `implementations/` when runtime exists.
- [x] Add project theme file (`app-theme.frame`) to web template.
- [x] Add `frame:watch` script to web template and wire into `npm run dev`.

## Phase 11 — Tauri/WebView Target

- [ ] Build a minimal Tauri/WebView app using the DOM runtime.
- [ ] Document packaging expectations.
- [ ] Add a small example app under `implementations/` when runtime exists.

## Phase 12 — Compatibility and Migration

- [ ] Decide how current Svelte integration survives the transition.
- [ ] Mark Svelte integration as legacy, bridge, or compatibility layer.
- [ ] Provide migration docs from generated CSS/TS usage to UI/IR/runtime usage.
- [ ] Keep existing examples compiling until intentionally replaced.

## Phase 13 — Quality Bar

- [x] `cargo fmt`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Runtime package tests when package exists.
- [x] Documentation examples compile or are clearly marked conceptual.

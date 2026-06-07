# AGENTS.md

## Mission

Frame is an experimental UI language moving from a structured styling DSL into a renderer-independent interface language.

The long-term architecture is:

```txt
.frame source
  -> parser
  -> semantic model
  -> Frame IR
  -> renderer targets
       - DOM runtime
       - static HTML
       - Tauri/WebView
       - future native desktop renderers
```

The immediate goal is documentation and architecture preparation. Do not start implementation work until the planned compiler, runtime, DOM, and CSS coverage documents are clear.

## Current Repository Reality

The current codebase has a compiler foundation and initial runtime model:

```txt
crates/
  frame_core/      AST, diagnostics, formatting, semantic model, knowledge tables
    semantic/      split validators: constants, helpers, ui, declarations, statements
  frame_parser/    line-oriented parser and parse errors
    lib.rs         parse(), ParseError, Parser struct, tests
    document.rs    component, state, props, slot, view parsing
    declarations.rs declaration and nested block parsing
    ui.rs          UI element, event, property, loop parsing
    helpers.rs     utility functions and keyword tables
  frame_codegen/   CSS, TypeScript class exports, IR JSON, contracts
    css/           split emitters: mod, emit, properties, helpers, tests
  frame_runtime/   renderer-neutral runtime model (state, events, patches)
  frame_cli/       check, compile, format, watch, init, build, doctor, new
  frame_lsp/       LSP server

packages/
  frame-svelte/    existing Svelte/Vite integration

editors/
  zed/             Zed extension with tree-sitter grammar and LSP

implementations/
  chat-app/        rough application experiments
```

This existing work should be preserved while the new UI/IR/runtime system is introduced incrementally.

## Non-Negotiable Design Goals

1. Frame owns the UI model.
   - Do not make Svelte, React, or HTMX the internal model.
   - Frameworks can be compatibility targets later, not the center of the language.

2. Frame targets the DOM directly first.
   - The first runtime target should be `Frame -> Frame IR -> DOM runtime`.
   - Tauri should reuse the DOM runtime through WebView.
   - Native rendering comes after the IR stabilizes.

3. Frame should be capable of full DOM and HTML coverage.
   - Every HTML element should have a documented representation.
   - Every relevant DOM capability should be cataloged.
   - Unsupported features should be explicitly tracked in `TODO-DOM.md`.

4. Frame styling should remain structured and teachable.
   - Do not throw away the existing CSS work.
   - Continue expanding structured CSS coverage.
   - Keep the advanced CSS escape hatch, but promote repeated patterns into native Frame syntax.

5. Frame scripting is external.
   - Do not put JavaScript or TypeScript bodies inside Frame declarations.
   - Frame references handlers with `@handlerName`.
   - Frame references data with `$valueName`.
   - The compiler should generate contracts and skeletons.

6. Security must be designed in from the beginning.
   - Text interpolation escapes by default.
   - Raw HTML is explicit and unsafe.
   - URL attributes, script-like sinks, and DOM injection must be modeled.
   - Event bindings should be typed references, not inline code.

7. Tooling is part of the product.
   - LSP diagnostics, hovers, completions, code actions, and formatting are core features.
   - Every new syntax feature needs editor behavior.
   - Hover docs should teach the user what Frame means.

## Preferred Syntax Direction

Named UI node with automatic style lookup:

```frame
button Send {
  text "Send"
  on click @sendMessage
}
```

Explicit style binding:

```frame
button Send:PrimaryButton {
  text "Send"
}
```

State-driven style switching:

```frame
button Send:PrimaryButton {
  text "Send"
  disabled when $sending
  style when $sending = LoadingButton
}
```

Data reference:

```frame
text $username
```

Handler reference:

```frame
on keydown.enter @submitMessage
```

Do not introduce inline scripting syntax.

## Implementation Rules

- Prefer small modules with clear responsibility.
- Add tests for every parser, semantic, codegen, and runtime feature.
- Keep generated output deterministic.
- Keep compiler diagnostics source-mapped and useful for LSP.
- Keep old styling examples working until intentionally migrated.
- Avoid large rewrites without an intermediate compatibility layer.
- Update `TODO.md` and `MILESTONES.md` as work progresses.
- Update `TODO-CSS.md` and `TODO-DOM.md` when coverage changes.
- Keep docs accurate with implemented behavior.
- Do not mark checklist items complete unless the implementation and tests exist.

## Expected New Architecture

Planned crates/packages:

```txt
crates/
  frame_core/
    ast.rs
    diagnostics.rs
    semantic/
      mod.rs
      constants.rs
      helpers.rs
      ui.rs
      declarations.rs
      statements.rs
    ir.rs
    source_map.rs

  frame_parser/
    lib.rs
    document.rs
    declarations.rs
    ui.rs
    helpers.rs

  frame_codegen/
    css/
      mod.rs
      emit.rs
      properties.rs
      helpers.rs
      tests.rs
    ts_contracts.rs
    ir_json.rs

  frame_runtime/
    node.rs
    state.rs
    event.rs
    patch.rs
    renderer.rs

  frame_cli/
    main.rs

  frame_lsp/
    main.rs

packages/
  runtime-dom/
    src/app.ts
    src/dom.ts
    src/events.ts
    src/state.ts
    src/patch.ts
```

Do not create these until implementation begins, but use this structure as the planning target.

## Testing Expectations

Use these commands before claiming implementation work is complete:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

When JavaScript packages are involved, also run the relevant package tests/builds.

## Documentation Expectations

Documentation should be direct, useful, and low on hype.

Every major feature should explain:

- what the syntax means
- what the compiler stores in the AST/IR
- what the runtime does
- what the DOM output should do
- what diagnostics should exist
- what remains unsupported

## Current Work Mode

The pre-DOM compiler foundation is complete. This includes the parser, semantic model, Frame IR, TypeScript contracts, CLI, and LSP support for UI syntax. The DOM runtime crate exists with a renderer-neutral model, but DOM rendering implementation is paused while the semantic language redesign milestone is resolved.

The CLI was recently modularized and now includes `build`, `doctor`, `new`, `init web`, and `init svelte`. The largest source files (`semantic.rs`, `css.rs`, `parser/lib.rs`) have been split into submodules for maintainability.

All changes should be tested with:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

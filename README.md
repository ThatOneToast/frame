# Frame

Frame is an experimental UI language for building interfaces through structured, teachable syntax.

The project began as a structured CSS language for Svelte-focused UI development. That work proved the core idea: interface styling can be easier to understand when the language describes intent instead of forcing every developer to memorize CSS property syntax first.

Frame is now moving toward a larger goal:

```txt
.frame source
  -> Frame parser
  -> Frame semantic model
  -> Frame IR
  -> renderer targets
       - DOM runtime
       - static HTML
       - Tauri/WebView apps
       - future native desktop renderers
```

Frame is not stable. The language, compiler internals, runtime model, and documentation are expected to change while the project explores its foundation.

## Why Frame exists

Modern UI development is powerful, but often split across HTML, CSS, JavaScript, framework-specific component syntax, build tooling, state libraries, and editor plugins. Frame is an attempt to make the UI layer feel like one language without hiding the underlying platform.

Frame should be:

- concise enough to read quickly
- structured enough for strong editor tooling
- explicit about events, state, styling, and DOM behavior
- capable of representing everything the DOM and HTML can do
- renderer-independent at the compiler layer
- safe by default, with unsafe escape hatches clearly marked

Frame should not become:

- raw HTML with different brackets
- raw CSS with different punctuation
- React or Svelte syntax rewritten with new keywords
- a language that hides security-sensitive behavior
- a UI framework that can only target one renderer

## Current status

The current repository contains a working structured styling compiler and editor tooling foundation.

Existing pieces include:

```txt
crates/
  frame_core/      AST, diagnostics, semantic model, formatting, knowledge tables
  frame_parser/    parser and parse errors
  frame_codegen/   CSS and TypeScript class export generation
  frame_cli/       check, compile, format, watch, init
  frame_lsp/       LSP server

packages/
  frame-svelte/    current Svelte/Vite integration

editors/
  zed/             Zed extension scaffold and syntax support

implementations/
  chat-app/        rough reference implementation experiments
```

The existing compiler still mainly outputs CSS and TypeScript class exports. The overhaul tracked in this repository changes the target architecture from `Frame -> CSS/Svelte helper output` toward `Frame -> Frame IR -> DOM runtime`.

Initial UI declarations now parse, validate, highlight, and have LSP completions/hovers. Frame IR lowering, DOM runtime rendering, and generated handler contracts are not implemented yet.

## Direction

Frame should own the UI model.

A Frame file should be able to describe structure, style, state references, event bindings, accessibility, attributes, DOM behavior, and renderer intent without writing inline JavaScript inside the UI declaration.

Example direction:

```frame
component ChatInput {
  state {
    draft text = ""
    sending bool = false
  }

  view {
    input MessageBox {
      value bind $draft
      placeholder "Message #general"
      on keydown.enter @sendMessage
    }

    button Send:PrimaryButton {
      text "Send"
      disabled when $sending
      on click @sendMessage
      style when $sending = LoadingButton
    }
  }
}
```

Important syntax ideas:

```frame
button Send {
  on click @sendMessage
  on keydown.enter @submitMessage
}
```

`button Send` creates a named UI node. By default Frame should look for a matching style named `Send` and inherit it automatically.

```frame
button Send:PrimaryButton {
  text "Send"
}
```

The name before `:` is the semantic UI node identity. The name after `:` is the explicit style binding.

```frame
button Send:PrimaryButton {
  style when $sending = LoadingButton
}
```

Style reactivity should be first-class. The compiler should understand the state dependency and the runtime should apply the correct style patch.

## Frame IR

The Frame IR is the central contract between the compiler and renderers.

The IR should represent:

- components
- nodes
- text nodes
- attributes
- properties
- events
- event filters and modifiers
- state reads and writes
- bindings
- conditions
- loops
- slots and children
- styles and style variants
- accessibility metadata
- asset references
- renderer capabilities
- diagnostics/source mappings

Renderers should not parse `.frame` files. Renderers consume the IR.

## DOM runtime

The first serious runtime target should be the DOM.

```txt
Frame source
  -> Frame IR
  -> generated runtime definition
  -> @frame/runtime-dom
  -> browser DOM
```

The DOM runtime should be small and explicit. It should create nodes, bind attributes, attach event listeners, apply style classes, update text, manage reactive state patches, and preserve security defaults.

This runtime should also be usable inside Tauri because Tauri already hosts a WebView.

```txt
Frame -> Frame IR -> DOM runtime -> browser
Frame -> Frame IR -> DOM runtime -> Tauri WebView
```

Native renderers can come later once the IR is stable.

## Existing styling syntax

The current styling syntax remains valuable and should not be thrown away.

Example:

```frame
grid AppShell {
  columns sidebar content inspector
  rows main
  gap medium
  height screen
}

area Sidebar {
  in AppShell
  place sidebar
  surface panel
  padding medium
}

card QuickLinkCard {
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

The styling language should continue growing toward full CSS capability through structured vocabulary plus a clear advanced escape hatch.

## Security expectations

Frame must be safe by default.

Default rules:

- `$value` text insertion escapes by default.
- Raw HTML requires an explicit unsafe form.
- Event handlers are references, not inline JavaScript.
- URL-bearing attributes must be validated or classified.
- DOM sinks that can execute code must be modeled carefully.
- Generated code should keep source mappings for diagnostics.
- Compiler and LSP diagnostics should explain unsafe behavior clearly.

## Development docs

Start here:

- `AGENTS.md` — contributor and automation guidance
- `TODO.md` — overhaul checklist
- `MILESTONES.md` — staged implementation plan
- `TODO-CSS.md` — structured CSS coverage tracker
- `TODO-DOM.md` — HTML, DOM, events, accessibility, and runtime coverage tracker
- `research/` — architecture notes for the Frame IR and DOM runtime direction

## Commands

Current commands are still styling/compiler oriented:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cargo run -p frame_cli -- check examples/svelte/src/lib/frame/app.frame
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
cargo run -p frame_cli -- format examples/svelte/src/lib/frame/app.frame
```

These commands should remain useful while the new IR/runtime architecture is introduced.

## License

MIT

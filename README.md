# Frame

Frame is an experimental UI-native language for building interfaces through structured, teachable syntax.

The canonical language registry in `crates/frame_core/src/language.rs` owns all language facts, and the `SemanticCursor` in `crates/frame_lsp/src/ide/cursor.rs` owns IDE context. Author-facing primitives include `panel`, `stack`, `dock`, `action`, `field`, `feed`, `overlay`, and others. HTML and CSS terms remain available as advanced or explicit escape-hatch syntax, not the primary authoring path.

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
- explicit about events, state, styling, and renderer behavior
- centered on semantic UI intent instead of browser tags
- capable of representing everything the DOM and HTML can do
- renderer-independent at the compiler layer
- safe by default, with unsafe escape hatches clearly marked

Frame should not become:

- raw HTML with different brackets
- raw CSS with different punctuation
- React or Svelte syntax rewritten with new keywords
- a direct mirror of browser implementation details
- a language that hides security-sensitive behavior
- a UI framework that can only target one renderer

## Canonical Language Registry

`crates/frame_core/src/language.rs` is the single source of truth for every Frame language concept. The registry defines declarations, UI primitives, properties, values, events, modifiers, state keywords, binding keywords, and effects. All of the following consume the registry:

- **Parser** — validates keywords and identifiers against the registry.
- **Semantic model** — classifies language items by kind and layer.
- **LSP** — provides completions, hover docs, semantic tokens, and diagnostics from the registry.
- **Zed extension** — tree-sitter grammar and highlighting are aligned with registry categories.

To add a new primitive, property, or value, update the registry first. See `docs/contributing.md`.

## Current status

The repository contains three distinct layers:

1. **Styling compiler: usable** — generates CSS and TypeScript class exports from structured styling declarations.
2. **UI compiler foundation: implemented** — parses, validates, and lowers UI components to Frame IR; generates TypeScript contracts.
3. **DOM runtime: Phase 4 implemented with maturity** — `@frame/runtime-dom` mounts Frame IR into browser DOM nodes, schedules batched dependency-aware patches, reconciles lists, cleans up listeners/subscriptions, covers practical HTML/forms/attribute behavior, applies accessibility defaults for semantic primitives, validates handlers and props at mount time, and provides debug output for patches.

The LSP now understands project-level Frame code through `#include`:
- imported styles, components, and declarations resolve for completions, hover, go-to-definition, and diagnostics
- `frame check` validates multi-file projects including cross-file component references
- missing handler, state, prop, and style code actions generate skeletons in the correct included files
- Find All References works across includes
- primitive-specific property validation catches intent mismatches with teacher-like diagnostics

The active milestone is semantic UI syntax. SSR, hydration, routing, portals, suspense, transition runtime, animation runtime, and async component work are intentionally paused while Frame's author-facing language moves away from HTML-like UI syntax.

Existing pieces:

```txt
crates/
  frame_core/      AST, diagnostics, semantic model, IR, formatting, knowledge tables
  frame_parser/    line-oriented parser and parse errors
  frame_codegen/   CSS, TypeScript class exports, IR JSON, contracts
  frame_runtime/   renderer-neutral runtime model
  frame_cli/       check, compile, format, watch, init, build, doctor, new
  frame_lsp/       LSP server

packages/
  frame-svelte/    current Svelte/Vite integration

editors/
  zed/             Zed extension with tree-sitter grammar and LSP

implementations/
  chat-app/        rough reference implementation experiments
```

The compiler outputs CSS and TypeScript class exports for styling declarations. For UI declarations, it produces Frame IR and TypeScript contracts. `frame build` also emits non-destructive handler skeletons and event-specific type aliases so developers can write Frame UI first, then fill in generated TypeScript.

The DOM runtime can consume Frame IR for mounting, disposal, elements, text, nested components, props, state, events, bindings, conditions, style classes, keyed or positional list reconciliation, common HTML elements, global attributes, safe URL attributes, form controls, scheduled updates, runtime diagnostics, accessibility defaults, handler/prop validation, and cleanup accounting.

## Direction

Frame should own the UI model.

A Frame file should be able to describe structure, style, state references, event bindings, accessibility, attributes, DOM behavior, and renderer intent without writing inline JavaScript inside the UI declaration.

### UI-native syntax first

Frame source is intent-first. Inside `view` blocks, use primitives that describe user intent:

```frame
component ChatInput {
  state {
    draft text = ""
    sending bool = false
  }

  view {
    composer ChatBox:ComposerShell {
      label "Message"
      draft bind $draft
      send @sendMessage

      field MessageField {
        input MessageBox {
          value bind $draft
          placeholder "Message #general"
          on keydown.enter @sendMessage
        }
      }
    }

    action Send:PrimaryButton {
      text "Send"
      disabled when $sending
      on press @sendMessage
      style LoadingButton when $sending
    }
  }
}
```

Author-facing UI primitives include:
- **Actions and navigation**: `action`, `link`, `menu`, `toolbar`, `tabs`
- **Content**: `panel`, `card`, `dialog`, `popover`, `screen`, `section`, `text`, `title`, `label`, `badge`, `icon`
- **Input**: `field`, `input`, `editor`, `toggle`, `choice`, `select`, `composer`
- **Data**: `list`, `feed`, `data`, `item`, `empty`
- **Media**: `image`, `avatar`, `media`
- **Layout**: `stack`, `row`, `dock`, `grid`, `split`, `overlay`, `scroll`

DOM elements such as `button`, `a`, `textarea`, `div`, `span`, `form`, `table`, `tr`, and `td` are internal lowering targets, not author-facing UI syntax. The compiler stores `semantic_kind` separately from `render_kind` so the LSP, diagnostics, and non-DOM renderers can reason about author intent.

### Advanced and escape hatches

Low-level CSS terms remain valid in the styling layer but should not be the primary authoring path in UI `view` blocks:

- Prefer `stack`, `flow`, `grid`, `dock` over `display flex`, `display grid`
- Prefer `overlay`, `scroll` over `position absolute`, `position fixed`
- Use `advanced { css "raw-property" value }` for true escape hatches

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

The first DOM runtime lives in `packages/runtime-dom`. It is small and explicit: it creates nodes, binds attributes, attaches event listeners once, applies style classes, and patches affected text, properties, attributes, conditions, styles, and list blocks when state changes.

The runtime intentionally does not implement SSR, hydration, routing, transitions, portals, suspense, async components, or advanced reconciliation.

Phase 4 hardening adds a DOM update scheduler, deterministic patch flushing, duplicate patch coalescing, list move/reuse/create/remove counters, nested list and component-in-list reconciliation, explicit listener/subscription cleanup, runtime error diagnostics with component/source context, and immediate `flush()` support for tests.

Further runtime feature work is paused during the language redesign milestone.

This runtime should also be usable inside Tauri because Tauri already hosts a WebView.

```txt
Frame -> Frame IR -> DOM runtime -> browser
Frame -> Frame IR -> DOM runtime -> Tauri WebView
```

Native renderers can come later once the IR is stable.

## Styling system

Frame is not CSS shorthand. Frame is a semantic UI language that compiles into
efficient platform output. Styling flows through a normalized style layer
(`crates/frame_core/src/style/`): statements lower to typed style facts,
token/theme contracts resolve, and a CSS backend emits static output
(`frame build --css-backend semantic|atomic`).

The high-level vocabulary:

```frame
tokens default {
  color text #f5f5f5
  surface panel #171722
  space md 1rem
  breakpoint tablet 48rem
}

theme dark uses default {
  surface app #101014
}

layout DashboardShell {
  shell {
    sidebar left fixed 18rem
    main fluid
  }
  gap large
  below tablet { shell stacked }
}

motion Pressable {
  enter fade up soft
  hover lift sm
  duration normal
  easing smooth
}

recipe Button {
  base {
    align center
    radius medium
    motion Pressable
  }
  variant tone {
    primary { background token(color.accent) }
    ghost { background transparent }
  }
}
```

Tokens are typed contracts with did-you-mean diagnostics; themes scope token
overrides behind `[data-frame-theme]`; motions expand at compile time (no
runtime styling); recipes compile to static variant classes plus typed
TypeScript metadata. `extends` inheritance merges by property path
(`border.width`, `layout.display`), not by first word.

Structured declarations remain for precise control:

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
- `docs/contributing.md` — how to add language concepts and follow the canonical registry
- `TODO.md` — overhaul checklist
- `MILESTONES.md` — staged implementation plan
- `docs/language-redesign.md` — current semantic language redesign report
- `docs/ui-primitives.md` — proposed intent-first UI primitive catalog
- `docs/layout-system.md` — proposed layout primitives
- `docs/forms.md` — proposed form control model
- `docs/accessibility-model.md` — accessibility-first compiler and renderer model
- `docs/semantic-lowering.md` — semantic primitive to renderer-target lowering strategy
- `TODO-CSS.md` — structured CSS coverage tracker
- `TODO-DOM.md` — HTML, DOM, events, accessibility, and runtime coverage tracker
- `research/` — architecture notes for the Frame IR and DOM runtime direction
- `editors/zed/samples/semantic-*.frame` — semantic UI examples

## Commands

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Project scaffolding
frame new my-app --template web
frame new my-app --template svelte

# Existing project init
frame init web
frame init svelte

# Check and compile
frame check src/app.frame
frame build
frame compile src/app.frame --out src/generated
frame watch src/app.frame --out src/generated
frame format src/app.frame

# Debug output
frame emit-ir src/app.frame
frame emit-contracts src/app.frame

# Environment check
frame doctor
```

Standalone web projects created with `frame new my-app --template web` use:

```txt
src/
  app.frame
  handlers.ts
  generated/
    app.ir.json
    app.ir.ts
    frame.types.ts
    frame.handlers.ts
```

`npm run dev` and `npm run build` run `frame build` before Vite. Generated IR and contracts are overwritten only when content changes. `frame.handlers.ts` is a generated reference file; `frame build` preserves existing content and appends missing stubs, while user-owned implementations live in `src/handlers.ts`.

## License

MIT

# Frame Agent Guide

This folder is written for LLMs and coding agents that need to generate correct Frame code without guessing.

Frame is an experimental UI-native language for building interfaces through structured, teachable syntax. It compiles `.frame` source through a parser, semantic model, and IR to renderer targets (DOM runtime, static HTML, Tauri/WebView).

Frame is **not stable**. Language, internals, runtime, and docs are expected to change.

## Agent Decision Tree

Use Frame for:

- **UI components** with `component`, `view`, `state`, `props`, `slot`
- **Structured styling** with `grid`, `area`, `card`, `stack`, `row`, etc.
- **Design tokens** with `tokens` (custom colors, gradients)
- **Animation** with `keyframes` and `animation` blocks
- **Responsive design** with `below`, `above`, `between`, `container` blocks
- **Interaction states** with `hover`, `focus`, `active`, `disabled` blocks

Use Svelte `<style lang="frame">` blocks when the user wants:

- component-local styles
- quick examples
- no generated TypeScript requirement

## Core Concepts

### Two contexts: styling vs UI

**Styling context** (file root): declarations like `grid`, `area`, `card`, `stack`, `row`, `tokens`, `keyframes`.

```frame
grid Dashboard {
  columns sidebar content
  gap medium
}

card ProjectCard {
  surface panel
  padding medium
  radius medium
}
```

**UI context** (`component { view { ... } }`): UI primitives like `action`, `panel`, `stack`, `text`, `input`, `field`, `toggle`, `select`, `list`, `feed`, `image`, `media`, `dialog`, `popover`.

```frame
component ChatApp {
  state {
    draft text = ""
  }
  view {
    panel Main {
      text "Hello"
      action Send {
        text "Send"
        on press @sendMessage
      }
    }
  }
}
```

### Key syntax rules

1. **No inline JavaScript** — use `@handler` references and `$data` references
2. **Component args use `:`** — `Greeting(name: "World")`, NOT `Greeting(name = "World")`
3. **Loop keys must start with `$`** — `for item in $items key $selected`
4. **`show when` goes inside elements** — not standalone at view level
5. **`slot` is component-level** — not inside declarations
6. **Surface values are specific** — `panel`, `main`, `glass`, `flat`, `raised`, `overlay`, `inset`, `sunken`
7. **Prop types are specific** — `text`, `string`, `bool`, `number`, `list`

## Setup Commands

```bash
# New web app project
frame new my-app --template web

# Check a Frame file
frame check src/app.frame

# Format a Frame file
frame format src/app.frame

# Compile
frame build

# Watch mode
frame watch
```

## Required Mental Model

### Top-level = styling declarations

```frame
grid Dashboard { columns sidebar content }
area Sidebar { in Dashboard; place sidebar }
card Card { surface panel; padding medium }
tokens Brand { color primary #3B82F6 }
keyframes FadeIn { from { opacity 0 } to { opacity 1 } }
```

### Inside components = UI primitives

```frame
component ChatApp {
  props { title text }
  state { draft text = "" }
  view {
    text $title
    action Send { text "Send"; on press @sendMessage }
  }
  slot Default { text "Fallback" }
}
```

### Data and handler references

- `$name` — references state, props, or loop variables
- `@name` — references external handler functions
- `bind $name` — two-way binding (on `input`, `toggle`, `select`, `choice`)

## Core Agent Rules

1. **Registry is source of truth**: `crates/frame_core/src/language.rs` defines all primitives, properties, values, events, modifiers.
2. **Use Frame-native syntax**: `surface panel` not `background-color #111`.
3. **Use UI primitives in view blocks**: `action`, `panel`, `stack`, not `button`, `div`, `span`.
4. **No inline scripts**: `on press @handler`, not `onclick="handler()"`.
5. **Security by default**: Text escapes by default. Raw HTML is explicit and unsafe.
6. **Declarations are classes**: `card Card` generates `fr-Card` CSS class.
7. **Components are PascalCase**: `component ChatApp { ... }`.
8. **State types are explicit**: `text`, `bool`, `number`, `list`.
9. **Props have no defaults**: `props { title text }` (no `= value`).
10. **State has defaults**: `state { draft text = "" }`.

## Common Failure Modes

### Do not mix styling and UI context

```frame
// WRONG: card at file root is styling, not UI
card Dialog {
  text "Hello"  // text is not a valid styling property
}

// RIGHT: card inside component view
component App {
  view {
    card Dialog {
      text "Hello"
    }
  }
}
```

### Do not use browser words as UI primitives

```frame
// WRONG
button Save { on click @save }
div Container { text "Hello" }

// RIGHT
action Save { on press @save }
panel Container { text "Hello" }
```

### Do not put state blocks inside declarations

```frame
// WRONG
card BadCard {
  hover {
    grid NestedGrid  // grid is not valid inside hover
  }
}

// RIGHT
card GoodCard {
  hover {
    lift small
    glow soft
  }
}
```

### Do not use `=` for component args

```frame
// WRONG
Greeting(name = "World")

// RIGHT
Greeting(name: "World")
```

### Do not use standalone `show when`

```frame
// WRONG
show when $showPanel

// RIGHT (inside an element)
panel Main {
  show when $showPanel
  text "Content"
}
```

## Related Agent Docs

- [Language Cheat Sheet](language-cheatsheet.md)
- [Recipes](recipes.md)
- [Troubleshooting](troubleshooting.md)

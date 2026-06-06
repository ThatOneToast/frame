# Frame Language

Frame is a structured CSS language for Svelte projects. A `.frame` file describes components and layout regions with concepts like `surface panel`, `columns responsive cards`, `radius large`, and `hover { lift small }`.

Frame is not raw CSS with different punctuation. The goal is to expose CSS power through readable, guided syntax: first-class grids, responsive rules, container queries, gradients, transitions, animations, keyframes, tokens, states, and safe advanced CSS escape hatches.

## Declarations

Top-level declarations create generated classes:

```frame
grid AppShell {
  columns sidebar content inspector
  gap medium
}

card ProjectCard {
  surface gradient dusk
  padding large
  radius large
  shadow soft
}

keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px)
  }

  to {
    opacity 1
    transform translateY(0)
  }
}
```

Generated TypeScript exports stable class names:

```ts
export const ui = {
  AppShell: 'fr-AppShell',
  ProjectCard: 'fr-ProjectCard'
} as const;
```

## Statements

Statements are line-based:

```frame
surface panel
padding medium
width fill
```

Nested state blocks describe interaction intent:

```frame
button PrimaryButton {
  surface panel
  padding medium
  radius pill

  focus {
    ring accent
  }

  active {
    press
  }

  disabled {
    dim medium
  }
}
```

The compiler emits readable CSS and a `generated.ts` file for Svelte imports.

## Feature Queries

Use typed `supports` blocks when styles should only emit behind a browser feature query:

```frame
supports display grid {
  grid AppShell {
    columns sidebar content
  }
}

supports subgrid {
  grid NestedGrid {
    columns subgrid
  }
}
```

Supported predicates are `display grid`, `display flex`, `backdrop blur`, `color oklch`, `selector has`, `container queries`, and `subgrid`.

Generated CSS uses `@supports`, for example `supports display grid` emits `@supports (display: grid)`.

## Style Groups

Use style groups when generated CSS needs deterministic cascade layers:

```frame
style-order reset, base, components, utilities

style-group components {
  button PrimaryButton {
    surface panel
    radius medium
  }
}
```

Generated CSS uses `@layer`, but Frame keeps the source syntax focused on named style groups.

## Experimental UI Syntax

Frame now parses and validates an initial UI declaration slice. This is a compiler and tooling foundation only: there is no Frame IR lowering, DOM runtime, or generated handler contract output yet.

```frame
component ChatInput {
  state {
    draft text = ""
    sending bool = false
  }

  view {
    input MessageBox {
      value bind $draft
      placeholder "Message"
      on keydown.ctrl.enter @sendMessage
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

Supported state types are `text`, `bool`, and `number`. Defaults must match the declared type.

View blocks support the initial element names `button`, `input`, `text`, `card`, `panel`, `row`, `stack`, `grid`, `area`, `image`, `link`, and `form`. Element names such as `Send` record automatic style lookup intent. Explicit style binding uses `ElementName:StyleName`.

Text nodes support literal text and state references:

```frame
text "Send"
text $username
```

Data references use `$valueName` and are validated against component state. Handler references use `@handlerName`; Frame records them as external references and does not allow inline JavaScript or TypeScript bodies.

Events currently validate the event name and modifiers:

```frame
on click @sendMessage
on keydown.enter @submitMessage
on keydown.ctrl.enter @submitMessage
```

The compiler stores these declarations in the UI AST, can lower them into Frame IR, and can generate TypeScript contracts. Runtime rendering through the DOM runtime is still future work.

Component invocations are also supported inside `view` blocks:

```frame
component ChatApp {
  state {
    activeChannel text = "general"
    draft text = ""
  }

  view {
    ChannelSidebar()
    ChatPanel(channel: $activeChannel)
    MessageComposer(draft bind $draft)
  }
}
```

Invocations validate against components declared in the same file. Arguments currently support `name: $state`, `name: "literal"`, and `name bind $state`.

Frame can lower this initial UI syntax into renderer-neutral Frame IR and serialize it as JSON with:

```bash
frame emit-ir app.frame
```

Frame can also generate TypeScript contracts for state and external handlers:

```bash
frame emit-contracts app.frame
```

Generated contracts define `ComponentState`, `ComponentHandlers`, and a shared `FrameEventContext<TState>`. They do not generate runtime code or overwrite user implementation files.

# Frame Language

Frame is a design-intent CSS DSL. It compiles declarations such as `grid`, `area`, `card`, `row`, `stack`, `dock`, and `text` into normal CSS classes and stable TypeScript exports.

Top-level declarations:

```frame
#include tokens

tokens Brand {
  color brand #7c3aed
}

grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

card DemoCard {
  surface gradient dusk
  border accent
  shadow medium
  transition smooth

  hover {
    lift small
    glow accent
  }
}
```

Core docs:

- `docs/app-vocabulary.md`
- `docs/imports.md`
- `docs/tokens.md`
- `docs/colors.md`
- `docs/surfaces.md`
- `docs/borders.md`
- `docs/effects.md`
- `docs/animations.md`
- `docs/layout.md`
- `docs/sizing.md`
- `docs/grid.md`
- `docs/svelte.md`

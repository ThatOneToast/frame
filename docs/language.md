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
card PrimaryButton {
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
  card PrimaryButton {
    surface panel
    radius medium
  }
}
```

Generated CSS uses `@layer`, but Frame keeps the source syntax focused on named style groups.

## UI Syntax

Frame parses, validates, and lowers semantic UI declarations into renderer-neutral Frame IR. TypeScript contracts and IR JSON are generated from the same AST. The DOM runtime consumes that IR for browser rendering; SSR and hydration are not implemented.

```frame
component ChatInput {
  props {
    placeholder text
  }

  state {
    draft text = ""
    sending bool = false
  }

  view {
    field MessageField {
      label "Message"

      editor Message {
        value bind $draft
        on keydown.ctrl.enter @sendMessage
      }
    }

    action Send:PrimaryButton {
      disabled when $sending
      on press @sendMessage
      style LoadingButton when $sending
    }
  }
}
```

### Props

Props are declared in a `props` block with type annotations:

```frame
props {
  title text
  active bool
  count number
}
```

Props do not have defaults; they are provided by the parent component. Supported types are `text`, `string`, `bool`, `number`, and `list`.

### State

State is declared in a `state` block with type and default:

```frame
state {
  draft text = ""
  sending bool = false
  count number = 1
}
```

### View

View blocks support semantic primitives, text nodes, component invocations, loops, and slots.

Author-facing UI syntax uses intent-first primitives:

```frame
screen Chat
panel Messages
action Send
link Documentation
input Username
field EmailField
editor Draft
toggle Notifications
composer ChatBox
list Messages
data Projects
dialog Settings
```

Browser-centric words such as `button`, `div`, `span`, `a`, `textarea`, `form`, `table`, `tr`, and `td` are not valid author-facing UI primitives. They may still appear internally as renderer lowering targets.

Node names such as `Send` record semantic identity and automatic style lookup intent. Explicit style binding uses `NodeName:StyleName`.

Text nodes support literal text and data references:

```frame
text "Send"
text $username
```

Data references use `$valueName` and are validated against component props and state. Handler references use `@handlerName`; Frame records them as external references and does not allow inline JavaScript or TypeScript bodies.

Events validate the event name and modifiers:

```frame
on press @sendMessage
on keydown.enter @submitMessage
on keydown.ctrl.enter @submitMessage
```

Conditional rendering and conditional properties are supported:

```frame
panel Main {
  show when $loggedIn
  text "Welcome"
}

action Send {
  disabled when $sending
}
```

Conditional style switching:

```frame
action Send:PrimaryButton {
  style LoadingButton when $sending
}
```

Form bindings support `value`, `checked`, and `selected`:

```frame
input Draft {
  bind $draft
}

toggle Enabled {
  bind $enabled
}

select Choice {
  selected bind $choice
}
```

Composer primitives support intent handlers:

```frame
composer ChatBox {
  draft bind $draft
  send @sendMessage
}
```

### Component invocations

Component invocations are supported inside `view` blocks:

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

Invocations validate against components declared in the same file. Arguments support `name: $state`, `name: "literal"`, and `name bind $state`.

### Slots

Slots define composable content regions inside a component:

```frame
slot Default {
  text "Fallback content"
}
```

### IR and Contracts

Frame can lower UI syntax into renderer-neutral Frame IR and serialize it as JSON:

```bash
frame emit-ir app.frame
```

Frame can generate TypeScript contracts for props, state, and external handlers:

```bash
frame emit-contracts app.frame
```

Generated contracts define `ComponentProps`, `ComponentState`, `ComponentHandlers`, and a shared `FrameEventContext<TState, TProps>`. They do not generate runtime code or overwrite user implementation files.

For standalone web apps, prefer `frame build`. It emits CSS, JSON IR, typed TS IR, contracts, and append-only handler skeletons under `src/generated`:

```bash
frame build
```

User-owned handler implementations live in `src/handlers.ts`; generated files describe the required signatures and runtime contract.

### Runtime status

The DOM runtime currently supports mounting, disposal, element/text creation, nested components, props, state, events, bindings, conditions, style classes, dependency-aware patches, keyed and positional lists, common HTML elements, global attributes, URL safety checks, and form controls. It does not implement SSR, hydration, routing, portals, suspense, or async components.

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

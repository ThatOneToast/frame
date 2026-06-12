# Frame Language Cheat Sheet For Agents

Use this page when generating Frame syntax.

## File Shape

Frame is line-oriented:

```frame
declaration Name {
  property value

  state {
    effect value
  }
}
```

Comments use `//`:

```frame
// Main dashboard layout
grid Dashboard {
  columns sidebar content
}
```

## Top-level Declarations

Use these at file root for styling and layout:

```txt
tokens
grid
area
card
stack
row
text
center
split
overlay
dock
```

Examples:

```frame
grid Dashboard {
  columns sidebar content inspector
}

area Sidebar {
  in Dashboard
  place sidebar
}

card ProjectCard {
  surface panel
  padding medium
}
```

UI primitives such as `action`, `link`, `input`, `editor`, `toggle`, `panel`, `dialog`, `menu`, `toolbar`, and `composer` belong inside `view` blocks, not at the file root. See `docs/ui-primitives.md`.

## Grid

Use `grid` for app shells, page sections, and card grids.

Named columns:

```frame
grid Dashboard {
  columns sidebar content inspector
  gap medium
}
```

Use named columns when child areas should say `place sidebar`.

Percentage columns:

```frame
grid Dashboard {
  columns 25% 50% 25%
  gap medium
}
```

Use percentage columns when the layout should have explicit sidebar/content/inspector widths. Percentages must be `0%` through `100%`.

Responsive card grid:

```frame
grid QuickLinks {
  columns responsive cards
  gap medium
}
```

Useful grid properties:

```txt
columns
rows
gap
height
width
padding
surface
align
justify
```

## Area

Use `area` for a child region inside a grid.

Named placement:

```frame
grid Dashboard {
  columns sidebar content inspector
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}
```

Numeric placement:

```frame
grid Dashboard {
  columns 25% 50% 25%
}

area Sidebar {
  in Dashboard
  col 1
  surface panel
  padding medium
}
```

Area properties:

```txt
in
place
col
row
span
surface
padding
margin
width
height
align
justify
border
shadow
```

## Card

Use `card` for contained reusable UI surfaces:

```frame
card ProjectCard {
  surface panel
  padding large
  radius large
  shadow medium
  text bright

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

Card properties:

```txt
surface
padding
margin
radius
border
shadow
text
color
width
height
align
justify
hover
focus
active
disabled
```

## Stack And Row

Use `stack` for vertical grouping:

```frame
stack SettingsPanel {
  gap medium
  align stretch
}
```

Use `row` for horizontal grouping:

```frame
row Toolbar {
  align center
  justify between
  gap small
}
```

## Center

Use `center` for empty states and loading states:

```frame
center EmptyState {
  height screen
  surface main
  text muted
}
```

## Surface Values

```txt
panel
main
glass
raised
flat
gradient dusk
gradient midnight
gradient aurora
```

Use `surface main` for primary page/content backgrounds.

Use `surface panel` for sidebars, inspectors, cards, and tool panels.

Use gradients for hero cards, feature cards, and interactive highlights.

## Spacing And Shape Values

Spacing:

```txt
none
small
medium
large
xlarge
```

Use with:

```frame
padding medium
margin large
gap small
```

Radius:

```txt
none
small
medium
large
xlarge
pill
full
```

Use with:

```frame
radius large
radius pill
```

## Size Values

Named sizes:

```txt
fill
content
screen
auto
sidebar
narrow
wide
small
medium
large
xlarge
```

Percentage sizes:

```frame
width 25%
width 50%
width 100%
height 50%
height 100%
```

Use percentages only from `0%` through `100%`.

## Alignment Values

```frame
align start
align center
align end
align stretch

justify start
justify center
justify end
justify between
justify around
justify evenly
```

`align` means vertical/cross-axis placement.

`justify` means horizontal/main-axis placement or distribution.

## Color Values

Semantic colors:

```txt
bright
muted
accent
primary
secondary
danger
success
warning
info
```

Basic colors:

```txt
white
black
gray
red
orange
yellow
green
blue
purple
pink
```

Examples:

```frame
text bright
text muted
color accent
background danger
theme success
```

Use `accent` for primary actions, focus rings, active navigation, and highlighted cards.

Use `muted` for helper text, captions, and metadata.

Use `danger`, `success`, `warning`, and `info` for status states.

## Interaction States

Only use these nested blocks:

```txt
hover
focus
focus-visible
focus-within
active
disabled
checked
invalid
required
target
```

Effect values:

```txt
lift
glow
brighten
dim
blur
press
ring
scale
fade
slide
shift
grow
shrink
tilt
pop
smooth
```

Examples:

```frame
card HoverCard {
  hover {
    lift small
    glow accent
    brighten subtle
  }
}

card PrimaryButton {
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

## UI Component Syntax

Components are declared with `component` and contain `props`, `state`, `view`, and `slot` blocks.

```frame
component ChatApp {
  props {
    title text
    count number
  }
  state {
    draft text = ""
    sending bool = false
  }
  view {
    text $title
    action Send {
      text "Send"
      on press @sendMessage
      disabled when $sending
    }
  }
  slot Default {
    text "Fallback"
  }
}
```

### Props vs State

- **Props** have types but no defaults: `props { title text }`
- **State** has types and defaults: `state { draft text = "" }`
- Types: `text`, `string`, `bool`, `number`, `list`

### View Block

Inside `view`, use UI primitives:

```frame
view {
  panel Main {
    text "Hello"
    action Send {
      text "Send"
      on press @sendMessage
    }
  }
}
```

### Data References

- `$name` — references state, props, or loop variables
- `@name` — references external handler functions
- Dotted refs work: `$user.name`, `$item.id`

### Event Bindings

```frame
on press @handler
on click @handler
on keydown.enter @handler
on keydown.ctrl.enter @handler
on click.once @handler
```

Modifiers: `enter`, `escape`, `tab`, `space`, `ctrl`, `shift`, `alt`, `meta`, `left`, `right`, `up`, `down`, `prevent`, `stop`, `once`, `capture`, `passive`

### Bindings

```frame
value bind $state        // two-way binding on input/editor
checked bind $bool       // two-way binding on toggle/choice
selected bind $value     // two-way binding on select/choice
show when $condition     // conditional rendering
disabled when $flag      // conditional attribute
style when $flag = Style // conditional style
```

### Component Invocations

```frame
Greeting(name: "World")
MessageComposer(draft bind $draft)
ChatPanel(channel: $activeChannel)
```

Arguments use `:` for data, `bind` for two-way binding.

### Loops

```frame
for item in $items {
  text $item
}

for item in $items key $selected {
  text $item
}
```

Loop keys must start with `$`.

### Slots

Slots provide fallback content when a component is used as a container:

```frame
component Dialog {
  view {
    card Body {
      text "Default content"
    }
  }
  slot Default {
    text "Fallback"
  }
}
```

### Style Bindings

```frame
action Send:PrimaryButton {
  text "Send"
}

panel Content:GlassPanel {
  text "Hello"
}
```

## Output Expectations

External Frame:

```txt
src/lib/frame/app.frame
src/lib/frame/generated.css
src/lib/frame/generated.ts
```

Inline Svelte Frame:

```svelte
<div class="fr-DemoCard">Demo</div>

<style lang="frame">
  card DemoCard {
    surface panel
    padding medium
  }
</style>
```

Inline blocks emit CSS only. They do not generate `generated.ts`.

## Semantic Styling (2026-06 overhaul)

```frame
tokens default {
  color text #f5f5f5
  surface panel #171722
  space md 1rem
  radius lg 1rem
  breakpoint tablet 48rem
  container content 64rem
}

theme dark uses default {
  surface app #101014
}

layout DashboardShell {
  shell {
    sidebar left fixed 18rem
    main fluid
    inspector right clamp(20rem, 28vw, 28rem)
  }
  gap large
  density comfortable
  below tablet { shell stacked }
}

motion Pressable {
  enter fade up soft
  hover lift sm
  active press
  focus ring accent
  duration normal
  easing smooth
}

recipe Button {
  base {
    align center
    gap small
    radius medium
    motion Pressable
  }
  variant tone {
    primary { background token(color.accent) }
    ghost { background transparent }
  }
}

card Panel {
  background token(surface.panel)
  padding token(space.md)
  motion Pressable
}
```

Notes:

- `token(kind.name)` works in any value position and is validated against the
  resolved contract.
- The first `theme` binds to `:root`; apply others with
  `data-frame-theme="name"` or the generated `applyTheme()` helper.
- `extends` merges by property path (`border.width` refines an inherited
  `border accent` instead of clobbering it).
- Effects accept t-shirt sizes (`lift sm`) alongside the named scales.
- `frame build --css-backend atomic` is experimental; semantic is the default.

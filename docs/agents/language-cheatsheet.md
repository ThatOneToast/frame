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

Use these at file root:

```txt
tokens
grid
area
card
stack
row
button
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
active
disabled
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

button PrimaryButton {
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

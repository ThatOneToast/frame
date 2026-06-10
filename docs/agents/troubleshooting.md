# Frame Troubleshooting For Agents

Use this when reviewing or repairing generated Frame code.

## Validation Commands

Check a Frame file:

```bash
frame check src/lib/frame/app.frame
```

Compile a Frame file:

```bash
frame compile src/lib/frame/app.frame --out src/lib/frame
```

Format a Frame file:

```bash
frame format src/lib/frame/app.frame
```

Local repository versions (run from a project with a `.frame` file):

```bash
cargo run -p frame_cli -- check src/lib/frame/app.frame
cargo run -p frame_cli -- compile src/lib/frame/app.frame --out src/lib/frame
cargo run -p frame_cli -- format src/lib/frame/app.frame
```

## Parser Rules

Frame declarations need a declaration kind, a PascalCase-ish name, and an opening brace:

```frame
card ProjectCard {
  surface panel
}
```

Bad:

```frame
card {
  surface panel
}
```

Bad:

```frame
card ProjectCard
  surface panel
}
```

Nested blocks can only be:

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
gradient
section
animation
below
above
between
container
from
to
0%
25%
50%
75%
100%
```

Bad:

```frame
card ProjectCard {
  mobile {
    padding small
  }
}
```

Use `below mobile` instead:

```frame
card ProjectCard {
  below mobile {
    padding small
  }
}
```

## Semantic Rules

### UI Primitive at File Root

`panel` is a UI primitive that belongs inside `view` blocks, not at the file root.

Bad:

```frame
panel Sidebar {
  surface panel
}
```

Fix (if this is a styling declaration):

```frame
area Sidebar {
  in Dashboard
  place sidebar
  surface panel
}
```

Fix (if this is a UI component):

```frame
component App {
  view {
    panel Sidebar {
      surface panel
    }
  }
}
```

### Area Missing Parent Grid

Bad:

```frame
area Sidebar {
  place sidebar
}
```

Fix:

```frame
grid Dashboard {
  columns sidebar content
}

area Sidebar {
  in Dashboard
  place sidebar
}
```

### Area References Unknown Grid

Bad:

```frame
area Sidebar {
  in AppShell
  place sidebar
}
```

If the grid is named `Dashboard`, fix:

```frame
area Sidebar {
  in Dashboard
  place sidebar
}
```

### Area Places Into Unknown Slot

Bad:

```frame
grid Dashboard {
  columns sidebar content
}

area Inspector {
  in Dashboard
  place inspector
}
```

Fix by adding the slot:

```frame
grid Dashboard {
  columns sidebar content inspector
}

area Inspector {
  in Dashboard
  place inspector
}
```

Or use numeric placement with percentage columns:

```frame
grid Dashboard {
  columns 25% 50% 25%
}

area Inspector {
  in Dashboard
  col 3
}
```

## Invalid Values

### Spacing

Bad:

```frame
padding 12px
gap tiny
```

Fix:

```frame
padding medium
gap small
```

Allowed spacing:

```txt
none small medium large xlarge
```

### Percentage Sizing

Bad:

```frame
width -10%
height 120%%
width abc%
```

Fix:

```frame
width 25%
height 100%
```

Percentages must be `0%` through `100%`.

### Surface

Bad:

```frame
surface dark
surface sidebar
```

Fix:

```frame
surface panel
surface main
surface glass
surface raised
surface flat
surface gradient dusk
```

### Color

Bad:

```frame
color neon
text faint
```

Fix:

```frame
color accent
text muted
text bright
```

Good semantic colors:

```txt
bright muted accent primary secondary danger success warning info
```

## Svelte Integration Problems

### Generated Class Missing

If markup uses `class={ui.ProjectCard}`, make sure `ProjectCard` exists in the external `.frame` file:

```frame
card ProjectCard {
  surface panel
  padding medium
}
```

Then compile:

```bash
frame compile src/lib/frame/app.frame --out src/lib/frame
```

### Inline Class Missing

If the declaration is inside `<style lang="frame">`, do not use `ui.ProjectCard`.

Use:

```svelte
<article class="fr-ProjectCard">Project</article>

<style lang="frame">
  card ProjectCard {
    surface panel
    padding medium
  }
</style>
```

### CSS Not Applied For External Frame

Make sure Svelte imports the generated CSS:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>
```

### CSS Completions In Frame Style Block

Expected block:

```svelte
<style lang="frame">
  card DemoCard {
    surface panel
  }
</style>
```

If the editor suggests CSS first, verify:

- the Zed Frame extension is installed
- `frame_lsp` is available
- the style tag uses exactly `lang="frame"`
- the editor loads the Svelte injection query

## Repair Strategy For Agents

1. Identify whether the code is styling (file root) or UI (inside `component { view { ... } }`).
2. Check top-level declarations use valid declaration keywords (`grid`, `area`, `card`, `stack`, `row`, `text`, `tokens`, `keyframes`, etc.).
3. Check UI primitives are inside `view` blocks, not at file root.
4. Check every `area` against its grid.
5. Replace raw CSS values with Frame tokens.
6. Replace invalid nested blocks with supported state blocks.
7. Use `surface main` for primary content and `surface panel` for secondary regions.
8. Use `text`, `color`, and `background` semantic tokens.
9. Check component args use `:` not `=`.
10. Check loop keys start with `$`.
11. Check `show when` is inside an element, not standalone.
12. Check `slot` is at component level, not inside declarations.
13. Prefer complete vertical examples over partial snippets.

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
active
disabled
```

Bad:

```frame
card ProjectCard {
  mobile {
    padding small
  }
}
```

Responsive nested blocks are not implemented in the current parser.

## Semantic Rules

### Unknown Declaration

Bad:

```frame
panel Sidebar {
  surface panel
}
```

Fix:

```frame
area Sidebar {
  in Dashboard
  place sidebar
  surface panel
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

1. Identify whether the code is external Frame or inline Svelte Frame.
2. Check top-level declarations.
3. Check every `area` against its grid.
4. Replace raw CSS values with Frame tokens.
5. Replace invalid nested blocks with supported state blocks.
6. Use `surface main` for primary content and `surface panel` for secondary regions.
7. Use `text`, `color`, and `background` semantic tokens.
8. Prefer complete vertical examples over partial snippets.

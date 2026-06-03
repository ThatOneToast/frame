# Frame

Frame is an experimental CSS DSL for Svelte-focused UI development.

The goal is to let developers describe interface intent with concepts like `grid`, `card`, `surface`, `hover`, `glow`, `area`, and `place`, then compile that into normal CSS and TypeScript class exports.

Frame is not meant to expose all of CSS. It is meant to make the common UI path easy while still allowing controlled escape hatches for advanced layouts.

## Example

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

grid QuickLinks {
  columns responsive cards
  gap medium
}

card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

Generated usage in Svelte:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Channels</aside>
  <main class={ui.QuickLinks}>
    <a class={ui.QuickLinkCard}>Docs</a>
  </main>
</div>
```

Set up Frame in an existing Svelte project:

```bash
frame init svelte
```

During local development from this repository:

```bash
cargo run -p frame_cli -- init svelte
```

Use external `.frame` files when you want typed `ui` exports. Use Svelte `<style lang="frame">` blocks for component-local styles:

```svelte
<div class="fr-HoverCard">Docs</div>

<style lang="frame">
  card HoverCard {
    surface gradient dusk
    padding large
  }
</style>
```

## Workspace

```txt
crates/
  frame_core/      AST, diagnostics, semantic model, tokens
  frame_parser/    parser and parse errors
  frame_codegen/   CSS and TypeScript generation
  frame_cli/       compile, check, format, watch, init
  frame_lsp/       LSP server
editors/
  zed/             Zed extension scaffold
examples/
  svelte/          intended Svelte output example
```

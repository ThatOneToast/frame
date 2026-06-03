# Frame

Frame is an experimental structured CSS language for Svelte-focused UI development.

The goal is to let developers describe interface intent with concepts like `grid`, `card`, `surface`, `hover`, `glow`, `area`, and `place`, then compile that into normal CSS and TypeScript class exports.

Frame should expose the full power of CSS through structured, discoverable, teachable syntax. It starts with first-class layout, surfaces, gradients, transitions, animations, responsive rules, container queries, tokens, and safe advanced CSS escape hatches.

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

  below tablet {
    columns content
  }
}

keyframes FloatIn {
  from {
    opacity 0
    transform translateY(12px) scale(0.98)
  }

  to {
    opacity 1
    transform translateY(0) scale(1)
  }
}

card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium
  animation FloatIn {
    duration 240ms
    ease smooth
    fill both
  }

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

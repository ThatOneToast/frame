# Frame Svelte Example

This example demonstrates both Frame integration paths:

- External `.frame` files through the Vite plugin.
- Inline Svelte `<style lang="frame">` blocks through the Svelte preprocessor.

The local package uses the Rust CLI as the compiler backend. The example config sets `frameBin` to `cargo run -p frame_cli --quiet --`, so it works from this repository without installing a global `frame` command.

Install and run:

```bash
cd examples/svelte
npm install
npm run dev
```

To set up a separate Svelte project, run:

```bash
frame init svelte
```

From this repository:

```bash
cargo run -p frame_cli -- init svelte
```

External Frame usage:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Sidebar</aside>

  <main class={ui.Content}>
    <div class={ui.QuickLinks}>
      <a class={ui.QuickLinkCard}>Docs</a>
      <a class={ui.QuickLinkCard}>GitHub</a>
    </div>
  </main>

  <section class={ui.Inspector}>Inspector</section>
</div>
```

Inline Frame usage:

```svelte
<main class="fr-PageShell">
  <section class="fr-HeroCard">
    <h1>Frame Inline Style</h1>
  </section>
</main>

<style lang="frame">
  stack PageShell {
    gap large
    padding large
    surface main
  }

  card HeroCard {
    surface gradient dusk
    padding large
    radius large
    shadow medium
  }
</style>
```

Inline blocks emit CSS only. Use raw class names like `fr-HeroCard` for inline-only declarations, or use external `.frame` files when you want typed `ui` exports from `generated.ts`.

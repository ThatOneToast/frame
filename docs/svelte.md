# Svelte Integration

Frame currently integrates with Svelte through generated CSS and TypeScript files.

Compile once:

```bash
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

Watch during development:

```bash
cargo run -p frame_cli -- watch examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

Use generated classes from Svelte:

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

If Svelte reports missing generated files, run `frame compile` first and confirm `generated.css` and `generated.ts` exist beside the `.frame` file. The example app assumes Svelte can resolve `$lib/frame/generated` to `src/lib/frame/generated.ts`.

A Vite plugin is planned, but not required for the current integration path.

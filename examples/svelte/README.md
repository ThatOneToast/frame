# Frame Svelte Example

This example uses `src/lib/frame/app.frame` as the source of generated classes.

Compile generated files:

```bash
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

Watch while editing:

```bash
cargo run -p frame_cli -- watch examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

Svelte usage:

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

If imports fail, run the compile command and confirm `generated.css` and `generated.ts` exist in `src/lib/frame`.

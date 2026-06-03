# Svelte Integration

Frame integrates with Svelte through external `.frame` files and inline `<style lang="frame">` blocks.

## One-command setup

Run this inside a Svelte or SvelteKit project:

```bash
frame init svelte
```

During local development from this repository, use:

```bash
cargo run -p frame_cli -- init svelte
```

The command creates `src/lib/frame/app.frame`, generates `generated.css` and `generated.ts`, updates Svelte/Vite config when safe, and adds `@frame/svelte` to `package.json` when one exists.

Preview changes without writing:

```bash
frame init svelte --dry-run
```

## External `.frame` files

Use the Vite plugin to compile a Frame file into `generated.css` and `generated.ts`.

```ts
// vite.config.ts
import { framePlugin } from '@frame/svelte/vite';

export default {
  plugins: [
    framePlugin({
      input: 'src/lib/frame/app.frame',
      outDir: 'src/lib/frame'
    })
  ]
};
```

Then use the generated class map from Svelte:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Sidebar</aside>
  <main class={ui.Content}>Content</main>
</div>
```

The plugin compiles on dev server startup, build startup, and file changes.

The CLI path still works:

```bash
cargo run -p frame_cli -- compile examples/svelte/src/lib/frame/app.frame --out examples/svelte/src/lib/frame
```

## Inline style blocks

Use the Svelte preprocessor for component-local Frame styles.

```js
// svelte.config.js
import adapter from '@sveltejs/adapter-auto';
import { framePreprocess } from '@frame/svelte';

export default {
  kit: {
    adapter: adapter()
  },
  preprocess: [
    framePreprocess()
  ]
};
```

Then write Frame inside the component style block:

```svelte
<div class="fr-HoverCard">
  Docs
</div>

<style lang="frame">
  card HoverCard {
    surface gradient dusk
    padding large
    radius large
  }
</style>
```

The preprocessor returns normal CSS to Svelte, so Svelte applies its normal component style scoping. Frame does not add Svelte scope hashes itself.

## Class names

Inline blocks currently emit CSS only. Use raw class names for inline-only declarations:

```svelte
<div class="fr-HoverCard">
```

For typed class names, put declarations in an external `.frame` file and import `ui`:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<a class={ui.QuickLinkCard}>Docs</a>
```

## Which style should I choose?

Use external `.frame` files when you want typed generated class names from `generated.ts`.

Use `<style lang="frame">` for component-local styles that should live beside the Svelte markup.

Inline style blocks currently use raw generated class names like `fr-HoverCard`.

## Sidebar layout example

```svelte
<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>

<style lang="frame">
  grid Dashboard {
    columns 25% 50% 25%
    gap medium
    height screen
  }

  area Sidebar {
    in Dashboard
    col 1
    surface panel
    padding medium
  }

  area Content {
    in Dashboard
    col 2
    surface main
    padding large
  }

  area Inspector {
    in Dashboard
    col 3
    surface panel
    padding medium
  }
</style>
```

## Known limitations

- Inline style blocks emit CSS only.
- Inline blocks do not generate `generated.ts`.
- Source maps are not emitted yet.
- The Node integration shells out to the Frame CLI. Set `FRAME_BIN` or `frameBin` if the `frame` command is not on `PATH`.
- For typed class names, use external `.frame` files.

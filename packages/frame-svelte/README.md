# @frame/svelte

Svelte and Vite integration for Frame.

## One-command setup

Inside a Svelte or SvelteKit project:

```bash
frame init svelte
```

From this repository during development:

```bash
cargo run -p frame_cli -- init svelte
```

This creates `src/lib/frame/app.frame`, generates `generated.css` and `generated.ts`, and updates Svelte/Vite config when safe.

## External `.frame` files

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

This emits:

```txt
src/lib/frame/generated.css
src/lib/frame/generated.ts
```

Use it from Svelte:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<a class={ui.QuickLinkCard}>Docs</a>
```

## Inline `<style lang="frame">`

```js
// svelte.config.js
import { framePreprocess } from '@frame/svelte';

export default {
  preprocess: [
    framePreprocess()
  ]
};
```

```svelte
<a class="fr-QuickLinkCard">Docs</a>

<style lang="frame">
  card QuickLinkCard {
    surface gradient dusk
    padding large
    radius large
  }
</style>
```

## Compiler command

The package shells out to the Frame CLI. By default it runs `frame`. Override it when developing from this repository:

```ts
framePlugin({
  frameBin: 'cargo run -p frame_cli --quiet --'
});
```

or:

```bash
FRAME_BIN="cargo run -p frame_cli --quiet --" npm run dev
```

## Limitations

- Inline style blocks currently emit CSS only.
- Inline blocks do not generate `generated.ts`.
- For typed class names, use external `.frame` files.
- Raw class names like `fr-QuickLinkCard` are valid for inline styles.
- Source maps are not emitted yet.

Choose external `.frame` files when you want typed `ui` exports. Choose inline `<style lang="frame">` for component-local styles.

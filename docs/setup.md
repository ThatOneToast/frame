# Setup

Frame compiles `.frame` files into normal CSS and TypeScript class exports.

```bash
frame compile src/lib/frame/app.frame --out src/lib/frame --include src/lib/frame
```

Svelte usage:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<section class={ui.BrandCard}>Content</section>
```

Vite plugin:

```ts
framePlugin({
  input: 'src/lib/frame/app.frame',
  outDir: 'src/lib/frame',
  include: ['src/lib/frame']
});
```

# Setup

Frame compiles `.frame` files into CSS, renderer-neutral IR, TypeScript contracts, and handler skeletons.

Standalone web app:

```bash
frame new my-app --template web
cd my-app
npm install
npm run dev
```

The generated web template runs `frame build` before `npm run dev`, `npm run check`, and `npm run build`.

```txt
src/
  app.frame
  handlers.ts
  generated/
    app.ir.json
    app.ir.ts
    frame.types.ts
    frame.handlers.ts
```

- `app.ir.ts` and `frame.types.ts` are generated-only and overwritten only when content changes.
- `frame.handlers.ts` is a generated reference file. Existing content is preserved and missing stubs are appended.
- `src/handlers.ts` is user-owned implementation code.

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

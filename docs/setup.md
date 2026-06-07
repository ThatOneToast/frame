# Setup

Frame compiles `.frame` files into CSS, renderer-neutral IR, TypeScript contracts, and handler skeletons.

Standalone web app:

```bash
frame new my-app --template web
cd my-app
npm install
npm run dev
```

The generated web template runs `frame build` before `npm run build` and `npm run check`, and runs `frame build --watch` alongside Vite for `npm run dev`.

```txt
src/
  app.frame
  app-theme.frame
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
- `src/app-theme.frame` is an optional project theme file. Styles and declarations defined there are automatically available to every Frame file in the project without explicit `#include`.

## Theme File

`app-theme.frame` lives in the same directory as your entry file (typically `src/app-theme.frame`). When present, Frame automatically includes it as a lower-precedence source:

1. Local declarations in the current file
2. Explicit `#include` files
3. `app-theme.frame`
4. Built-in default Frame theme values

This lets you define shared styles like `PrimaryButton` or `AppShell` once and reference them throughout the project.

## Watch Mode

Rebuild automatically when any `.frame` file changes:

```bash
frame build --watch
```

In a web template, `npm run dev` runs Frame watch and Vite dev in parallel:

```bash
npm run frame:watch   # frame build --watch
npm run vite:dev      # vite
npm run dev           # runs both in parallel
```

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

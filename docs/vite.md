# Vite Plugin

The Frame Vite plugin compiles external `.frame` files into Svelte-friendly generated files.

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

Default options:

```ts
{
  input: 'src/lib/frame/app.frame',
  outDir: 'src/lib/frame',
  generatedCssName: 'generated.css',
  generatedTsName: 'generated.ts',
  watch: true
}
```

The plugin compiles on dev server startup, build startup, and watched `.frame` file changes. It writes readable CSS plus stable TypeScript exports:

```txt
src/lib/frame/generated.css
src/lib/frame/generated.ts
```

If `frame` is not installed globally, pass a compiler command:

```ts
framePlugin({
  frameBin: 'cargo run -p frame_cli --quiet --'
})
```

During development, normal Frame authoring errors are printed to the terminal without crashing the dev server. During build, compile errors fail the build.

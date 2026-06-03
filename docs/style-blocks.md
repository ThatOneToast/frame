# Svelte Style Blocks

Frame supports component-local styles through Svelte preprocessing.

```js
// svelte.config.js
import { framePreprocess } from '@frame/svelte';

export default {
  preprocess: [
    framePreprocess()
  ]
};
```

Write Frame in a component style block:

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

    hover {
      lift small
      glow accent
    }
  }
</style>
```

The preprocessor compiles Frame to plain CSS before Svelte compiles the component. Svelte then scopes the returned CSS normally.

Editor support should treat the block as Frame, not CSS. In Zed, the local extension includes a Svelte injection query for `lang="frame"` and `frame_lsp` extracts embedded Frame blocks for completions, diagnostics, and hover docs.

Known limitations:

- Inline style blocks emit CSS only.
- Inline style blocks do not generate `generated.ts`.
- Raw class names like `fr-HeroCard` are valid for inline styles.
- Use external `.frame` files when you want typed classes through `ui`.
- Source maps may be limited initially.

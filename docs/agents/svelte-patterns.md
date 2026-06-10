# Svelte Patterns For Frame Agents

Use this page when generating Svelte components or setup instructions that involve Frame.

## Web App Template

For new projects, use the web app template:

```bash
frame new my-app --template web
```

This creates a standalone Frame project with DOM runtime, not Svelte.

For Svelte integration:

```bash
frame init svelte
```

## Pick The Correct Integration

External `.frame` file:

- best for shared layouts
- best for typed class names
- generates `generated.css`
- generates `generated.ts`
- Svelte markup uses `class={ui.Name}`

Inline `<style lang="frame">`:

- best for component-local styles
- emits CSS only
- does not generate TypeScript
- Svelte markup uses raw class names like `class="fr-Name"`

## One-command Setup

Tell users to run:

```bash
frame init svelte
```

For dry-run:

```bash
frame init svelte --dry-run
```

From this repository:

```bash
cargo run -p frame_cli -- init svelte
```

## External Frame File Setup

Expected files:

```txt
src/lib/frame/app.frame
src/lib/frame/generated.css
src/lib/frame/generated.ts
```

Vite config:

```ts
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

Svelte usage:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.Dashboard}>
  <aside class={ui.Sidebar}>Channels</aside>
  <main class={ui.Content}>Messages</main>
</div>
```

Frame file:

```frame
grid Dashboard {
  columns sidebar content
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

area Content {
  in Dashboard
  place content
  surface main
  padding large
}
```

## Inline Frame Style Setup

Svelte config:

```js
import { framePreprocess } from '@frame/svelte';

export default {
  preprocess: [
    framePreprocess()
  ]
};
```

Component:

```svelte
<article class="fr-ProjectCard">
  <h2>Project Alpha</h2>
  <p>Deploy-ready workspace.</p>
</article>

<style lang="frame">
  card ProjectCard {
    surface panel
    padding large
    radius large
    shadow medium
    text bright

    hover {
      lift small
      glow accent
      brighten subtle
    }
  }
</style>
```

Do not import `ui` for inline-only declarations. The inline preprocessor returns CSS only.

## Full SvelteKit Example With External Frame

`src/lib/frame/app.frame`:

```frame
grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

area Content {
  in Dashboard
  place content
  surface main
  padding large
}

area Inspector {
  in Dashboard
  place inspector
  surface panel
  padding medium
}

grid QuickLinks {
  columns responsive cards
  gap medium
}

card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium
  text bright

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

`+page.svelte`:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.Dashboard}>
  <aside class={ui.Sidebar}>Channels</aside>

  <main class={ui.Content}>
    <div class={ui.QuickLinks}>
      <a class={ui.QuickLinkCard} href="/docs">Docs</a>
      <a class={ui.QuickLinkCard} href="/settings">Settings</a>
    </div>
  </main>

  <section class={ui.Inspector}>Details</section>
</div>
```

## Full Svelte Component With Inline Frame

```svelte
<section class="fr-SettingsPanel">
  <header class="fr-Toolbar">
    <h2>Settings</h2>
    <button class="fr-PrimaryButton">Save</button>
  </header>

  <article class="fr-PanelCard">
    <p>Profile and workspace preferences.</p>
  </article>
</section>

<style lang="frame">
  stack SettingsPanel {
    gap medium
    surface main
    padding large
  }

  row Toolbar {
    align center
    justify between
    gap small
  }

  card PanelCard {
    surface panel
    padding large
    radius large
    shadow soft
    text muted
  }

  card PrimaryButton {
    surface raised
    padding medium
    radius pill
    text bright

    focus {
      ring accent
    }

    active {
      press
    }
  }
</style>
```

## Agent Pitfalls

Do not write this when the declaration only exists inline:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
</script>

<div class={ui.ProjectCard}></div>

<style lang="frame">
  card ProjectCard {
    surface panel
  }
</style>
```

Use this instead:

```svelte
<div class="fr-ProjectCard"></div>

<style lang="frame">
  card ProjectCard {
    surface panel
  }
</style>
```

Do not forget the generated CSS import for external `.frame` files:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>
```

Do not mix `place` with missing named columns:

```frame
grid Dashboard {
  columns 25% 50% 25%
}

area Sidebar {
  in Dashboard
  place sidebar
}
```

Use numeric placement for percentage columns:

```frame
area Sidebar {
  in Dashboard
  col 1
}
```

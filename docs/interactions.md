# Interactions

Frame interaction blocks describe state intent instead of raw pseudo-class CSS.

Supported state blocks:

```txt
hover focus active disabled
```

Use effect keywords inside state blocks:

```txt
lift glow brighten dim blur press ring scale fade slide
```

```svelte
<a class="fr-QuickLinkCard">Docs</a>

<style lang="frame">
  card QuickLinkCard {
    surface gradient dusk
    padding large
    radius large
    shadow medium
    text bright

    hover {
      lift small
      glow accent
    }
  }
</style>
```

Generated CSS emits state selectors for the generated class while keeping the Frame source focused on interaction behavior.

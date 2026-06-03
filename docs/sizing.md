# Sizing

Frame sizing uses high-level values and percentages:

```txt
fill content screen auto sidebar narrow wide 25% 33% 50% 66% 75% 100%
```

Use `screen` for viewport-height regions, `fill` for available space, and percentages for explicit dashboard ratios.

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

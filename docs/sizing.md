# Sizing

Frame sizing uses high-level values and percentages:

```txt
fill content screen auto sidebar narrow wide 25% 33% 50% 66% 75% 100%
```

Use `screen` for viewport-height regions, `fill` for available space, and percentages for explicit dashboard ratios.

Physical sizing properties:

```frame
width fill
height screen
min-width zero
max-width content
min-height screen
max-height 100%
```

Logical sizing properties use the same values and emit CSS logical properties:

```frame
inline-size fill
block-size screen
min-inline-size zero
max-inline-size content
min-block-size zero
max-block-size 100%
```

In horizontal writing modes, inline size usually behaves like width and block size usually behaves like height.

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

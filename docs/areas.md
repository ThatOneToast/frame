# Areas

`area` declares a child region inside a `grid`.

Use `in GridName` to connect the area to its parent grid, then use `place` for named sections or `col` / `row` for numeric tracks.

## Named Dashboard

```svelte
<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>

<style lang="frame">
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
</style>
```

Use `place` when the grid columns are named. Use `col 1`, `col 2`, and `col 3` when the grid uses explicit percentage columns.

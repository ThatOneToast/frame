# Sizing

Frame sizing uses high-level values and percentages:

```txt
fill content screen auto sidebar narrow wide chart panel input dashboard 25% 33% 50% 66% 75% 100%
```

Use `screen` for viewport-height regions, `fill` for available space, and percentages for explicit dashboard ratios.

## Content Sizing Tokens

For content regions that need larger sizes than spacing tokens provide:

| Token | CSS Value | Use Case |
|-------|-----------|----------|
| `chart` | `12rem` | Chart panels, data visualizations |
| `panel` | `16rem` | Side panels, moderate-height regions |
| `sidebar` | `18rem` | Navigation sidebars |
| `narrow` | `12rem` | Narrow fixed-width regions |
| `wide` | `32rem` | Wide content regions (also used for search inputs) |
| `input` | `32rem` | Search bars, form input containers |
| `dashboard` | `96rem` | Dashboard content panels, large app content areas |

These are distinct from spacing tokens (`small`, `medium`, `large`) which are designed for padding and gaps.

### When to use `input` vs `dashboard`

The `input` and `dashboard` tokens both set max-width, but for different contexts:

- `max-width input` (32rem): Use for search bars, form inputs, and narrow content containers. This prevents inputs from stretching too wide in large viewports.
- `max-width dashboard` (96rem): Use for dashboard content areas, main content panels, and app-level containers that need to fill the viewport. Combined with `width fill`, this lets content expand up to 96rem before capping.

```frame
row SearchBar {
  max-width input
  flex grow 1
}

stack DashboardContent {
  width fill
  max-width dashboard
}
```

Do **not** use `wide` or `input` for dashboard content containers -- they cap at 32rem, which is too narrow for app layouts.

```frame
row ChartBars {
  height chart
  flex grow 1
}

stack SidePanel {
  height panel
}
```

## Spacing Tokens (for padding/gap)

```frame
padding small    // 0.5rem
padding medium   // 1rem
padding large    // 1.5rem
padding xlarge   // 2rem
```

## Min-height Resets

Use `min-height none` or `min-height zero` to reset min-height:

```frame
min-height none    // 0
min-height zero    // 0
```

## Physical Sizing Properties

```frame
width fill
height screen
min-width zero
max-width content
min-height screen
max-height 100%
```

## Logical Sizing Properties

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

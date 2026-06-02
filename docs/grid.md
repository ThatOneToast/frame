# Grid

`grid` is a first-class Frame layout concept. Use named columns and rows so child areas can claim readable places.

```frame
grid Dashboard {
  columns sidebar content inspector
  rows main
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
}

area Content {
  in Dashboard
  place content
}
```

Responsive card grids use intent instead of grid-template syntax:

```frame
grid CardGrid {
  columns responsive cards
  gap medium
}
```

Generated CSS uses normal CSS grid with stable class names.

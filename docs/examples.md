# Examples

## App Shell

```frame
grid AppShell {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in AppShell
  place sidebar
  surface panel
  padding medium
}
```

## Responsive Cards

```frame
grid QuickLinks {
  columns responsive cards
  gap medium
}

card QuickLinkCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

## Toolbar

```frame
row Toolbar {
  gap small
  align center
  justify between
  border soft
}
```

## Modal Layer

```frame
overlay ModalLayer {
  surface glass
  position center
  z modal
}
```

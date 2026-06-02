# Layout

Frame layout declarations cover common app structure without requiring raw CSS.

```frame
stack SettingsPanel {
  gap medium
  align stretch
}

row Toolbar {
  gap small
  align center
  justify between
}

center EmptyState {
  height screen
}
```

Two-pane and layered layouts:

```frame
split AppLayout {
  gap medium
}

overlay ModalLayer {
  position center
  z modal
}

dock AppDock {
  surface glass
  padding medium
}
```

Supported sizing intent includes:

```frame
width fill
width content
width sidebar
height screen
min-height screen
max-width content
```

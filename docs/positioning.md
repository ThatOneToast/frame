# Positioning

Use `anchor` for common sticky edge placement:

```frame
row TopBar {
  anchor top
  padding x medium
  padding top small
  surface panel
}
```

Generated CSS:

```css
.fr-TopBar {
  position: sticky;
  top: 0;
  padding-inline: var(--frame-space-medium);
  padding-top: var(--frame-space-small);
}
```

Supported anchors:

```txt
top bottom left right top-left top-right bottom-left bottom-right
```

Targeted spacing works for `padding` and `margin`:

```frame
padding top large
padding right medium
padding bottom small
padding left medium
padding x medium
padding y large
```

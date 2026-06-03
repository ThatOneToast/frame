# Borders

Borders describe edge emphasis without writing raw CSS.

```frame
card WarningCard {
  surface panel
  border warning
  border width medium
  radius large
}
```

Common values:

- `border none`
- `border soft`
- `border strong`
- `border accent`
- `border muted`
- `border danger`
- `border success`
- `border warning`
- `border width small`
- `border width medium`
- `border width large`
- `border radius large`

Border colors can use custom color tokens:

```frame
tokens Brand {
  color brand-muted #a78bfa
}

card BrandCard {
  border brand-muted
}
```

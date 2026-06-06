# Borders

Borders describe edge emphasis without writing raw CSS.

```frame
card WarningCard {
  surface panel
  border warning
  border width medium
  border style dashed
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
- `border style solid`
- `border style dashed`
- `border style dotted`
- `border style double`
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

## Outline

```frame
button PrimaryAction {
  outline accent
  outline offset small
}

button FlatAction {
  outline none
}
```

`outline offset` uses Frame spacing tokens:

```frame
outline offset none
outline offset small
outline offset medium
outline offset large
outline offset xlarge
```

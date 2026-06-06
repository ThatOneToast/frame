# Typography

Text declarations express type role and emphasis.

```frame
text PageTitle {
  size heading
  weight bold
  color bright
}

text MutedLabel {
  size caption
  color muted
}

text CodeText {
  font mono
  size body
}
```

Use `text` or `color` values for semantic color intent, and `font`, `size`, and `weight` for type intent.

## Alignment and Casing

```frame
text SectionLabel {
  align-text start
  case uppercase
}

text ArticleLead {
  align-text justify
  case normal
}
```

`align-text` supports `left`, `center`, `right`, `start`, `end`, `justify`, and `match-parent`.

`case` supports `uppercase`, `lowercase`, `capitalize`, and `normal`.

## Decoration and Whitespace

```frame
text LinkText {
  decoration underline
}

text MessageBody {
  whitespace pre-wrap
  word-break break-word
  hyphenate auto
}
```

`decoration` emits `text-decoration-line` with `none`, `underline`, `overline`, or `line-through`.

`whitespace` emits `white-space` with `normal`, `nowrap`, `pre`, `pre-wrap`, `pre-line`, or `break-spaces`.

`word-break` emits `word-break` with `normal`, `break-all`, `keep-all`, or `break-word`.

`hyphenate` emits `hyphens` with `none`, `manual`, or `auto`.

# Diagnostics

Frame diagnostics are meant to teach the language while pointing to the failing span.

Examples:

```frame
card Demo {
  surface pannel
}
```

Reports that `pannel` is not a known surface and suggests `panel`.

```frame
area Sidebar {
  in Dashbord
}
```

Reports an unknown grid and suggests the closest grid name when one exists.

```frame
grid Dashboard {
  columns 25%% 50% 25%
}
```

Reports that `25%%` is not a valid percentage and suggests values like `25%`, `50%`, and `100%`.

Diagnostics come from the parser and semantic validator, so the CLI and LSP share the same messages.

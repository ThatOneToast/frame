# Code Actions

The Frame LSP provides quick fixes for common authoring mistakes and layout scaffolding.

Current actions include:

- Replace close typos such as `surface pannel` with `surface panel`.
- Create a missing grid referenced by `area ... in GridName`.
- Add a missing `place` line to an area.
- Create matching `area` blocks from a named `grid`.
- Convert three named grid columns to `columns 25% 50% 25%`.
- Add hover lift/glow effects to a card.
- Convert common browser primitives such as `button`, `div`, and `a` to Frame primitives such as `action`, `panel`, and `link`.
- Convert browser event attributes such as `onclick` and `onchange` to Frame event bindings such as `on press @handler`.
- Create a missing same-file style skeleton for unresolved style bindings (automatic lookup, explicit binding, and conditional aliases).
- Create a missing handler skeleton in a companion `handlers.ts` file.
- Create a missing state entry with inferred type (`text`, `bool`, `number`, or `list`).
- Create a missing prop entry with inferred type.

These actions are intentionally design-intent-first. They create Frame concepts like grids, areas, surfaces, and effects rather than raw CSS properties.

## Cross-File Limitations

If a missing symbol belongs in another file, the LSP currently creates the edit in the current file or generates a companion file. Workspace edits that create symbols in arbitrary imported files are not yet supported. This limitation is documented so users understand when to move generated skeletons manually.

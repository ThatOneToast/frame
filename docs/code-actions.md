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
- Create a missing same-file style skeleton for unresolved style bindings.

These actions are intentionally design-intent-first. They create Frame concepts like grids, areas, surfaces, and effects rather than raw CSS properties.

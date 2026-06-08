# Code Actions

The Frame LSP provides quick fixes for common authoring mistakes and layout scaffolding.

All code actions are driven by the canonical language registry in `crates/frame_core/src/language.rs`. The LSP uses registry entries to suggest replacements, create skeletons, and validate intent.

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
- Map an automatic style lookup node to an existing style declaration (e.g., `action Send` → `action Send:PrimaryButton`).
- Create a missing handler skeleton in a companion `handlers.ts` file.
- Create a missing state entry with inferred type (`text`, `bool`, `number`, or `list`).
- Create a missing prop entry with inferred type.

These actions are intentionally design-intent-first. They create Frame concepts like grids, areas, surfaces, and effects rather than raw CSS properties.

## Cross-File Workspace Edits

When a missing symbol is defined in an included file, the LSP creates a workspace edit that targets the correct file. This works for:

- **Styles** in `styles.frame` — when a style binding is unresolved in a component, the skeleton is added to the first included file that contains style declarations.
- **Handlers** in `handlers.ts` — when an event references an undefined handler, the skeleton is added to the first included file that contains handler declarations.
- **State** in any included `.frame` file — when a state reference is undefined, the entry is added to the first included file that contains state declarations.
- **Props** in any included `.frame` file — when a prop reference is undefined, the entry is added to the first included file that contains prop declarations.

The LSP uses `DocumentChanges::Operations` with `TextDocumentEdit` so the editor applies the edit atomically across files. If no appropriate included file exists, the LSP falls back to the current file or a companion file.

## Current Limitations

- Component creation across files is not yet supported. If you reference an undefined component, you must create it manually.
- The LSP does not yet suggest import restructuring when a symbol exists in an un-included file.

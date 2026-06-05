# Architecture

Frame is moving toward a compiler and renderer split.

```txt
Frame source
  -> parser
  -> AST
  -> semantic model
  -> Frame IR
  -> renderer output
```

## Compiler Layers

Parser:

- Reads `.frame` files.
- Produces source spans.
- Keeps syntax separate from renderers.

Semantic model:

- Resolves names.
- Checks style references.
- Checks state and prop references.
- Checks handler references.
- Reports diagnostics for CLI and LSP.

Frame IR:

- Stores component structure.
- Stores node data.
- Stores style bindings.
- Stores event bindings.
- Stores source mappings.

Code generation:

- Emits CSS.
- Emits TypeScript contracts.
- Emits runtime definitions.
- Emits static HTML when possible.

## First Target

The first serious target should be the browser DOM. Tauri can use the same output through WebView.

Native desktop renderers should wait until the IR is stable.

## Migration Plan

1. Keep current styling output working.
2. Add UI syntax.
3. Add Frame IR.
4. Add TypeScript contracts.
5. Add DOM runtime.
6. Build a small web example.
7. Revisit Svelte as a compatibility layer.

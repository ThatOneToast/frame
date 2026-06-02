# LSP

The Frame LSP provides editor intelligence for `.frame` files.

Current support:

- Parser and semantic diagnostics.
- Completions for declaration keywords, property keywords, token values, effects, typography, and responsive keywords.
- Hover docs for common language concepts.
- Document formatting with two-space indentation and stable blank lines.

Formatting preserves comment text with conservative line formatting.

Zed setup uses the existing extension under `editors/zed`. The extension handles `.frame` syntax highlighting and can launch `frame_lsp` when the binary is available in the workspace or user path.

Known limitations:

- Responsive blocks are documented and completed, but codegen support is not finished.
- The current formatter is line-oriented because the parser does not yet preserve comments in the AST.
- The Vite plugin is future work; use `frame watch` for Svelte development.

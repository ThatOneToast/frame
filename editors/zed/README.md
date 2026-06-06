# Frame Zed Extension

This directory contains Zed language support for Frame, an experimental
structured CSS and UI language.

Repository: `https://github.com/ThatOneToast/frame`

## Support

- Associates `.frame` files with the Frame language.
- Uses Tree-sitter for syntax parsing and highlighting.
- Highlights declarations, declaration names, `#include`, state blocks,
  properties, values, numeric values, percentages, hex colors, `//` comments,
  UI component syntax, props, state, view, slots, event bindings, and data
  references.
- Registers `frame_lsp` for diagnostics, scope-aware completions, rich
  completion docs, hover docs, formatting, document symbols, document links,
  go-to-definition, references, code actions, semantic tokens, and folding.
- Includes samples for layouts, colors, effects, gradients, imports, UI
  components, and broad keyword highlighting.

## Local Installation

1. Build the language server:

```bash
cargo build -p frame_lsp
```

2. Open Zed.
3. Run `zed: install dev extension`.
4. Select this directory:

```txt
editors/zed
```

The extension resolves the server command in this order:

1. `FRAME_LSP`
2. `frame_lsp` on `PATH`

Install the language server on `PATH` from this repository:

```bash
cargo install --path crates/frame_lsp
```

You can point directly at the binary:

```bash
FRAME_LSP="/path/to/frame/target/debug/frame_lsp" zed .
```

Or launch Zed with the binary on `PATH`:

```bash
PATH="/path/to/frame/target/debug:$PATH" zed .
```

## Tree-Sitter Grammar

For local parser work, generate the grammar from the grammar directory:

```bash
cd editors/zed/tree-sitter-frame
npm install
npx tree-sitter generate --abi 14
```

Parse sample files:

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter parse --grammar-path . ../samples/app.frame
npx tree-sitter parse --grammar-path . ../samples/highlighting.frame
npx tree-sitter parse --grammar-path . ../samples/imports.frame
npx tree-sitter parse --grammar-path . ../samples/grid.frame
npx tree-sitter parse --grammar-path . ../samples/card.frame
npx tree-sitter parse --grammar-path . ../samples/states.frame
npx tree-sitter parse --grammar-path . ../samples/ui-component.frame
```

Test highlighting:

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter highlight --grammar-path . --query-paths queries/highlights.scm --check ../samples/app.frame
npx tree-sitter highlight --grammar-path . --query-paths queries/highlights.scm --check ../samples/highlighting.frame
npx tree-sitter highlight --grammar-path . --query-paths queries/highlights.scm --check ../samples/ui-component.frame
```

Check that `dock`, `min-height`, `align`, `text`, `justify`, `surface`,
`background`, `border`, `transition`, `animation`, `#include`, percentages,
numeric values, hex colors, UI component keywords, and data references are styled.

## Examples

```frame
grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

card ProjectCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium

  hover {
    lift small
    glow accent
  }
}
```

Svelte usage:

```svelte
<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.Dashboard}>
  <aside class={ui.Sidebar}>Sidebar</aside>
  <main class={ui.Content}>Content</main>
</div>
```

## Svelte Style Blocks

Inline Svelte `<style lang="frame">` blocks compile through the Svelte
preprocessor, but this Zed extension registers `frame_lsp` only for `.frame`
files. That avoids conflicts with Svelte and CSS tooling in `.svelte` buffers.

For the best editor experience, keep shared Frame code in external `.frame`
files and import the generated CSS/TypeScript from Svelte.

## Troubleshooting

For install or server launch failures, start Zed from a terminal:

```bash
zed --foreground .
```

Then open a `.frame` file with an invalid declaration, duplicate declaration
name, or invalid area placement. Zed should show diagnostics from `frame_lsp`.

## Known Limitations

- The extension does not download or build `frame_lsp` automatically yet.
- Inline Svelte `<style lang="frame">` blocks compile, but full Zed LSP support
  is intentionally limited to `.frame` files.
- The generated parser is emitted with Tree-sitter ABI 14 for broader Zed
  compatibility.

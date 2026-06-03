# Frame Zed Extension

This directory contains local Zed language support for Frame.

## Current Support

- Associates `.frame` files with the Frame language.
- Uses Tree-sitter for parsing and syntax highlighting.
- Highlights declaration keywords, declaration names, state blocks, property words, values, and `//` comments.
- Registers `frame_lsp` as the Frame language server for diagnostics, completions, hover docs, and formatting.
- Registers `frame_lsp` only for `.frame` files. Svelte buffers keep their normal Svelte/CSS tooling.
- Includes samples for layout, cards, and state blocks.

Milestone 7 supports diagnostics, completions, hover docs, and document formatting. Code actions and rename are intentionally not implemented yet.

## Local Installation

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select this directory:

```txt
editors/zed
```

For LSP support, build the LSP binary:

```bash
cargo build -p frame_lsp
```

The extension first checks `FRAME_LSP`, then searches for `frame_lsp` on `PATH`. When developing locally, it also falls back to:

```txt
/Users/whitebread/projects/svelte/frame/target/debug/frame_lsp
```

If you use the extension from another project, either install `frame_lsp` on `PATH` or launch Zed from a shell with the Frame target directory available:

```bash
PATH="/Users/whitebread/projects/svelte/frame/target/debug:$PATH" zed .
```

You can also point directly at the binary:

```bash
FRAME_LSP="/Users/whitebread/projects/svelte/frame/target/debug/frame_lsp" zed .
```

The local manifest uses a filesystem grammar repository:

```toml
[grammars.frame]
repository = "file:///Users/whitebread/projects/svelte/frame/editors/zed/tree-sitter-frame"
rev = "main"
```

If this checkout moves, replace the repository value with the new absolute path:

```toml
repository = "file:///path/to/frame/editors/zed/tree-sitter-frame"
```

## Generate The Parser

```bash
cd editors/zed/tree-sitter-frame
npm install
npx tree-sitter generate --abi 14
```

## Test Parsing

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter parse --grammar-path . ../samples/app.frame
npx tree-sitter parse --grammar-path . ../samples/grid.frame
npx tree-sitter parse --grammar-path . ../samples/card.frame
npx tree-sitter parse --grammar-path . ../samples/states.frame
```

Malformed files should produce Tree-sitter `ERROR` nodes rather than crashing the parser. For a quick check:

```bash
cd editors/zed/tree-sitter-frame
printf 'card Broken {\n  hover {\n    lift small\n}\n' > /tmp/broken.frame
npx tree-sitter parse --grammar-path . /tmp/broken.frame
```

## Test Highlighting

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter highlight --grammar-path . --query-paths queries/highlights.scm --check ../samples/app.frame
```

In Zed, open `editors/zed/samples/app.frame` after installing the dev extension and confirm the buffer language is `Frame`.

For best editor support, put shared Frame code in `.frame` files. Inline Svelte `<style lang="frame">` blocks still compile through the Svelte preprocessor, but this extension does not attach `frame_lsp` to Svelte buffers because it conflicts with Svelte/CSS completions.

For install failures, launch Zed from a terminal with foreground logs:

```bash
zed --foreground .
```

## Test LSP Diagnostics

```bash
cargo build -p frame_lsp
zed .
```

Then open a `.frame` file with an invalid declaration, duplicate declaration name, or invalid area placement. Zed should show diagnostics published by `frame_lsp`.

## Known Limitations

- The grammar intentionally covers the current MVP language only.
- Statement values are parsed as bare identifiers; semantic validity still belongs to the Rust compiler.
- `hover`, `focus`, `active`, and `disabled` nested blocks are recognized.
- Inline Svelte `<style lang="frame">` blocks compile through the preprocessor, but Frame LSP support is intentionally limited to `.frame` files.
- The Zed extension does not download or build the server automatically yet.
- The generated parser is intentionally emitted with Tree-sitter ABI 14 for broader Zed compatibility.

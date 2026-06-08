# tree-sitter-frame

Tree-sitter grammar for Frame editor support.

## Generate

```bash
cd editors/zed/tree-sitter-frame
npm install
npx tree-sitter generate --abi 14
```

## Parse Samples

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter parse --grammar-path . ../samples/app.frame
npx tree-sitter parse --grammar-path . ../samples/grid.frame
npx tree-sitter parse --grammar-path . ../samples/card.frame
npx tree-sitter parse --grammar-path . ../samples/states.frame
```

## Validate Highlighting

The canonical validation command is:

```bash
cd editors/zed/tree-sitter-frame
tree-sitter generate
tree-sitter parse ../samples/highlighting.frame
tree-sitter highlight ../samples/highlighting.frame
```

Note: `tree-sitter highlight --check` expects a `test/highlight/` directory with expected highlight files, which this repository does not currently include.

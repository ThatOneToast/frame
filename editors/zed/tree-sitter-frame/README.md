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
npx tree-sitter parse --grammar-path . ../samples/inheritance.frame
npx tree-sitter parse --grammar-path . ../samples/component-ui.frame
npx tree-sitter parse --grammar-path . ../samples/loops.frame
npx tree-sitter parse --grammar-path . ../samples/conditionals.frame
npx tree-sitter parse --grammar-path . ../samples/bindings.frame
npx tree-sitter parse --grammar-path . ../samples/named-grid-tracks.frame
npx tree-sitter parse --grammar-path . ../samples/llm-dashboard-snippet.frame
```

## Validate Highlighting

```bash
cd editors/zed/tree-sitter-frame
npx tree-sitter highlight --grammar-path . ../samples/inheritance.frame
npx tree-sitter highlight --grammar-path . ../samples/component-ui.frame
npx tree-sitter highlight --grammar-path . ../samples/loops.frame
npx tree-sitter highlight --grammar-path . ../samples/conditionals.frame
npx tree-sitter highlight --grammar-path . ../samples/bindings.frame
npx tree-sitter highlight --grammar-path . ../samples/named-grid-tracks.frame
npx tree-sitter highlight --grammar-path . ../samples/llm-dashboard-snippet.frame
```

## All Samples

Parse and highlight all samples:

```bash
cd editors/zed/tree-sitter-frame
for f in ../samples/*.frame; do
  echo "=== $f ==="
  npx tree-sitter parse --grammar-path . "$f" 2>/dev/null | grep -c ERROR || echo "0 errors"
done
```

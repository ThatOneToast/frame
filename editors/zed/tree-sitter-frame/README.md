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

# Compiler Pipeline

This file expands the planned compiler pipeline for the Frame overhaul.

## Pipeline

```txt
source text
  -> lexical structure
  -> parsed AST
  -> semantic model
  -> lowered Frame IR
  -> generated outputs
```

## Parser

The parser should focus on syntax only.

Responsibilities:

- parse style declarations
- parse UI declarations
- parse state and prop declarations
- parse data references
- parse handler references
- parse event names and event filters
- preserve source spans
- preserve comments for formatting

The parser should not decide how the DOM runtime works.

## Semantic Model

The semantic model should resolve meaning.

Responsibilities:

- resolve component names
- resolve element names
- resolve style names
- resolve prop and state references
- resolve handler references
- validate DOM elements
- validate attributes where possible
- validate event names
- validate unsafe operations
- produce diagnostics for CLI and LSP

## Lowering to IR

Lowering should convert the AST and semantic data into a renderer-neutral shape.

The IR should not preserve every syntax detail. It should preserve behavior, source maps, and enough metadata for renderers and diagnostics.

## Outputs

Planned outputs:

- CSS for style declarations
- TypeScript contracts for props, state, and handlers
- serialized IR for the runtime
- static HTML where possible

## Compatibility

The existing CSS and TypeScript class export pipeline should remain usable during the transition.

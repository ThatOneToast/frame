# Diagnostics and Source Maps

Frame should treat diagnostics as part of the language design.

The compiler, CLI, LSP, generated contracts, and runtime should all be able to point back to useful Frame source locations.

## Diagnostic Sources

Diagnostics can come from:

- parser errors
- semantic validation
- style resolution
- DOM validation
- accessibility checks
- security checks
- code generation
- runtime development checks

## Source Mapping Goals

Source mappings should connect:

- AST nodes to source spans
- IR nodes to source spans
- generated CSS back to Frame declarations
- generated TypeScript contracts back to Frame references
- runtime errors back to Frame nodes where possible

## Teachable Diagnostics

Frame diagnostics should explain the issue and suggest the next action.

Example categories:

- missing style
- unknown DOM element
- invalid attribute
- missing handler
- missing state value
- unsafe HTML insertion
- inaccessible button or link
- form control without label

## LSP Use

The LSP should use the same diagnostic model as the CLI.

Editor features should include:

- hover explanations
- completions
- code actions
- go-to-definition
- references
- document symbols
- formatting

## Runtime Use

In development mode, the DOM runtime should report errors with Frame component and node names. Runtime errors should avoid exposing confusing generated internals when source information exists.

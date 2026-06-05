# Frame IR

Frame IR is the contract between the compiler and renderers.

Renderers should not parse `.frame` source. They should consume an IR produced by the compiler.

## Required Data

The IR needs to represent:

- documents
- components
- props
- state
- nodes
- text
- attributes
- DOM properties
- event bindings
- event filters
- style bindings
- reactive style rules
- conditions
- loops
- slots
- source mappings

## Example Shape

```json
{
  "version": 1,
  "components": {
    "ChatInput": {
      "state": {
        "draft": "text",
        "sending": "bool"
      },
      "nodes": [
        {
          "kind": "element",
          "tag": "button",
          "name": "Send",
          "style": "PrimaryButton",
          "events": [
            { "event": "click", "handler": "sendMessage" }
          ]
        }
      ]
    }
  }
}
```

## IR Rules

- Keep IR renderer-neutral.
- Keep source mappings for diagnostics.
- Version the IR format.
- Do not store raw script bodies.
- Store handler names and generated contract metadata.
- Store unsafe operations explicitly.

## Future Work

- [ ] Define Rust IR structs.
- [ ] Define serialized IR format.
- [ ] Add IR snapshot tests.
- [ ] Add source map tests.

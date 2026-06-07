# Semantic Lowering

Frame source stores semantic UI intent first. Renderer targets lower that intent into platform details.

The compiler does not treat `action`, `field`, `editor`, `composer`, `list`, or `data` as browser tags. It records:

- `semantic_kind`: the Frame primitive the author wrote
- `render_kind`: the default renderer mapping for the selected target
- `name`: stable node identity and automatic style lookup key
- bindings, conditions, handler references, and source spans
- attributes that represent Frame intent, such as `goto`, `source`, `label`, and `description`

## Default DOM Mapping

| Frame primitive | DOM default |
| --- | --- |
| `screen`, `panel`, `section`, `stack`, `row`, `grid`, `split`, `dock`, `overlay`, `scroll` | structural container |
| `action` | `button type="button"` |
| `link` | `a` with `goto` lowered to `href` |
| `menu` | navigation/container structure |
| `toolbar`, `tabs`, `field` | structural container with accessibility metadata |
| `input` | `input` |
| `editor`, `composer` | `textarea` |
| `toggle` | `input type="checkbox"` |
| `choice`, `select` | `select` for Phase 1 |
| `title` | heading-like text |
| `text`, `label`, `badge`, `icon` | inline text-like node |
| `avatar`, `image` | image-like node with alternate text requirements |
| `list`, `feed` | repeated content container |
| `data` | structured data container |
| `card`, `popover` | grouped content container |
| `dialog` | dialog-like surface |

## Handler Lowering

Frame supports intent events and handler properties:

```frame
action Save {
  on press @save
}

composer ChatBox {
  draft bind $draft
  send @sendMessage
}
```

Both forms generate TypeScript handler contracts. The DOM runtime maps `press` to the target action event. Custom renderers may map the same semantic event to native command dispatch.

## Property Lowering

Frame properties are not raw DOM attributes:

- `goto` is navigation intent and may lower to `href` on DOM.
- `source` is media or data source intent and may lower to `src` only for media-like DOM nodes.
- `label`, `description`, and `hint` are accessibility metadata.
- `bind $state` is shorthand for the primary value binding of an input-like primitive.
- `draft bind $state` records composer draft binding.
- `send @handler` records composer send intent.

Raw HTML attributes such as `href`, `target`, `rel`, `role`, and `aria-label` are not accepted as author-facing UI syntax in Phase 1.

## Renderer Overrides

Renderers may override `render_kind` as long as they preserve:

- accessible names and descriptions
- required state, disabled state, selected/checked/value state
- handler contracts
- binding contracts
- source spans for diagnostics
- security checks for URL-like destinations

If a renderer cannot preserve the semantic contract, it should emit a capability diagnostic instead of silently degrading behavior.

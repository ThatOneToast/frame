# Security Architecture

Frame assumes renderer output is hostile unless proven safe. The compiler and runtime preserve intent; renderers enforce target-specific sinks.

## Text Escaping

Text interpolation escapes by default:

```frame
text $username
```

Renderers must treat this as text content, not executable markup.

## URL Validation

URL-bearing attributes are security-sensitive:

- `href`
- `src`
- `srcset`
- `poster`
- `action`
- `formaction`

Semantic validation warns when these appear and errors for literal `javascript:` values. The DOM renderer rejects `javascript:` URL values at runtime before writing to a URL sink. Allowed URL forms are http, https, mailto, tel, relative paths, root-relative paths, fragments, and other non-script schemes explicitly accepted by a future policy.

Scheduled patches run the same URL validation as initial mounting, so a state update cannot turn a safe `href` or `src` into a `javascript:` sink.

`target "_blank"` should include `rel "noopener"` or `rel "noreferrer"`. The compiler emits a warning when the safe relationship is missing.

## Unsafe HTML

The following attributes are rejected by semantic validation:

- `html`
- `innerHTML`
- `outerHTML`

Raw HTML requires a future explicit unsafe capability. It must not be smuggled through ordinary attributes.

## Unsafe Attributes

Frame event syntax uses typed handler references:

```frame
on press @submit
```

Inline event attributes such as JavaScript source bodies are not part of Frame UI syntax. Properties that look like inline event attributes, such as `onclick`, are semantic errors.

## XSS Prevention

- Text is escaped by default.
- URL sinks are identified in diagnostics.
- Raw HTML sinks are errors.
- Event handlers are references, not inline code.
- Handler failures and missing handlers are captured as runtime diagnostics with component/source context.
- Renderers own final target-specific escaping and validation.

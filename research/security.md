# Security Model

Frame should be safe by default.

## Defaults

- Text inserted through `$value` is escaped.
- Event handlers are references, not inline code.
- Raw HTML requires explicit unsafe syntax.
- URL attributes need validation or classification.
- Desktop API access must stay outside Frame UI declarations.

## Data Insertion

Safe text:

```frame
text $username
```

Unsafe HTML should require a form that is hard to use accidentally.

```frame
unsafe html $trustedMarkup
```

This should produce diagnostics unless the project explicitly allows it.

## Handler Boundary

Frame references handlers:

```frame
on click @logout
```

The handler body lives in TypeScript, JavaScript, Rust, or another target language depending on the renderer.

## URL Attributes

The compiler and runtime should treat URL-bearing attributes carefully:

- `href`
- `src`
- `srcset`
- `action`
- `formaction`
- `poster`
- `cite`

JavaScript URLs should be rejected by default.

## Desktop Boundary

For Tauri and desktop targets, Frame should not directly expose privileged APIs in UI syntax. Desktop commands should be called from external handler code.

## Required Diagnostics

- raw HTML usage
- missing accessible names
- unsafe external links
- iframe without sandbox policy
- suspicious URL protocol
- form action policy warnings

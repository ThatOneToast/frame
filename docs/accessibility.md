# Accessibility Architecture

Frame records accessibility intent in compiler IR and validates common omissions before renderer work begins.

Runtime hardening does not add new accessibility semantics. The DOM renderer preserves accessibility attributes during scheduled patches and cleanup, and runtime diagnostics include source context when accessibility-relevant URL or attribute patches fail.

## Labels

Interactive or content-bearing elements should expose text or a label. Semantic validation warns when `button`, `input`, `img`/`image`, `a`/`link`, `table`, or `dialog` misses common accessible metadata.

Supported label metadata:

- `text "Label"`
- `text $label`
- `aria-label "Label"`
- `aria-labelledby "existing-id"`
- `label "Label"`
- `alt "Description"`

## ARIA Support

ARIA attributes are renderer metadata. Frame preserves `aria-*` attributes in IR and the DOM runtime applies them as attributes. Each non-DOM renderer is responsible for mapping those attributes into the target accessibility system.

## Alt Requirements

`img` and `image` nodes require `alt`, or `aria-hidden true` when decorative. Missing alt text is a semantic warning.

## Role Support

`role` is treated as semantic metadata. Renderers may reject unsupported roles for their target.

## Diagnostics

Current semantic diagnostics include:

- warning for image/button/input/link without accessible text or label
- warning for links without `href`
- warning for skipped heading-order risks
- warning for tables without `caption` or `th`
- warning for dialogs without an accessible name
- warning for URL-bearing attributes that need target validation
- error for unsafe HTML injection sinks

Future diagnostics should add full role validation, aria attribute spelling, element-role compatibility, dialog focus-management guidance, and keyboard interaction rules.

# Accessibility Model

Frame should make common UI accessible by default. Authors should not need to know ARIA attributes, DOM roles, or ID wiring for ordinary controls, navigation, dialogs, tabs, forms, lists, and data displays.

The canonical language registry in `crates/frame_core/src/language.rs` defines every primitive and property. LSP completions, hover docs, and diagnostics consume the registry to teach accessible defaults.

ARIA and roles are not part of Phase 1 author-facing UI syntax. Renderers may generate ARIA and roles internally from Frame semantics.

## Principles

- Accessibility intent is stored in Frame IR.
- Semantic primitives generate accessibility requirements.
- The compiler reports missing names, labels, descriptions, and relationships.
- Renderers map Frame accessibility metadata into their target platform.
- DOM-specific ARIA output is an implementation detail of the DOM renderer.
- Raw overrides are explicit and validated where practical.

## Names and Descriptions

Frame should derive accessible names from:

1. `label`
2. visible `text`
3. `heading`
4. primitive name when safe and human-readable
5. renderer-generated accessibility metadata

Descriptions should come from:

- `description`
- `hint`
- validation messages
- associated helper text

Example:

```frame
input Email {
  label "Email"
  hint "Use your work email address"
  value bind $email
}
```

The compiler stores `name = "Email"` and `description = "Use your work email address"` in IR. The DOM renderer can create IDs and relationships without exposing them in source.

## Primitive Behavior

### Actions

`action` receives command semantics, keyboard activation, focus behavior, disabled state, and accessible name requirements.

### Links

`link` receives navigation semantics and requires destination plus name. External destinations should get safe target/relationship behavior by default.

### Inputs

`input`, `editor`, `toggle`, `choice`, and `select` require labels, expose value/checked/selected state, and link validation messages automatically.

### Dialogs

`dialog` requires name, modal/non-modal intent, focus entry, dismissal behavior, and background interaction policy. DOM focus trapping is a renderer responsibility, but Frame IR must declare the requirement.

### Tabs

`tabs` creates tablist, tab, and panel relationships. Keyboard behavior should be part of the primitive contract.

### Menus and Toolbars

`menu` and `toolbar` require labels when ambiguity exists. Menu semantics should only be used for real menu behavior, not every navigation list.

### Data and Tables

`data` and `table` require captions or labels and field/header information. Authors should not manually wire `th`, `scope`, `headers`, `tr`, or `td` for common cases.

## Removed Browser Accessibility Syntax

Phase 1 rejects common browser accessibility attributes in author-facing UI:

- `role`
- `aria-label`
- `aria-labelledby`
- `aria-describedby`
- `aria-hidden`

Use Frame semantics instead:

- `label`
- `title`
- `description`
- `hint`
- `decorative true`

## Compiler Model

The semantic model should collect:

- primitive kind
- accessible name source
- description source
- role intent
- state such as disabled, required, invalid, expanded, selected, checked
- relationships such as label/control, tab/panel, dialog/title, table/header
- focus requirements
- keyboard interaction requirements
- live region intent

The IR should preserve this data independently from DOM attributes.

## DOM Mapping

The DOM renderer may emit:

- semantic elements where correct
- generated IDs for relationships
- `aria-*` attributes where native HTML is insufficient
- `role` only when native semantics do not cover the primitive
- safe `rel` defaults for external links
- focus management hooks for dialogs and overlays

The renderer should not require authors to provide IDs for common relationships.

## Diagnostics

Required diagnostics:

- interactive primitive without accessible name
- form control without label
- image/media without alternative text or decorative intent
- dialog without name or dismissal/focus policy
- tabs without panels
- table/data without label, caption, fields, or headers
- ambiguous menu/toolbar labels
- custom interactive primitive without keyboard behavior
- raw `role` that conflicts with primitive semantics
- ARIA spelling or value errors where practical

## LSP Teaching

Hover text should explain what the primitive means:

- `action`: "A user command. Frame maps this to an accessible control for the renderer."
- `editor`: "Multi-line text entry. Requires a label and exposes a text value."
- `dialog`: "A focused layer for a decision or interruption. Requires a name and focus behavior."
- `table`: "Structured comparison data. Frame records headers and relationships for accessibility."

Completions should prefer `label`, `hint`, `description`, `required`, `invalid`, `expanded`, and `selected` before raw ARIA attributes.

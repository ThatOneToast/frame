# Forms

Frame forms should be built around the user's task and the data contract, not raw DOM controls.

The canonical language registry in `crates/frame_core/src/language.rs` defines all form primitives, properties, and values. LSP completions, hover docs, and diagnostics consume the registry.

The DOM renderer can map controls to `input`, `textarea`, `select`, `button`, and form elements. Frame source should first describe what value is being collected and what action is being submitted.

## Principles

- Every control has an accessible name by default.
- Bindings are explicit and typed.
- Validation intent is part of the control, not only an attribute.
- Common controls do not require authors to know DOM input types.
- Form submission references external handlers with `@handler`.
- Inline JavaScript remains unsupported.

## Proposed Controls

### `input`

Single-line value entry.

```frame
input Name {
  label "Name"
  value bind $name
  required
}
```

Kinds:

```frame
input Email {
  kind email
  label "Email"
  value bind $email
}

input Search {
  kind search
  label "Search"
  value bind $query
}
```

Default DOM mapping: `input` with safe type mapping.

### `field`

Labeled control group.

```frame
field EmailField {
  label "Email"
  hint "Use the address where updates should arrive."

  input EmailInput {
    value bind $email
  }
}
```

Default DOM mapping: neutral container. Renderers should connect labels, hints, descriptions, and validation metadata to the contained control when supported.

### `editor`

Multi-line text entry.

```frame
editor Message {
  label "Message"
  value bind $message
  min-lines 4
}
```

Default DOM mapping: `textarea`.

### `toggle`

Binary setting.

```frame
toggle Enabled {
  label "Enabled"
  checked bind $enabled
}
```

Default DOM mapping: checkbox. A renderer may display a switch if it preserves semantics.

### `choice`

Small fixed option set.

```frame
choice Theme {
  label "Theme"
  options $themes
  selected bind $theme
}
```

Modes:

```frame
choice Density {
  mode segmented
  options "Compact", "Comfortable"
  selected bind $density
}
```

Default DOM mapping: radio group or select depending on mode and option count.

### `select`

Larger or dynamic option set.

```frame
select Country {
  label "Country"
  options $countries
  selected bind $country
}
```

Default DOM mapping: `select`.

### `action`

Form command.

```frame
action Save {
  label "Save"
  submit
  on press @save
}
```

Default DOM mapping: `button`. `submit` maps to `type="submit"` in DOM forms.

## Composer

```frame
composer Chat {
  label "Message"
  draft bind $draft
  send @send
}
```

`composer` replaces raw form submission for message-like input workflows. The compiler records draft binding and send handler intent; renderers choose the underlying controls.

## Validation

Validation should be explicit:

```frame
input Email {
  label "Email"
  kind email
  value bind $email
  required
  invalid when $emailInvalid {
    message "Enter a valid email address"
  }
}
```

Compiler behavior:

- record required fields in IR
- record validation dependencies
- require validation messages for custom invalid states
- generate handler contracts for submit events
- keep browser-native validation mapping as a DOM renderer detail

## Accessibility Behavior

The compiler should derive:

- accessible name from `label`
- description from `hint` or `description`
- error relationship from `invalid message`
- required state from `required`
- disabled state from `disabled when`
- value binding type from state/prop declarations

Authors should not need `aria-label`, `aria-describedby`, `aria-invalid`, or explicit `for`/`id` wiring for common form controls.

Advanced ARIA remains available when a control cannot be represented by the common model.

## DOM Mapping

| Frame | Default DOM |
| --- | --- |
| `input kind text` | `input type="text"` |
| `input kind email` | `input type="email"` |
| `input kind search` | `input type="search"` |
| `editor` | `textarea` |
| `toggle` | `input type="checkbox"` |
| `choice mode radio` | `fieldset` plus radio inputs |
| `choice mode segmented` | button/radio group pattern |
| `select` | `select` |
| `action submit` | `button type="submit"` |

Mappings should be overridable by custom renderers only when the accessibility and data contracts are preserved.

## Diagnostics

The compiler and LSP should report:

- control without label
- bound value whose state type is incompatible with the control
- `choice` without options
- dynamic options without stable labels/values
- submit action without form handler when no parent handles submit
- custom invalid state without message
- raw DOM form attributes when an intent-level form API exists

# UI Primitives

Frame UI primitives describe interface intent before renderer mechanics. Each primitive should lower to Frame IR with a semantic kind, name, style binding, accessibility metadata, state dependencies, and event contracts.

The DOM mappings below are defaults. Custom renderers may provide native widgets or different DOM structures as long as they preserve the primitive contract.

## Primitive Catalog

### `action`

Meaning: a user-triggered command.

Default DOM mapping: `button type="button"` unless used as a submit action inside a form.

Accessibility: receives button semantics, keyboard activation, disabled state, accessible name from `label`, `text`, or node name.

Events: prefer `on press @handler`; DOM maps this to click plus keyboard activation where needed.

```frame
action Save {
  label "Save changes"
  on press @save
}
```

### `link`

Meaning: navigation to a resource, route, file, or external location.

Default DOM mapping: `a`.

Accessibility: requires a label and destination. External targets should imply safe `rel` behavior unless explicitly overridden.

```frame
link Docs {
  goto "/docs"
}
```

### `menu`

Meaning: a set of navigation or commands.

Default DOM mapping: `nav` for navigation menus, button/list structure for command menus.

Accessibility: named region for navigation menus; menu semantics only when behavior matches actual menu interaction.

```frame
menu MainNav {
  label "Main navigation"
  link Docs { goto "/docs" }
  link Examples { goto "/examples" }
}
```

### `toolbar`

Meaning: compact group of related actions.

Default DOM mapping: container with toolbar semantics when actions are command-like.

Accessibility: requires a label when more than one toolbar exists in a view.

```frame
toolbar EditorTools {
  action Bold { on press @bold }
  action Italic { on press @italic }
}
```

### `tabs`

Meaning: switch between related panels.

Default DOM mapping: tablist, tabs, and tab panels.

Accessibility: generates keyboard behavior requirements and panel relationships in IR.

```frame
tabs ProfileTabs {
  tab Overview { panel OverviewPanel }
  tab Security { panel SecurityPanel }
}
```

### `sidebar`

Meaning: secondary persistent app region.

Default DOM mapping: `aside` or structural container.

Accessibility: named complementary region when it contains navigation or secondary content.

### `panel`

Meaning: named region of interface content.

Default DOM mapping: `section` when named, otherwise `div`.

Accessibility: may become a region when it has a heading or label.

### `card`

Meaning: grouped content or an object preview.

Default DOM mapping: article-like or generic container depending on content.

Accessibility: should expose a title when interactive or repeated.

### `dialog`

Meaning: modal or non-modal interruption requiring attention.

Default DOM mapping: `dialog` where supported or accessible dialog structure.

Accessibility: requires name, focus plan, dismissal behavior, and inert background behavior for modal dialogs.

### `input`

Meaning: single-value text-like input.

Default DOM mapping: `input`.

Accessibility: requires label. Type is semantic, such as `email`, `search`, `password`, `number`, not raw DOM-only unless needed.

### `field`

Meaning: a labeled data-entry group with help, description, validation, and one or more controls.

Default DOM mapping: neutral container. Renderers may add label/control relationships from `label`, `hint`, `description`, and child controls.

Accessibility: should provide a label and connect helper/error text to the contained control when renderer support exists.

```frame
field EmailField {
  label "Email"
  hint "Use the address where updates should arrive."

  input EmailInput {
    value bind $email
  }
}
```

### `editor`

Meaning: multi-line text editing.

Default DOM mapping: `textarea`.

Accessibility: requires label. May include formatting toolbar relationships later.

### `toggle`

Meaning: binary setting.

Default DOM mapping: checkbox input or switch pattern, depending on renderer policy.

Accessibility: exposes checked state and label.

### `choice`

Meaning: choose one or more options from a small set.

Default DOM mapping: radio group, checkbox group, segmented control, or listbox depending on mode and renderer.

Accessibility: group label, option labels, selected state, keyboard navigation.

### `select`

Meaning: choose from a larger or externally provided option set.

Default DOM mapping: `select` by default; custom listbox only when behavior is implemented.

Accessibility: label, selected state, option names.

### `list`

Meaning: ordered or unordered repeated items where row/cell semantics are not primary.

Default DOM mapping: `ul`, `ol`, or generic repeated containers.

Accessibility: list semantics when useful; supports item labels and selection state.

### `feed`

Meaning: chronological or activity stream content.

Default DOM mapping: `section`/`article` sequence.

Accessibility: supports article names, timestamps, update announcements, and stable item identity.

### `data`

Meaning: structured record or data set presentation.

Default DOM mapping: description list, table, grid, or custom structure based on declared fields.

Accessibility: exposes field names and values without requiring `tr`/`td`.

### `stack`

Meaning: ordered layout along one direction.

Default DOM mapping: block/flex/grid implementation chosen by renderer.

Accessibility: no implicit landmark.

### `dock`

Meaning: fixed edge or app chrome region.

Default DOM mapping: positioned region with renderer-managed edge placement.

Accessibility: landmark depends on content, such as navigation or toolbar.

### `grid`

Meaning: two-dimensional arrangement.

Default DOM mapping: CSS grid for DOM. Other renderers may use native grid layout.

Accessibility: not a data grid unless declared as `data` or interactive grid.

### `scroll`

Meaning: bounded scrollable region.

Default DOM mapping: overflow container with focus and scroll restoration considerations.

Accessibility: named region when independently scrollable and important.

## Compiler Mapping

The compiler should lower primitives into IR like:

```txt
UiNode {
  primitive: Action,
  semantic_kind: action,
  render_kind: button,
  name: Save,
  style: inferred or explicit,
  accessibility: { name, description, role_intent },
  events: [Press -> save],
  renderer_mapping: default_button
}
```

The IR should preserve the primitive even when a renderer maps it to HTML. This lets LSP, diagnostics, tests, and non-DOM renderers reason about author intent.

## Custom Renderer Overrides

Renderer targets may override mappings through a capability table:

- `action` may map to native button, web button, menu item, or toolbar button
- `editor` may map to textarea, native multi-line text field, or rich text editor
- `dialog` may map to platform modal APIs
- `table` may map to native table widgets

Overrides must preserve required accessibility, state, event, and validation contracts or emit diagnostics.

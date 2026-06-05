# TODO-DOM.md

Frame's UI layer should be able to represent the full capability of HTML and the DOM while staying concise, typed, and understandable.

Do not mark an item complete until Frame syntax, AST/IR representation, semantic validation, runtime behavior, docs, and tests exist.

## Design Goals

- Frame should be able to target the browser DOM 1:1 where practical.
- Frame syntax should be more readable than raw HTML for common UI work.
- Every DOM feature should either be represented, intentionally deferred, or explicitly unsafe.
- Scripting stays outside Frame and is linked through `@handler` references.
- Data is referenced through `$value` and escaped by default.

## Core Runtime Capabilities

- [ ] Mount a Frame component into a DOM container.
- [ ] Unmount and clean up listeners/effects.
- [ ] Create elements.
- [ ] Create text nodes.
- [ ] Create comments where needed for anchors.
- [ ] Set attributes.
- [ ] Set DOM properties.
- [ ] Remove attributes/properties.
- [ ] Apply classes.
- [ ] Apply reactive classes/styles.
- [ ] Attach event listeners.
- [ ] Remove event listeners.
- [ ] Apply patches deterministically.
- [ ] Batch updates.
- [ ] Preserve source maps for runtime diagnostics.

## Frame UI Syntax

- [ ] `component Name { ... }`
- [ ] `props { ... }`
- [ ] `state { ... }`
- [ ] `view { ... }`
- [ ] `slot Default { ... }`
- [ ] `button Send { ... }`
- [ ] `button Send:PrimaryButton { ... }`
- [ ] `text "literal"`
- [ ] `text $value`
- [ ] `on click @handler`
- [ ] `on keydown.enter @handler`
- [ ] `show when $condition`
- [ ] `disabled when $condition`
- [ ] `style when $condition = StyleName`
- [ ] `repeat item in $items keyed item.id { ... }`
- [ ] `if $condition { ... } else { ... }`

## HTML Document Structure

- [ ] `html`
- [ ] `head`
- [ ] `body`
- [ ] `title`
- [ ] `base`
- [ ] `link`
- [ ] `meta`
- [ ] `style`
- [ ] `script` as external asset only
- [ ] document language handling
- [ ] viewport metadata helpers
- [ ] preload/preconnect helpers

## Sectioning and Text Elements

- [ ] `main`
- [ ] `section`
- [ ] `nav`
- [ ] `article`
- [ ] `aside`
- [ ] `header`
- [ ] `footer`
- [ ] `address`
- [ ] `h1` through `h6`
- [ ] `p`
- [ ] `span`
- [ ] `div`
- [ ] `blockquote`
- [ ] `pre`
- [ ] `code`
- [ ] `em`
- [ ] `strong`
- [ ] `small`
- [ ] `mark`
- [ ] `abbr`
- [ ] `time`
- [ ] `br`
- [ ] `wbr`

## Lists

- [ ] `ul`
- [ ] `ol`
- [ ] `li`
- [ ] `dl`
- [ ] `dt`
- [ ] `dd`

## Links and Navigation

- [ ] `a`
- [ ] `href` URL validation/classification
- [ ] `target`
- [ ] `rel`
- [ ] download behavior
- [ ] router/link abstraction research

## Media

- [ ] `img`
- [ ] `picture`
- [ ] `source`
- [ ] `video`
- [ ] `audio`
- [ ] `track`
- [ ] `canvas`
- [ ] `svg`
- [ ] `math`
- [ ] lazy loading helpers
- [ ] responsive image helpers
- [ ] alt text diagnostics

## Embedded Content

- [ ] `iframe`
- [ ] sandbox policy helpers
- [ ] `object`
- [ ] `embed`
- [ ] `portal` research if relevant

## Tables

- [ ] `table`
- [ ] `caption`
- [ ] `thead`
- [ ] `tbody`
- [ ] `tfoot`
- [ ] `tr`
- [ ] `th`
- [ ] `td`
- [ ] `colgroup`
- [ ] `col`
- [ ] table accessibility diagnostics

## Forms

- [ ] `form`
- [ ] `label`
- [ ] `input`
- [ ] `button`
- [ ] `select`
- [ ] `option`
- [ ] `optgroup`
- [ ] `textarea`
- [ ] `fieldset`
- [ ] `legend`
- [ ] `datalist`
- [ ] `output`
- [ ] `progress`
- [ ] `meter`
- [ ] form submission handling
- [ ] validation state
- [ ] `bind value`
- [ ] `bind checked`
- [ ] `bind files`
- [ ] `bind selected`
- [ ] `bind group`

## Interactive Elements

- [ ] `details`
- [ ] `summary`
- [ ] `dialog`
- [ ] popover API research
- [ ] focus management helpers
- [ ] inert handling

## Global Attributes

- [ ] `id`
- [ ] `class`
- [ ] `style` policy
- [ ] `title`
- [ ] `hidden`
- [ ] `tabindex`
- [ ] `role`
- [ ] `part`
- [ ] `slot`
- [ ] `contenteditable`
- [ ] `draggable`
- [ ] `spellcheck`
- [ ] `translate`
- [ ] `dir`
- [ ] `lang`
- [ ] `data-*`
- [ ] `aria-*`

## Events

- [ ] mouse events
- [ ] pointer events
- [ ] keyboard events
- [ ] input/change events
- [ ] form submit/reset events
- [ ] focus/blur events
- [ ] drag/drop events
- [ ] clipboard events
- [ ] composition events
- [ ] animation events
- [ ] transition events
- [ ] media events
- [ ] scroll events
- [ ] beforeinput
- [ ] custom events
- [ ] event modifiers: `prevent`, `stop`, `once`, `capture`, `passive`
- [ ] key filters: `enter`, `escape`, `tab`, arrows, modifiers

## Accessibility

- [ ] semantic element diagnostics
- [ ] alt text diagnostics
- [ ] label/input association diagnostics
- [ ] button accessible name diagnostics
- [ ] link accessible name diagnostics
- [ ] heading order diagnostics
- [ ] dialog focus diagnostics
- [ ] keyboard interaction diagnostics
- [ ] ARIA role validation
- [ ] ARIA attribute validation
- [ ] live region helpers

## Security-Sensitive DOM Areas

- [ ] raw HTML insertion marked unsafe
- [ ] URL attribute validation
- [ ] JavaScript URL rejection by default
- [ ] iframe sandbox diagnostics
- [ ] external link `rel` diagnostics
- [ ] CSP documentation
- [ ] script execution policy
- [ ] Trusted Types research
- [ ] sanitizer integration research

## Web Components and Custom Elements

- [ ] custom element syntax
- [ ] dashed tag validation
- [ ] property vs attribute binding
- [ ] custom events
- [ ] slots
- [ ] shadow DOM research
- [ ] `part` and `exportparts`

## Browser APIs and Boundaries

Frame should not directly become a scripting language for browser APIs. It should expose typed integration boundaries.

- [ ] timers through handlers only
- [ ] fetch through handlers/services only
- [ ] local storage through handlers/services only
- [ ] media devices through handlers/services only
- [ ] file system access through handlers/services only
- [ ] WebSocket/WebRTC through handlers/services only
- [ ] Tauri commands through handlers/services only

## Documentation Requirements

Each supported DOM feature needs:

- [ ] Frame syntax
- [ ] IR representation
- [ ] DOM runtime behavior
- [ ] accessibility notes
- [ ] security notes if relevant
- [ ] LSP completion/hover behavior
- [ ] tests

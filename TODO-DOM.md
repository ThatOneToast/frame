# TODO-DOM.md

Frame's UI layer should be able to represent the full capability of HTML and the DOM while staying concise, typed, and understandable.

Do not mark an item complete until Frame syntax, AST/IR representation, semantic validation, runtime behavior, docs, and tests exist.

Current status: UI syntax including `component`, `props`, `state`, `view`, `slot`, semantic primitives, text, events, data references, handler references, bindings, conditional rendering, conditional style, loop metadata, and component invocation is parsed, validated, lowered to Frame IR, and can generate TypeScript contracts. `packages/runtime-dom` implements a Phase 4 browser renderer with scheduled dependency-aware patching, hardened list reconciliation, practical HTML coverage, forms, global attributes, URL safety checks, cleanup counters, and accessibility/security diagnostics. The standalone web app workflow now generates typed IR, contracts, append-only handler skeletons, and a Vite-compatible app without Svelte or React. SSR, hydration, routing, portals, suspense, async components, and transition/animation runtimes remain open.

## Design Goals

- Frame should be able to target the browser DOM 1:1 where practical.
- Frame syntax should be more readable than raw HTML for common UI work.
- Every DOM feature should either be represented, intentionally deferred, or explicitly unsafe.
- Scripting stays outside Frame and is linked through `@handler` references.
- Data is referenced through `$value` and escaped by default.

## Core Runtime Capabilities

- [x] Mount a Frame component into a DOM container.
- [x] Unmount and clean up listeners/effects.
- [x] Create elements.
- [x] Create text nodes.
- [x] Create comments where needed for anchors.
- [x] Set attributes.
- [x] Set DOM properties.
- [x] Remove attributes/properties.
- [x] Apply classes.
- [x] Apply reactive classes/styles.
- [x] Attach event listeners.
- [x] Remove event listeners.
- [x] Apply patches deterministically.
- [x] Batch updates.
- [x] Preserve source maps for runtime diagnostics.
- [x] Mount-time handler validation (debug warnings).
- [x] Mount-time prop type validation.
- [x] Debug output for queued and flushed patches.

## Frame UI Syntax

- [x] `component Name { ... }`
- [x] `props { ... }`
- [x] `state { ... }`
- [x] `view { ... }`
- [ ] `slot Default { ... }`
- [x] `screen App { ... }`
- [x] `panel Sidebar { ... }`
- [x] `stack Content { ... }`
- [x] `row Toolbar { ... }`
- [x] `field Email { input EmailInput { ... } }`
- [x] `action Send { ... }`
- [x] `action Send:PrimaryButton { ... }`
- [x] `text "literal"`
- [x] `text $value`
- [x] `on press @handler`
- [x] `on keydown.enter @handler`
- [x] `show when $condition`
- [x] `disabled when $condition`
- [x] `style when $condition = StyleName`
- [x] `style StyleName when $condition`
- [x] `for item in $items key $item.id { ... }`
- [ ] `if $condition { ... } else { ... }`

Browser element spellings such as `button` and `div` are retained only as migration diagnostics and lowering targets, not primary author-facing syntax.

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

- [x] `main`
- [x] `section`
- [x] `nav`
- [x] `article`
- [ ] `aside`
- [x] `header`
- [x] `footer`
- [ ] `address`
- [x] `h1` through `h6`
- [x] `p`
- [x] `span`
- [x] `div`
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

- [x] `ul`
- [x] `ol`
- [x] `li`
- [x] `dl`
- [x] `dt`
- [x] `dd`

## Links and Navigation

- [x] `a`
- [x] `href` URL validation/classification
- [x] `target`
- [x] `rel`
- [x] download behavior
- [ ] router/link abstraction research

## Media

- [x] `img`
- [x] `picture`
- [x] `source`
- [x] `video`
- [x] `audio`
- [x] `track`
- [x] `canvas`
- [x] `svg`
- [ ] `math`
- [ ] lazy loading helpers
- [ ] responsive image helpers
- [x] alt text diagnostics

## Embedded Content

- [ ] `iframe`
- [ ] sandbox policy helpers
- [ ] `object`
- [ ] `embed`
- [ ] `portal` research if relevant

## Tables

- [x] `table`
- [x] `caption`
- [x] `thead`
- [x] `tbody`
- [x] `tfoot`
- [x] `tr`
- [x] `th`
- [x] `td`
- [x] `colgroup`
- [x] `col`
- [x] table accessibility diagnostics

## Forms

- [x] `form`
- [x] `label`
- [x] `input`
- [x] `button`
- [x] `select`
- [x] `option`
- [x] `optgroup`
- [x] `textarea`
- [x] `fieldset`
- [x] `legend`
- [ ] `datalist`
- [x] `output`
- [x] `progress`
- [x] `meter`
- [x] form submission handling
- [ ] validation state
- [x] `bind value`
- [x] `bind checked`
- [ ] `bind files`
- [x] `bind selected`
- [ ] `bind group`

## Interactive Elements

- [x] `details`
- [x] `summary`
- [x] `dialog`
- [ ] popover API research
- [ ] focus management helpers
- [ ] inert handling

## Global Attributes

- [x] `id`
- [x] `class`
- [ ] `style` policy
- [x] `title`
- [x] `hidden`
- [x] `tabindex`
- [x] `role`
- [x] `part`
- [x] `slot`
- [x] `contenteditable`
- [x] `draggable`
- [x] `spellcheck`
- [x] `translate`
- [x] `dir`
- [x] `lang`
- [x] `data-*`
- [x] `aria-*`

## Events

- [x] mouse events
- [ ] pointer events
- [x] keyboard events
- [x] input/change events
- [x] form submit/reset events
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
- [x] event modifiers: `prevent`, `stop`, `once`, `capture`, `passive`
- [x] key filters: `enter`, `escape`, `tab`, arrows, modifiers
- [x] generated event-specific handler type aliases (FramePressEvent, FrameInputEvent, FrameKeyboardEvent, FrameFormEvent)

## Accessibility

- [x] semantic element diagnostics
- [x] alt text diagnostics
- [x] label/input association diagnostics
- [x] button accessible name diagnostics
- [x] link accessible name diagnostics
- [x] heading order diagnostics
- [x] runtime accessibility defaults for semantic primitives (action, toggle, image, avatar, icon, field, media, composer)
- [x] keyboard activation for action-like controls
- [ ] dialog focus diagnostics
- [ ] keyboard interaction diagnostics
- [ ] ARIA role validation
- [ ] ARIA attribute validation
- [ ] live region helpers

## Security-Sensitive DOM Areas

- [x] raw HTML insertion marked unsafe
- [x] URL attribute validation
- [x] JavaScript URL rejection by default
- [ ] iframe sandbox diagnostics
- [x] external link `rel` diagnostics
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

## Standalone Web App Workflow

- [x] `frame new --template web` creates a Vite app using `src/app.frame`.
- [x] `frame init web` can scaffold an empty directory.
- [x] `npm run dev` regenerates Frame output before Vite starts.
- [x] `npm run build` regenerates Frame output before Vite builds.
- [x] Generated typed IR imports `defineFrameIrDocument(... as const)`.
- [x] Generated contracts include event-specific handler aliases.
- [x] Handler skeletons are generated non-destructively and append missing stubs.
- [x] User-owned handler implementations live outside `src/generated`.
- [x] CLI tests cover template scripts, output paths, and repeated build stability.

## Documentation Requirements

Each supported DOM feature needs:

- [ ] Frame syntax
- [ ] IR representation
- [ ] DOM runtime behavior
- [ ] accessibility notes
- [ ] security notes if relevant
- [ ] LSP completion/hover behavior
- [ ] tests

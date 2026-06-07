# Frame Runtime Architecture

Frame runtime is the renderer-neutral layer between Frame IR and concrete output targets.

```txt
.frame source
  -> parser
  -> semantic model
  -> Frame IR
  -> frame_runtime model
  -> renderer target
       DOM, SSR, static HTML, WebView, native, game/UI experiments
```

## Compiler Responsibilities

- Parse Frame syntax into AST.
- Validate props, state, slots, events, conditions, bindings, accessibility, and security-sensitive sinks.
- Lower components into Frame IR.
- Preserve source spans for diagnostics and LSP.
- Emit TypeScript contracts and serialized IR.
- Never target a concrete renderer as the internal model.

## Runtime Responsibilities

- Represent component identity, props, state, handlers, slots, conditions, lists, and lifecycle metadata.
- Track state reads, writes, dirty state, dependencies, subscriptions, and batches.
- Describe event dispatch metadata without binding concrete events.
- Describe list update behavior as keyed reuse or positional updates.
- Expose renderer abstraction traits.
- Avoid creating elements, mounting, patching, hydration, or browser/native API calls.

## Renderer Responsibilities

- Consume Frame IR and runtime metadata.
- Provide concrete node handles for its target.
- Validate renderer capability support before output.
- Realize element, text, event, slot, conditional, and list behavior.
- Own target-specific escaping, URL handling, accessibility mapping, lifecycle execution, and patching.

## DOM Runtime Phase 4

`packages/runtime-dom` is the first concrete renderer. It consumes serialized Frame IR and mounts one component into a browser `Element`.

Implemented:

- mounting and disposal
- element and text node creation
- nested component invocation
- prop and state initialization
- external handler invocation
- click and keyboard modifier listeners
- `value` and `checked` bindings
- `show`, `hidden`, and conditional style metadata
- automatic and explicit style class application
- dependency-aware text, attribute, property, condition, style, and list patching
- keyed list reconciliation by stable item key
- non-keyed list reconciliation by position
- common HTML sectioning, text, list, form, media, SVG, and table elements
- global attributes including `id`, `class`, `data-*`, and `aria-*`
- URL safety checks for `href`, `src`, `srcset`, `poster`, and `action`
- `value`, `checked`, and `selected` bindings
- event modifiers: `prevent`, `stop`, `once`, `capture`, and `passive`
- batched scheduler with deterministic flush order and duplicate patch coalescing
- debug counters for mounts, unmounts, patch categories, queued/flushed patches, active listeners/subscriptions, disposed nodes, runtime errors, and list create/reuse/move/remove operations

Known limitations:

- no SSR
- no hydration
- no routing, portals, transitions, suspense, or async components
- no transition or animation runtime

## Patch Engine

The DOM renderer registers patch callbacks against the state root each IR node depends on.

```txt
state.set("count", 1)
  -> notify subscribers for "count"
  -> enqueue affected patch records
  -> microtask flush in stable registration order
  -> patch text/property/condition/style/list callbacks once
  -> preserve unaffected DOM nodes and listeners
```

Text nodes update `nodeValue`. Bindings update DOM properties such as `value`, `checked`, and `selected`. Conditional properties update the existing element. `show when` and `hidden when` update visibility through `hidden`. Conditional styles toggle classes.

List blocks own their own start and end anchors. Keyed lists reuse items by key and move them into the requested order while preserving node identity and listeners. Non-keyed lists reuse items by position; this behavior is predictable but does not preserve semantic identity across reorder operations. Item-scoped reads such as `$message.text` patch within the item when the list collection changes. Nested lists, component invocations inside list items, conditional nodes inside lists, and lists inside conditional containers are covered by the same local patch path.

The scheduler coalesces duplicate patch records within the same tick and exposes `flush()` for tests. Recursive update loops are guarded with a fixed cycle limit.

## Forms and Attributes

The DOM renderer preserves Frame-generated `fr-*` classes while merging user-provided `class` values. User classes that start with `fr-` are ignored to avoid spoofing compiler-generated style classes.

Form controls patch in place. `value bind` updates text-like controls, `checked bind` updates checkboxes, and `selected bind` updates select controls through `select.value`. Patching preserves focus and text selection where the browser exposes selection APIs.

URL-bearing attributes reject `javascript:` values by default. The compiler also warns about URL sinks and unsafe blank-target links.

## Cleanup Guarantees

The DOM renderer tracks cleanup for component instances, list items, subscriptions, bindings, and event listeners. Disposing an app clears queued patches, disposes root and nested component state, removes active listeners/subscriptions, and removes rendered DOM nodes. Removing a list item disposes that item's listeners/subscriptions without affecting moved or reused items.

## Runtime Diagnostics

Runtime errors include component name and IR source span where available. Missing handlers and handler exceptions are captured in runtime counters and debug traces instead of being silent. Invalid component lookup, invalid state access, invalid list sources, unsafe URL updates, and patch failures throw `FrameDomError` with contextual metadata.

## Component Lifecycle

```txt
instantiate component runtime
  -> validate props
  -> initialize state
  -> register handlers and slots
  -> mount requested
  -> update requested when dirty subscriptions flush
  -> dispose requested
```

The runtime records lifecycle intent only. A renderer decides what mounting, updating, and disposal mean.

## State Lifecycle

```txt
define state descriptor
  -> read inside subscription tracker
  -> write validates type
  -> mark state key dirty
  -> batch end flushes affected subscriptions
  -> renderer schedules target-specific work
```

Reads establish dependencies. Writes mark dirty state. Batches coalesce invalidations.

## Event Lifecycle

```txt
IR event: on keydown.enter @send
  -> EventDescriptor { event, modifiers, handler }
  -> HandlerRegistry lookup
  -> DispatchMetadata { component, target, descriptor }
  -> renderer invokes external handler adapter
```

Frame stores handler references, not JavaScript or TypeScript bodies.

## Abstraction Boundary

The runtime exposes abstract capabilities only:

- `Renderer`
- `RenderNode`
- `RenderElement`
- `RenderText`
- `RenderEvent`
- `RendererCapabilities`

No runtime type mentions DOM, browser APIs, WebView APIs, platform widgets, or canvas APIs.

## Future SSR Support

SSR can consume the same IR and runtime metadata to serialize HTML strings. SSR owns escaping, URL validation, accessibility attribute emission, and serialized event contract manifests. It must not depend on browser APIs.

## Future Native Support

Native renderers can map Frame element kinds to toolkit widgets or retained scene graph nodes. The runtime only requires stable node identity, state dependencies, events, slots, and list metadata.

## Future WASM Support

The runtime model is serializable and can cross a WASM boundary. A WASM host can provide renderer capabilities while Frame keeps component state, dirty tracking, and dispatch metadata in a portable representation.

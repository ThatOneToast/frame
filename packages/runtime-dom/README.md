# @frame/runtime-dom

Browser DOM renderer for Frame IR.

## Supported Features

- Mounting and disposal with full cleanup
- Elements, text nodes, component invocation, props, and state
- Events with modifiers (`prevent`, `stop`, `once`, `capture`, `passive`) and key filters (`enter`, `escape`, `space`, `ctrl`, `shift`, etc.)
- `value`/`checked`/`selected` bindings that sync both directions
- Conditional rendering (`show`/`hidden`) and conditional style classes
- Scheduled dependency-aware patching with deterministic flush order
- Keyed and positional list reconciliation with move/reuse tracking
- Common HTML and SVG element coverage
- Global attributes, `data-*`, `aria-*`, and user class preservation
- URL safety checks rejecting `javascript:` URLs at runtime
- Form controls and event metadata
- Accessibility defaults for semantic primitives
- Mount-time handler and prop validation
- Debug counters and contextual runtime diagnostics

## Not Supported

SSR, hydration, routing, transitions, portals, suspense, async components, or advanced reconciliation.

## Quick Start

```ts
import { mount } from '@frame/runtime-dom';
import ir from './generated/app.ir';

const app = mount(ir, {
  component: 'Counter',
  target: document.getElementById('app')!,
  handlers: {
    increment({ state }) {
      state.set('count', Number(state.get('count')) + 1);
    }
  }
});

app.dispose();
```

## Accessibility

The runtime applies accessibility defaults based on semantic primitive kind:

- `action` → `<button type="button">`
- `toggle` → `<input type="checkbox">`
- `input`/`editor` → placeholder, readonly, and disabled properties
- `image`/`avatar` → `alt` from the `alt` attribute, `decoding="async"`
- `media` → `controls` for video
- `icon` with `decorative: true` → `aria-hidden="true"`
- `field` with `label` → `role="group"` and `aria-label`
- `composer` → `<form method="post">`
- `label` attribute → mapped to `aria-label`

## Diagnostics

Debug mode logs queued and flushed patches with component context:

```ts
const app = mount(ir, { component: 'App', target, debug: true });
```

Debug counters track mounts, unmounts, listeners, subscriptions, patches, list moves, and runtime errors:

```ts
app.getDebugStats();
app.resetDebugStats();
app.flush();
```

Missing handlers are warned at mount time in debug mode. Invalid prop types throw immediately at mount.

## IR Generation

`frame build` writes both `app.ir.json` and `app.ir.ts`. The JSON file is the stable serialized IR artifact. The TS module imports `defineFrameIrDocument` and checks the same IR object as a literal, which keeps TypeScript enum fields such as `value_type: "Text"` aligned with the runtime types.

## Examples

The `examples/` directory uses current Frame-native UI syntax: `screen`, `panel`, `stack`, `row`, `field`, `input`, `editor`, `composer`, `action`, `list`, `feed`, and `data`. Browser element names are intentionally absent from primary examples.

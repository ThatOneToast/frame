# @frame/runtime-dom

Minimal browser DOM renderer for Frame IR.

Phase 4 supports mounting, disposal, elements, text nodes, component invocation, props, state, events, `value`/`checked`/`selected` bindings, conditional rendering, style class application, scheduled dependency-aware patching, keyed or positional list reconciliation, common HTML elements, global attributes, URL safety checks, form controls, cleanup accounting, and runtime diagnostics.

It does not support SSR, hydration, routing, transitions, portals, suspense, async components, or advanced reconciliation.

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

Debug counters are available for tests and instrumentation:

```ts
app.getDebugStats();
app.resetDebugStats();
app.flush();
```

`frame build` writes both `app.ir.json` and `app.ir.ts`. The JSON file is the stable serialized IR artifact. The TS module imports `defineFrameIrDocument` and checks the same IR object as a literal, which keeps TypeScript enum fields such as `value_type: "Text"` aligned with the runtime types.

The `examples/` directory uses current Frame-native UI syntax: `screen`, `panel`, `stack`, `row`, `field`, `input`, `editor`, `composer`, `action`, `list`, `feed`, and `data`. Browser element names are intentionally absent from primary examples.

# @frame/runtime-dom

Minimal browser DOM renderer for Frame IR.

Phase 4 supports mounting, disposal, elements, text nodes, component invocation, props, state, events, `value`/`checked`/`selected` bindings, conditional rendering, style class application, scheduled dependency-aware patching, keyed or positional list reconciliation, common HTML elements, global attributes, URL safety checks, form controls, cleanup accounting, and runtime diagnostics.

It does not support SSR, hydration, routing, transitions, portals, suspense, async components, or advanced reconciliation.

```ts
import { mount } from '@frame/runtime-dom';
import ir from './app.ir.json' assert { type: 'json' };

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

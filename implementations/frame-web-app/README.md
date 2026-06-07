# Frame Web App

A standalone Frame UI project using the DOM runtime.

This app demonstrates:

- Semantic layout (`screen`, `stack`, `row`)
- Accessible actions (`action` with label and disabled state)
- Input field binding (`value bind`)
- Keypress event handling (`keydown.enter`)
- List rendering with `list` and `item`
- State updates from handlers
- Conditional disabled state (`disabled when`)
- Generated typed IR consumed by TypeScript
- Handler implementations using generated types

## Files

- `src/app.frame` — Frame UI source
- `src/handlers.ts` — TypeScript handler implementations
- `src/main.ts` — app entry point
- `src/generated/generated.css` — compiled CSS output
- `src/generated/app.ir.json` — stable serialized Frame IR
- `src/generated/app.ir.ts` — typed IR module
- `src/generated/frame.types.ts` — generated TypeScript contracts
- `src/generated/frame.handlers.ts` — generated skeleton (non-destructive)

## Commands

Build:
```bash
frame build
npm run build
```

Dev server:
```bash
npm install
npm run dev
```

## Runtime Debug

Enable debug mode to see queued and flushed patches:
```ts
const app = mount(appIr, {
  component: 'TodoApp',
  target: document.getElementById('app')!,
  handlers,
  debug: true
});
```

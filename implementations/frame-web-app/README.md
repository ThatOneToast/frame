# Frame Web App

A standalone Frame UI project using the DOM runtime.

This app demonstrates:

- Semantic layout (`screen`, `stack`, `row`)
- Accessible actions (`action` with label and disabled state)
- Input field binding (`value bind`)
- Keypress event handling (`keydown.enter`)
- Keyed list rendering with empty fallback content
- State updates from handlers
- Conditional disabled state (`disabled when`)
- Conditional style switching (`style DisabledButton when $saving`)
- Generated typed IR consumed by TypeScript
- Handler implementations using generated types
- Runtime debug mode with `?debug`

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
npm run frame:build
npm run check
npm run build
```

Dev server:
```bash
npm install
npm run dev
```

`npm run dev`, `npm run check`, and `npm run build` regenerate Frame output before running Vite or TypeScript.

## Runtime Debug

Enable debug mode to see queued and flushed patches:
```bash
npm run dev -- --open '/?debug'
```

The entry point passes `debug: true` when the URL contains `?debug`:
```ts
const app = mount(appIr, {
  component: 'TodoApp',
  target: document.getElementById('app')!,
  handlers,
  debug: new URLSearchParams(window.location.search).has('debug')
});
```

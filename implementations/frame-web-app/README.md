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
- Project theme file (`app-theme.frame`) for shared styles
- Generated typed IR consumed by TypeScript
- Handler implementations using generated types
- Runtime debug mode with `?debug`

## Files

- `src/app.frame` — Frame UI source
- `src/app-theme.frame` — project-wide shared styles and declarations
- `src/handlers.ts` — TypeScript handler implementations
- `src/main.ts` — app entry point
- `src/generated/generated.css` — compiled CSS output
- `src/generated/app.ir.json` — stable serialized Frame IR
- `src/generated/app.ir.ts` — typed IR module
- `src/generated/frame.types.ts` — generated TypeScript contracts
- `src/generated/frame.handlers.ts` — generated skeleton (non-destructive)

## Theme File

Shared styles like `AppShell`, `PrimaryButton`, and `TaskRow` live in `src/app-theme.frame` and are automatically available to every Frame file in the project without explicit `#include`.

## Commands

Build:
```bash
npm run frame:build
npm run check
npm run build
```

Watch mode (rebuilds when any `.frame` file changes):
```bash
npm run frame:watch
```

Dev server (watch + Vite):
```bash
npm install
npm run dev
```

`npm run dev` runs Frame watch and Vite dev in parallel.
`npm run build` runs a one-shot Frame build then Vite build.
`npm run check` regenerates Frame output and type-checks the runtime wiring.

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

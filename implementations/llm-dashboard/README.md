# LLM Operations Dashboard

A dark, glassy, high-contrast dashboard for monitoring LLM operations built with Frame's DOM runtime.

## Features

- **Metric Cards**: Active Model, Tokens/sec, Context Used, VRAM Usage
- **Performance Chart**: SVG-based inference performance visualization
- **Quick Inference Test**: Run test prompts against models
- **Recent Runs Table**: View recent inference runs with filtering
- **Top Models**: Performance comparison across models

## Architecture

- **Frame Styling**: `.frame` files define the dashboard layout and dark theme
- **TypeScript Handlers**: Business logic for interactions
- **DOM Runtime**: `@frame/runtime-dom` for rendering
- **Mock Data**: Simulated LLM metrics and runs

## Development

```bash
# Install dependencies
pnpm install

# Start dev server (builds Frame + starts Vite)
pnpm run dev

# Build for production
pnpm run build

# Type check
pnpm run check
```

**Prerequisites**: The `@frame/runtime-dom` package must be built first:
```bash
cd ../../packages/runtime-dom && npm install && npx tsc -p tsconfig.json
```

## Project Structure

```
src/
  app.frame          # Main Frame component (UI structure)
  app-theme.frame    # Dark theme styles and layout
  main.ts            # DOM runtime mount point
  handlers.ts        # Event handlers
  data/
    models.ts        # Mock data models
  generated/         # Frame compiler output (do not edit)
```

## Design System

- **Background**: #0A0A0F (dark)
- **Surface**: #12121A (raised panels)
- **Border**: #2A2A36 (soft white opacity)
- **Accent**: #3B82F6 (blue)
- **Text Primary**: #F8FAFC (white)
- **Text Secondary**: #94A3B8 (gray)

## Known Limitations

- **No body/root styling**: Frame cannot set `body { background }` or `body { color }`. The dark theme only applies to Frame elements, not the document body.
- **No table primitives**: Tables are built with `row` + `card` patterns, not native table elements.
- **No real LLM provider connections**: Mock data only.
- **SVG chart is a placeholder**: No real charting library integration.

## Frame Patterns Used

See `docs/agents/ui-implementation-guide.md` for the Frame patterns and anti-patterns demonstrated in this implementation.

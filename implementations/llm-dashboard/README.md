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
npm install

# Start dev server (builds Frame + starts Vite)
npm run dev

# Build for production
npm run build

# Type check
npm run check
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
- **Border**: #2A2A36
- **Accent**: #3B82F6 (blue)
- **Text Primary**: #F8FAFC
- **Text Secondary**: #94A3B8

## Limitations

- No real LLM provider connections
- Mock data only
- SVG chart is a placeholder
- Table uses row-based layout (Frame doesn't have native table primitives)

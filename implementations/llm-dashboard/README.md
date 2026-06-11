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

- **No table primitives**: Tables are built with `row` + `card` patterns, not native table elements.
- **No real LLM provider connections**: Mock data only.
- **SVG chart is a placeholder**: No real charting library integration.

## Frame Patterns Used

### Grid-Backed Table Rows

The runs table and top models list use `columns` on `row` declarations for column-aligned layouts. This emits `display: grid` with a shared column template instead of flex:

```frame
row TableRowBase {
  columns 2fr 1fr 1fr 1fr 1fr 1fr
  align center
}
```

### Content Sizing Tokens

Chart panels use `height chart` (12rem) instead of spacing tokens:

```frame
row ChartBars {
  height chart
  flex grow 1
}
```

### Gap-Safe Grid Columns

Dashboard grids use `fr` columns instead of percentages to prevent overflow with gaps:

```frame
grid PerformanceGrid {
  columns 3fr 2fr
  gap medium
}
```

### Action and Tab Styling

Tabs are `action` declarations with hover states:

```frame
action TabAll {
  surface overlay
  color text-primary
  hover {
    lift tiny
  }
}
```

### Loop-Driven Sections

The top models list uses `for` loops instead of hardcoded rows:

```frame
for model in $topModels key $model.id {
  row ModelRow {
    card RankBadge { text $model.rank }
    stack ModelInfo { text $model.name }
    text $model.delta
  }
}
```

See `docs/agents/ui-implementation-guide.md` for more Frame patterns and anti-patterns.

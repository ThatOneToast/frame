# Frame IDE

A minimal text editor for Frame, built using Frame UI primitives, the Frame DOM runtime, and the Frame LSP server.

## Features

- **Dark, floaty, minimal theme** using Frame design tokens
- **File tree sidebar** with localStorage persistence
- **Text editor** using Frame's native `editor` primitive (`<textarea>`)
- **LSP client** that communicates with `frame_lsp` via JSON-RPC over WebSocket
- **Keyboard shortcuts**: Ctrl/Cmd + N (new), O (open), S (save)

## Architecture

```
Frame source (app.frame, theme.frame)
  -> frame build
  -> generated.css (dark theme + layout)
  -> generated.ts (class map)
  -> app.ir.json (serialized Frame IR)
  -> app.ir.ts (typed IR module)

TypeScript handlers
  -> main.ts (mounts Frame DOM runtime)
  -> handlers.ts (file operations, LSP, keyboard shortcuts)
  -> lsp-client.ts (JSON-RPC over WebSocket)
```

No manual DOM setup is required. Frame owns the UI model; TypeScript only provides logic. `main.ts` imports `app.ir.ts` so the compiler checks IR enum/default shapes against `@frame/runtime-dom` types.

## Usage

Build the Frame styling and IR:

```bash
cd implementations/frame-ide
frame build
```

Or use the Frame CLI from the repository:

```bash
cargo run -p frame_cli -- build
```

Run the Vite dev server:

```bash
npm run dev
```

Or build for production:

```bash
npm run build
npx serve dist
```

## LSP Connection

The editor attempts to connect to a WebSocket LSP server at `ws://localhost:3000/frame-lsp`.

To run the LSP server:

```bash
cargo run -p frame_lsp -- --port 3000
```

Or install it on PATH:

```bash
cargo install --path crates/frame_lsp
frame_lsp --port 3000
```

## Dogfooding

This application is built entirely from Frame's own primitives:

- `screen` for the root container
- `dock` for the toolbar and status bar
- `panel` for the sidebar and editor pane
- `split` for the main layout
- `list` for the file tree
- `editor` for the code editor
- `action` for buttons
- `card` for file items

No Monaco, CodeMirror, or external editor libraries are used.

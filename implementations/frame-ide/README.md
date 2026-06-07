# Frame IDE

A minimal text editor for Frame, built using Frame UI primitives and styling.

## Features

- **Dark, floaty, minimal theme** using Frame design tokens
- **File tree sidebar** with localStorage persistence
- **Text editor** built from scratch with:
  - Line numbers
  - Basic syntax highlighting for Frame source
  - Diagnostic underlines from LSP
  - Keyboard shortcuts (Ctrl/Cmd + N, O, S)
- **LSP client** that communicates with `frame_lsp` via JSON-RPC over WebSocket

## Architecture

```
Frame source (app.frame, theme.frame)
  -> frame compile
  -> generated.css (dark theme + layout)
  -> generated.ts (class map)

TypeScript app
  -> app.ts (main controller)
  -> editor.ts (contenteditable-based editor with token highlighting)
  -> file-tree.ts (localStorage-backed file list)
  -> lsp-client.ts (JSON-RPC over WebSocket)
```

## Usage

Compile the Frame styling:

```bash
cd implementations/frame-ide
frame compile src/frame/app.frame --out src/generated
```

Or use the Frame CLI from the repository:

```bash
cargo run -p frame_cli -- compile src/frame/app.frame --out src/generated
```

Serve `index.html` with any static server:

```bash
npx serve .
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
- `card` for file items and tokens

No Monaco, CodeMirror, or external editor libraries are used.

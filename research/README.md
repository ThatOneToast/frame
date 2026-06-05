# Research

This directory contains planning notes for the Frame overhaul.

Frame is moving toward this architecture:

```txt
Frame source
  -> parser
  -> semantic model
  -> Frame IR
  -> renderer targets
```

The first renderer target is the browser DOM. Tauri can reuse that output through a WebView. Native renderers should wait until the IR is stable.

## Files

- `architecture.md` explains the planned compiler and runtime layers.
- `frame-ir.md` explains what the intermediate representation needs to store.
- `dom-runtime.md` explains what the DOM runtime needs to do.
- `security.md` explains the default safety model.
- `syntax-notes.md` collects early syntax examples.

These notes are planning material. They are not proof that the behavior exists yet.

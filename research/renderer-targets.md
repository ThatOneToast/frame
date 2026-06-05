# Renderer Targets

Frame should not be designed around a single framework.

The compiler should lower Frame source into Frame IR. Renderer targets should consume that IR.

## Target Order

1. DOM runtime
2. Static HTML output where possible
3. Tauri WebView using the DOM runtime
4. Native renderer research

## DOM Runtime

The DOM runtime should be the first target because it directly maps to web platform behavior.

It should support:

- elements
- text nodes
- attributes
- properties
- events
- forms
- accessibility attributes
- reactive state patches
- style class changes

## Static HTML

Static HTML output should be possible for Frame views that do not require client state or client handlers.

Useful cases:

- documentation pages
- marketing pages
- server-rendered shells
- initial render output

## Tauri WebView

Tauri should reuse the DOM runtime.

Frame should not expose desktop commands directly in UI syntax. Desktop calls should live in external handler code.

## Native Renderers

Native renderers should wait until the IR stabilizes.

Potential future targets:

- custom winit and wgpu renderer
- egui
- iced
- slint
- platform-native widgets

## Compatibility Targets

Svelte and React can be studied as compatibility targets, but they should not define Frame's internal model.

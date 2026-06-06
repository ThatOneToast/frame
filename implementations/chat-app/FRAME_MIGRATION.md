# Frame UI Migration Plan

This is a planning document for moving the chat app from Svelte components plus Frame-generated classes toward Frame UI components. The app should keep running in Svelte until Frame has a DOM runtime with enough behavior coverage.

## Target Components

- `ChatApp`
  - Owns app-level state such as active server, active channel, draft text, loading state, and message send state.
  - Maps to `src/lib/components/layout/AppShell.svelte` plus the route-level app entry.
- `ServerRail`
  - Replaces `src/lib/components/layout/Sidebar.svelte`.
  - Uses existing styles from `sidebar.frame` and `app-shell.frame` such as `SidebarRail`, `SidebarTop`, and `RailLabel`.
- `ChannelSidebar`
  - Replaces `src/lib/components/layout/ChannelPanel.svelte` and `src/lib/components/channels/ChannelList.svelte`.
  - Uses `PanelHeader`, `PanelTitle`, `PanelMeta`, `ChannelList`, and channel row styles.
- `ChannelButton`
  - Replaces `src/lib/components/channels/ChannelRow.svelte`.
  - Needs props for `channel`, active state, unread count, and handler references such as `@selectChannel`.
- `ChatPanel`
  - Replaces `src/lib/components/layout/ChatPanel.svelte`.
  - Composes `Header`, `MessageList`, and eventually `MessageComposer`.
- `MessageList`
  - Replaces `src/lib/components/messages/MessageList.svelte`.
  - Needs list rendering over messages and access to users/authors.
- `MessageItem`
  - Replaces `src/lib/components/messages/MessageItem.svelte`.
  - Needs props, conditional rendering for edited state, formatted time, and safe text insertion.
- `MessageComposer`
  - Replaces `src/lib/components/messages/MessageComposer.svelte`.
  - Needs input binding, submit/key handling, disabled/sending state, placeholder data, and handler contracts.
- `UserPanel`
  - Replaces `src/lib/components/layout/UserPanel.svelte`.
  - Composes `UserList` and panel header styles.
- `UserItem`
  - Replaces `src/lib/components/users/UserRow.svelte`.
  - Needs props for user identity and presence state.
- `SettingsPanel`
  - Maps to `settings.frame` styles such as `SettingsOverlay` and `SettingsPanel`.
  - Needs conditional visibility, focus management, and close/dismiss handlers before it can be runtime-backed.

## Required Frame Features

Already implemented:

- Structured styling declarations and CSS/TypeScript class export generation.
- UI `component`, `props`, `state`, `view`, and `slot` parsing.
- UI elements, text nodes, data refs, handler refs, event bindings, value bindings, conditional rendering, conditional properties, and conditional style syntax.
- Same-file component invocation syntax, including `Child()`, `Child(prop: $state)`, and `Child(prop bind $state)`.
- Semantic validation for UI syntax, same-file component invocation names, prop resolution, state resolution, accessibility, and URL-bearing attributes.
- Frame IR lowering for UI syntax including props, state, slots, conditions, events, and bindings.
- JSON IR serialization with `frame emit-ir`.
- TypeScript prop, state, and handler contract generation with `frame emit-contracts`.
- LSP completions, hovers, semantic tokens, and Zed highlighting for UI syntax.

Still needed (DOM runtime phase):

- List rendering for channels, messages, users, reactions, and servers.
- DOM runtime mounting, patching, event dispatch, and cleanup.
- Handler implementation binding from generated contracts to user TypeScript.
- Form and input runtime behavior for `MessageComposer`.
- Attribute coverage for accessibility, ARIA, labels, roles, time/date metadata, and forms.
- URL and unsafe sink validation where relevant.
- Derived/computed data support for active channel, active server, author lookup, and formatted timestamps.
- Routing or route integration if the app grows beyond the current single-screen shell.

## Migration Strategy

1. Keep the existing Svelte app running.
2. Create `.frame` UI equivalents next to the current Svelte components, starting with leaf components that use currently implemented syntax.
3. Generate IR and contracts from those Frame files and inspect them in CI or local scripts.
4. Add the DOM runtime only after the IR shape is stable enough for components, state, bindings, and events.
5. Replace Svelte components incrementally, starting with static or low-interaction surfaces.
6. Move interactive components such as `ChannelButton` and `MessageComposer` only after props, loops, form bindings, and handler binding are implemented.
7. Remove Svelte only after Frame can render the full shell, lists, conditionals, accessibility attributes, and input behavior.

## Near-Term Frame Samples

Useful first samples that fit the current compiler:

- `MessageComposer` with local `draft` and `sending` state, `value bind $draft`, `on keydown.enter @submitMessage`, and `on click @submitMessage`.
- `ChatApp` as a composition-only shell using `ServerRail()`, `ChannelSidebar()`, `ChatPanel()`, `UserPanel()`, and `MessageComposer(draft bind $draft)`.
- `SettingsPanel` as a styled panel with conditional `disabled when $saving` and external handlers.

These samples should not be wired into the app until the DOM runtime exists.

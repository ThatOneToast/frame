# Frame Chat App

This implementation is a SvelteKit reference app for Frame. It presents a terminal-inspired communication workspace with a Discord-like information architecture: server rail, channel panel, chat timeline, member list, header, and message composer.

The app is intentionally desktop-first. It is not a Discord clone; it uses a dense, modern terminal visual language to stress Frame's layout, state, token, typography, and component styling workflows.

## Architecture

The project keeps application concerns separated:

- `src/lib/models`: typed `Server`, `Channel`, `Message`, and `User` models.
- `src/lib/api`: mocked backend services such as `getChannels()`, `getMessages()`, `getUsers()`, and `sendMessage()`.
- `src/lib/stores`: Svelte stores that load data from services and expose active server, active channel, messages, and users.
- `src/lib/components`: modular Svelte components for layout, channels, messages, users, and shared controls.
- `src/lib/frame`: Frame source files.
- `src/lib/generated`: generated `generated.css` and `generated.ts` outputs.

Components consume stores and generated Frame classes. They do not own mocked data.

## Frame Usage

All styling lives in `.frame` files. There are no Svelte style blocks.

`src/lib/frame/app.frame` is the compiler entry point and includes focused modules:

- `theme.frame`: color, typography, shared button, and text primitives.
- `app-shell.frame`: viewport and main application grid.
- `header.frame`: workspace header and actions.
- `sidebar.frame`: server rail.
- `channels.frame`: channel panel and channel rows.
- `messages.frame`: message timeline and composer.
- `users.frame`: member list and presence states.
- `settings.frame`: future settings overlay classes.

The app uses Frame declarations aggressively: `tokens`, `grid`, `area`, `stack`, `row`, `center`, `card`, `button`, `text`, `overlay`, hover/focus/disabled states, animations, surfaces, borders, spacing, and typography.

## Components

- `AppShell.svelte`: composes the main application regions.
- `Sidebar.svelte`: server navigation only.
- `ChannelPanel.svelte`: channel navigation only.
- `ChatPanel.svelte`: message viewport only.
- `MessageList.svelte` and `MessageItem.svelte`: message rendering only.
- `MessageComposer.svelte`: draft input and send action only.
- `UserPanel.svelte`, `UserList.svelte`, and `UserRow.svelte`: member list only.
- `Header.svelte`: current workspace and channel context.

## API Layer

The mocked API returns typed data and keeps UI components independent from fixture data. Replacing the mock layer with REST, WebSockets, gRPC, TNet, or a custom Rust backend should happen in `src/lib/api` and `src/lib/stores`, without rewriting presentation components.

## Future Work

- Voice channel surfaces and live participant state.
- Direct messages and multi-server routing.
- Notifications and unread mention filtering.
- User profiles and authentication.
- Runtime theme switching.
- WebSocket-backed message streaming.
- File uploads and attachments.
- Settings routes using the existing settings Frame module.

# Contributing to Frame

## Canonical Language Registry

`crates/frame_core/src/language.rs` is the single source of truth for every Frame language concept. All parser keywords, semantic categories, LSP completions, hover docs, semantic token classes, and diagnostics ultimately consume this registry.

Before adding or changing any language concept, update the registry.

### Registry responsibilities

- **Declarations** — `grid`, `area`, `card`, `stack`, `row`, `dock`, `split`, `overlay`, `tokens`, `keyframes`, `component`, `view`, `text`, `center`, etc.
- **UI primitives** — `action`, `link`, `panel`, `field`, `input`, `editor`, `toggle`, `choice`, `select`, `list`, `feed`, `data`, `dialog`, `menu`, `toolbar`, `tabs`, `composer`, `image`, `avatar`, `media`, `icon`, `badge`, `title`, `label`, `screen`, `scroll`, etc.
- **Properties** — `surface`, `padding`, `gap`, `align`, `justify`, `color`, `shadow`, `radius`, `border`, `display`, `flex`, `position`, `css`, etc.
- **Values** — `small`, `medium`, `large`, `accent`, `panel`, `flex`, `row`, `start`, `center`, `end`, etc.
- **Events** — `click`, `press`, `keydown`, `keyup`, `change`, `submit`, `send`, etc.
- **Event modifiers** — `enter`, `ctrl`, `alt`, `shift`, `meta`, `once`, `prevent`, `stop`, `passive`, etc.
- **State keywords** — `hover`, `focus`, `focus-visible`, `active`, `disabled`, `checked`, `invalid`, `required`, etc.
- **Binding keywords** — `bind`, `when`.
- **Effects** — `lift`, `sink`, `shift`, `grow`, `shrink`, `tilt`, `glow`, `brighten`, `dim`, `press`, `pop`, `ring`, `blur`, `smooth`, `fade`, `scale`, `rotate`, `slide`, `transition`, `duration`, `ease`, `animation`, `animate`.

### How to add a new language concept

1. **Add it to the canonical registry** in `crates/frame_core/src/language.rs`.
   - Choose the correct `LanguageItemKind` (Primitive, Declaration, Property, Value, Event, etc.).
   - Choose the correct `LanguageLayer` (Ui, Style, Motion, Layout, Typography, Advanced, EscapeHatch).
   - Choose the correct `CompletionCategory` and `SemanticTokenClass`.
   - Set `status` to `Stable`, `Experimental`, `Deprecated`, `Advanced`, or `EscapeHatch`.

2. **Add parser grammar only if the syntax shape changes.**
   - If the new concept fits an existing parser rule (e.g. a new property value), no parser changes are needed.
   - If the syntax shape is new (e.g. a new block type or modifier position), update the relevant parser module.

3. **Add tests for completion, hover, and highlighting.**
   - Ensure the LSP picks up the new item from the registry.
   - Add semantic token tests if the token class changes.

4. **Update samples and docs.**
   - Add examples in `editors/zed/samples/` if the concept is UI-related.
   - Update relevant docs under `docs/`.
   - Update the language cheat sheet in `docs/agents/language-cheatsheet.md`.

## Syntax Direction

Frame is UI-native first, advanced/escape-hatch second.

### Preferred authoring path

Use intent-first primitives inside `view`:

```frame
component ChatShell {
  state message Text = ""
  view {
    dock App {
      panel ChannelList
      stack Conversation {
        feed Messages
        composer MessageBox {
          field MessageInput
          bind $message
          action Send
          on press @send_message
        }
      }
    }
  }
}
```

### Advanced and escape hatches

Low-level CSS and DOM concepts remain available in the styling layer and through explicit escape hatches:

- `display flex`, `display grid` — valid in styling declarations, but prefer `stack`, `flow`, `grid`, `dock` layout primitives in UI `view` blocks.
- `position absolute`, `position fixed` — valid in styling, but prefer `overlay`, `dock`, `scroll` primitives in UI `view` blocks.
- `css "raw-property" value` — valid inside `advanced { ... }` blocks for true escape hatches.

The compiler and LSP should teach the preferred path first, then document the advanced path when a user reaches for it.

## IDE Intelligence Pipeline

Frame IDE features (completions, hover, diagnostics, references, definitions, and semantic tokens) are built from two shared sources:

1. **The canonical language registry** in `crates/frame_core/src/language.rs`
   - Owns language concepts, docs, completion metadata, semantic categories, and value domains.
   - Consumed by completions, hover, diagnostics, and semantic tokens.

2. **The semantic cursor model** in `crates/frame_lsp/src/ide/cursor.rs`
   - Owns cursor context, scope, visible symbols, enclosing blocks, and incomplete syntax recovery.
   - Built from parsed AST + source text.
   - Consumed by completions, hover, diagnostics, and references.

### How to add a new IDE-aware concept

1. **Add the concept to the canonical registry** in `crates/frame_core/src/language.rs`.
2. **Add parser support** only if the syntax shape changes.
3. **Add semantic cursor slot/scope handling** if the new concept changes cursor context (e.g., a new block type or reference syntax).
4. **Add completion/hover/diagnostic/reference tests** in `crates/frame_lsp/src/`.
5. **Update examples and docs**.

### Cursor slot dispatch

Completions and hover dispatch on `CursorSlot`:

- `RootDeclaration` — declarations, snippets, includes
- `DeclarationBody` / `StylePropertyName` / `StylePropertyValue` — properties and values
- `ViewBody` / `ViewPrimitive` / `ViewNodeName` — UI primitives, declarations, components
- `EventName` — events and modifiers
- `DataReference` — `$state`, `$props`, loop vars
- `HandlerReference` — `@handler` names
- `StateDeclaration` — effects and state blocks

Diagnostics use `SemanticCursor` to detect unknown primitives, properties, values, handlers, and state references with registry-aware suggestions.

References use `SemanticCursor` to classify reference kinds (`StateRead`, `HandlerReference`, `ComponentInvocation`, etc.) and honor `includeDeclaration`.

## Testing Requirements

Run the full workspace test suite before claiming work complete:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

When JavaScript packages are involved, also run the relevant package tests:

```bash
npm test
```

## Documentation Expectations

Every new language concept should explain:

- what the syntax means
- what the compiler stores in the AST/IR
- what the runtime does
- what the DOM output should do
- what diagnostics should exist
- what remains unsupported

Keep docs direct, useful, and low on hype.

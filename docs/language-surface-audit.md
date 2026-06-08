# Language Surface Audit

This document tracks the coverage of the Frame language surface across the canonical registry, parser, semantic layer, LSP, Zed, and runtime DOM. It is actionable and designed to be updated as coverage changes.

**Last updated:** 2026-06-08

## Legend

| Symbol | Meaning |
|--------|---------|
| **yes** | Full support |
| **partial** | Recognized but with gaps or limitations |
| **no** | Not yet supported |

## UI Primitives

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| action | yes | yes | yes | yes | yes | yes | yes | complete |
| avatar | yes | yes | yes | yes | yes | yes | yes | complete |
| badge | yes | yes | yes | yes | yes | yes | yes | complete |
| choice | yes | yes | yes | yes | yes | yes | yes | complete |
| composer | yes | yes | yes | yes | yes | yes | yes | complete |
| data | yes | yes | yes | yes | yes | yes | yes | complete |
| dialog | yes | yes | yes | yes | yes | yes | yes | complete |
| editor | yes | yes | yes | yes | yes | yes | yes | complete |
| empty | yes | yes | yes | yes | yes | yes | yes | complete |
| feed | yes | yes | yes | yes | yes | yes | yes | complete |
| field | yes | yes | yes | yes | yes | yes | yes | complete |
| icon | yes | yes | yes | yes | yes | yes | yes | complete |
| image | yes | yes | yes | yes | yes | yes | yes | complete |
| input | yes | yes | yes | yes | yes | yes | yes | complete |
| item | yes | yes | yes | yes | yes | yes | yes | complete |
| label | yes | yes | yes | yes | yes | yes | yes | complete |
| link | yes | yes | yes | yes | yes | yes | yes | complete |
| list | yes | yes | yes | yes | yes | yes | yes | complete |
| media | yes | yes | yes | yes | yes | yes | yes | complete |
| menu | yes | yes | yes | yes | yes | yes | yes | complete |
| panel | yes | yes | yes | yes | yes | yes | yes | complete |
| popover | yes | yes | yes | yes | yes | yes | yes | complete |
| screen | yes | yes | yes | yes | yes | yes | yes | complete |
| scroll | yes | yes | yes | yes | yes | yes | yes | complete |
| section | yes | yes | yes | yes | yes | yes | yes | complete |
| select | yes | yes | yes | yes | yes | yes | yes | complete |
| tabs | yes | yes | yes | yes | yes | yes | yes | complete |
| title | yes | yes | yes | yes | yes | yes | **no** | **missing** |
| text | yes | yes | yes | yes | yes | yes | yes | complete |
| toggle | yes | yes | yes | yes | yes | yes | yes | complete |
| toolbar | yes | yes | yes | yes | yes | yes | yes | complete |

### Notes
- `title` is a registry primitive and is recognized by the parser, semantic layer, and LSP, but it is **missing from the runtime DOM `ELEMENT_TAGS` mapping**. The runtime should map `title` to an appropriate heading element (e.g. `<h1>`–`<h6>`) or a generic container.
- `card`, `row`, `stack`, `grid`, `split`, `dock`, `overlay`, `center`, `button` are registry **declarations** but are also valid inside `view` blocks. The parser accepts them with braces, but `looks_like_semantic_shorthand` does not recognize them as primitives, so single-line shorthand without braces does not work for these declarations.

## Declarations

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| area | yes | yes | yes | yes | yes | yes | yes | complete |
| card | yes | yes | yes | yes | yes | yes | yes | complete |
| center | yes | yes | yes | yes | yes | yes | yes | complete |
| component | yes | yes | yes | yes | yes | yes | no | partial |
| dock | yes | yes | yes | yes | yes | yes | yes | complete |
| grid | yes | yes | yes | yes | yes | yes | yes | complete |
| keyframes | yes | yes | yes | yes | yes | yes | no | partial |
| overlay | yes | yes | yes | yes | yes | yes | yes | complete |
| row | yes | yes | yes | yes | yes | yes | yes | complete |
| split | yes | yes | yes | yes | yes | yes | yes | complete |
| stack | yes | yes | yes | yes | yes | yes | yes | complete |
| style-group | yes | yes | yes | yes | yes | yes | no | partial |
| style-order | yes | yes | yes | yes | yes | yes | no | partial |
| supports | yes | yes | yes | yes | yes | yes | no | partial |
| text | yes | yes | yes | yes | yes | yes | yes | complete |
| tokens | yes | yes | yes | yes | yes | yes | no | partial |
| button | yes | yes | yes | yes | yes | yes | yes | complete |

### Notes
- `component` has a dedicated parser rule but is **not handled by `declaration_kind()`** in the parser helpers. It is a structural declaration, not a renderable element, so runtime DOM mapping is intentionally absent.
- `keyframes`, `style-group`, `style-order`, `supports`, `tokens` are non-renderable declarations and do not need runtime DOM mappings.
- `component` should be added to `declaration_kind()` in `frame_parser/src/helpers.rs` to keep the parser helper surface aligned with the registry.

## Properties (Top 30)

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| surface | yes | yes | yes | yes | yes | yes | N/A | complete |
| padding | yes | yes | yes | yes | yes | yes | N/A | complete |
| gap | yes | yes | yes | yes | yes | yes | N/A | complete |
| align | yes | yes | yes | yes | yes | yes | N/A | complete |
| justify | yes | yes | yes | yes | yes | yes | N/A | complete |
| color | yes | yes | yes | yes | yes | yes | N/A | complete |
| shadow | yes | yes | yes | yes | yes | yes | N/A | complete |
| radius | yes | yes | yes | yes | yes | yes | N/A | complete |
| border | yes | yes | yes | yes | yes | yes | N/A | complete |
| display | yes | yes | yes | yes | yes | yes | N/A | complete |
| flex | yes | yes | yes | yes | yes | yes | N/A | complete |
| position | yes | yes | yes | yes | yes | yes | N/A | complete |
| width | yes | yes | yes | yes | yes | yes | N/A | complete |
| height | yes | yes | yes | yes | yes | yes | N/A | complete |
| inline-size | yes | yes | yes | yes | yes | yes | N/A | complete |
| block-size | yes | yes | yes | yes | yes | yes | N/A | complete |
| flow | yes | yes | yes | yes | yes | yes | N/A | complete |
| columns | yes | yes | yes | yes | yes | yes | N/A | complete |
| rows | yes | yes | yes | yes | yes | yes | N/A | complete |
| tracks | yes | yes | yes | yes | yes | yes | N/A | complete |
| areas | yes | yes | yes | yes | yes | yes | N/A | complete |
| section | yes | yes | yes | yes | yes | yes | N/A | complete |
| transition | yes | yes | yes | yes | yes | yes | N/A | complete |
| duration | yes | yes | yes | yes | yes | yes | N/A | complete |
| ease | yes | yes | yes | yes | yes | yes | N/A | complete |
| animation | yes | yes | yes | yes | yes | yes | N/A | complete |
| advanced | yes | yes | yes | yes | yes | yes | N/A | complete |
| hover | yes | yes | yes | yes | yes | yes | N/A | complete |
| focus | yes | yes | yes | yes | yes | yes | N/A | complete |
| focus-visible | yes | yes | yes | yes | yes | yes | N/A | complete |
| active | yes | yes | yes | yes | yes | yes | N/A | complete |
| disabled | yes | yes | yes | yes | yes | yes | N/A | complete |
| checked | yes | yes | yes | yes | yes | yes | N/A | complete |
| invalid | yes | yes | yes | yes | yes | yes | N/A | complete |
| required | yes | yes | yes | yes | yes | yes | N/A | complete |
| target | yes | yes | yes | yes | yes | yes | N/A | complete |

### Notes
- Parser accepts any property name in UI blocks and declarations; validation is semantic.
- Semantic validation has per-primitive allow-lists (`valid_properties_for_primitive`) but does not yet validate every property value against the registry.
- Runtime DOM does not validate properties directly; they are lowered to CSS classes via `frame_codegen`.

## Values (Top 30)

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| small | yes | partial | yes | yes | partial | partial | N/A | partial |
| medium | yes | partial | yes | yes | partial | partial | N/A | partial |
| large | yes | partial | yes | yes | partial | partial | N/A | partial |
| accent | yes | partial | yes | yes | partial | partial | N/A | partial |
| panel | yes | partial | yes | yes | partial | partial | N/A | partial |
| flex | yes | partial | yes | yes | partial | partial | N/A | partial |
| row | yes | partial | yes | yes | partial | partial | N/A | partial |
| start | yes | partial | yes | yes | partial | partial | N/A | partial |
| center | yes | partial | yes | yes | partial | partial | N/A | partial |
| end | yes | partial | yes | yes | partial | partial | N/A | partial |
| stretch | yes | partial | yes | yes | partial | partial | N/A | partial |
| between | yes | partial | yes | yes | partial | partial | N/A | partial |
| around | yes | partial | yes | yes | partial | partial | N/A | partial |
| evenly | yes | partial | yes | yes | partial | partial | N/A | partial |
| screen | yes | partial | yes | yes | partial | partial | N/A | partial |
| fill | yes | partial | yes | yes | partial | partial | N/A | partial |
| content | yes | partial | yes | yes | partial | partial | N/A | partial |
| auto | yes | partial | yes | yes | partial | partial | N/A | partial |
| sidebar | yes | partial | yes | yes | partial | partial | N/A | partial |
| main | yes | partial | yes | yes | partial | partial | N/A | partial |
| inspector | yes | partial | yes | yes | partial | partial | N/A | partial |
| header | yes | partial | yes | yes | partial | partial | N/A | partial |
| footer | yes | partial | yes | yes | partial | partial | N/A | partial |
| responsive | yes | partial | yes | yes | partial | partial | N/A | partial |
| cards | yes | partial | yes | yes | partial | partial | N/A | partial |
| rail | yes | partial | yes | yes | partial | partial | N/A | partial |
| composer | yes | partial | yes | yes | partial | partial | N/A | partial |
| heading | yes | partial | yes | yes | partial | partial | N/A | partial |
| body | yes | partial | yes | yes | partial | partial | N/A | partial |
| caption | yes | partial | yes | yes | partial | partial | N/A | partial |
| mono | yes | partial | yes | yes | partial | partial | N/A | partial |
| bold | yes | partial | yes | yes | partial | partial | N/A | partial |
| semibold | yes | partial | yes | yes | partial | partial | N/A | partial |
| normal | yes | partial | yes | yes | partial | partial | N/A | partial |
| thin | yes | partial | yes | yes | partial | partial | N/A | partial |
| dusk | yes | partial | yes | yes | partial | partial | N/A | partial |
| midnight | yes | partial | yes | yes | partial | partial | N/A | partial |
| aurora | yes | partial | yes | yes | partial | partial | N/A | partial |
| ember | yes | partial | yes | yes | partial | partial | N/A | partial |
| ocean | yes | partial | yes | yes | partial | partial | N/A | partial |
| forest | yes | partial | yes | yes | partial | partial | N/A | partial |

### Notes
- Values are accepted as literals by the parser. Semantic value validation is limited to a few hardcoded contexts (spacing, colors, surfaces, etc.). The registry catalogs many more values than the semantic validator currently checks.
- Zed highlights most values as `@constant` inside statement values, but not all value categories are equally covered.

## Events

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| press | yes | yes | yes | yes | yes | yes | yes | complete |
| click | yes | yes | yes | yes | yes | yes | yes | complete |
| keydown | yes | yes | yes | yes | yes | yes | yes | complete |
| keyup | yes | yes | yes | yes | yes | yes | yes | complete |
| change | yes | yes | yes | yes | yes | yes | yes | complete |
| submit | yes | yes | yes | yes | yes | yes | yes | complete |
| send | yes | yes | yes | yes | yes | yes | yes | complete |
| input | yes | yes | yes | yes | yes | yes | yes | complete |
| reset | yes | yes | yes | yes | yes | yes | yes | complete |
| focus | yes | yes | yes | yes | yes | yes | yes | complete |
| blur | yes | yes | yes | yes | yes | yes | yes | complete |
| pointerdown | yes | yes | yes | yes | yes | yes | yes | complete |
| pointerup | yes | yes | yes | yes | yes | yes | yes | complete |
| pointermove | yes | yes | yes | yes | yes | yes | yes | complete |
| mouseenter | yes | yes | yes | yes | yes | yes | yes | complete |
| mouseleave | yes | yes | yes | yes | yes | yes | yes | complete |
| open | yes | yes | yes | yes | yes | yes | yes | complete |
| close | yes | yes | yes | yes | yes | yes | yes | complete |
| select | yes | yes | yes | yes | yes | yes | yes | complete |

### Notes
- `press`, `blur`, `focus`, `select`, `input` are classified in the registry as `Effect`, `StateKeyword`, or `Primitive`, but are treated as events by the semantic layer and runtime. The registry should either reclassify them as `Event` or the semantic layer should source them from the registry dynamically.
- Parser accepts any event name at parse time; validation happens in semantic.

## Event Modifiers

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| enter | yes | yes | yes | yes | yes | yes | yes | complete |
| ctrl | yes | yes | yes | yes | yes | yes | yes | complete |
| alt | yes | yes | yes | yes | yes | yes | yes | complete |
| shift | yes | yes | yes | yes | yes | yes | yes | complete |
| meta | yes | yes | yes | yes | yes | yes | yes | complete |
| once | yes | yes | yes | yes | yes | yes | yes | complete |
| prevent | yes | yes | yes | yes | yes | yes | yes | complete |
| stop | yes | yes | yes | yes | yes | yes | yes | complete |
| passive | yes | yes | yes | yes | yes | yes | yes | complete |
| capture | yes | yes | yes | yes | yes | yes | yes | complete |
| escape | yes | yes | yes | yes | yes | yes | yes | complete |
| tab | yes | yes | yes | yes | yes | yes | yes | complete |
| space | yes | yes | yes | yes | yes | yes | yes | complete |
| left | yes | yes | yes | yes | yes | yes | yes | complete |
| right | yes | yes | yes | yes | yes | yes | yes | complete |
| up | yes | yes | yes | yes | yes | yes | yes | complete |
| down | yes | yes | yes | yes | yes | yes | yes | complete |

### Notes
- Event modifiers are fully covered across all layers. The parser accepts any modifier; semantic and runtime validate the known set.

## State Keywords

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| hover | yes | yes | yes | yes | yes | yes | N/A | complete |
| focus | yes | yes | yes | yes | yes | yes | N/A | complete |
| focus-visible | yes | yes | yes | yes | yes | yes | N/A | complete |
| focus-within | yes | yes | yes | yes | yes | yes | N/A | complete |
| active | yes | yes | yes | yes | yes | yes | N/A | complete |
| disabled | yes | yes | yes | yes | yes | yes | N/A | complete |
| checked | yes | yes | yes | yes | yes | yes | N/A | complete |
| invalid | yes | yes | yes | yes | yes | yes | N/A | complete |
| required | yes | yes | yes | yes | yes | yes | N/A | complete |
| target | yes | yes | yes | yes | yes | yes | N/A | complete |

### Notes
- State keywords are fully covered. They lower to CSS pseudo-class selectors via `frame_codegen`.

## Effects (Top 25)

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| lift | yes | partial | yes | yes | partial | yes | N/A | partial |
| sink | yes | partial | yes | yes | partial | yes | N/A | partial |
| shift | yes | partial | yes | yes | partial | yes | N/A | partial |
| grow | yes | partial | yes | yes | partial | yes | N/A | partial |
| shrink | yes | partial | yes | yes | partial | yes | N/A | partial |
| tilt | yes | partial | yes | yes | partial | yes | N/A | partial |
| glow | yes | partial | yes | yes | partial | yes | N/A | partial |
| brighten | yes | partial | yes | yes | partial | yes | N/A | partial |
| dim | yes | partial | yes | yes | partial | yes | N/A | partial |
| press | yes | partial | yes | yes | partial | yes | N/A | partial |
| pop | yes | partial | yes | yes | partial | yes | N/A | partial |
| ring | yes | partial | yes | yes | partial | yes | N/A | partial |
| blur | yes | partial | yes | yes | partial | yes | N/A | partial |
| smooth | yes | partial | yes | yes | partial | yes | N/A | partial |
| fade | yes | partial | yes | yes | partial | yes | N/A | partial |
| scale | yes | partial | yes | yes | partial | yes | N/A | partial |
| rotate | yes | partial | yes | yes | partial | yes | N/A | partial |
| slide | yes | partial | yes | yes | partial | yes | N/A | partial |
| transition | yes | partial | yes | yes | partial | yes | N/A | partial |
| duration | yes | partial | yes | yes | partial | yes | N/A | partial |
| ease | yes | partial | yes | yes | partial | yes | N/A | partial |
| animation | yes | partial | yes | yes | partial | yes | N/A | partial |
| animate | yes | partial | yes | yes | partial | yes | N/A | partial |

### Notes
- Effects are accepted by the parser inside state blocks and animation blocks. Semantic validation does not yet exhaustively validate effect syntax or argument types against the registry. The CSS emitter (`frame_codegen`) handles most effects.

## Special / Structural

| Item | Parser | Semantic | LSP Completion | LSP Hover | Diagnostics | Zed | Runtime DOM | Status |
|------|--------|----------|----------------|-----------|-------------|-----|-------------|--------|
| component | yes | yes | yes | yes | yes | yes | N/A | complete |
| bind | yes | yes | yes | yes | yes | yes | N/A | complete |
| show | yes | yes | yes | yes | yes | yes | N/A | complete |
| when | yes | yes | yes | yes | yes | yes | N/A | complete |
| for | yes | yes | yes | yes | yes | yes | N/A | complete |
| in | yes | yes | yes | yes | yes | yes | N/A | complete |
| key | yes | yes | yes | yes | yes | yes | N/A | complete |
| on | yes | yes | yes | yes | yes | yes | N/A | complete |
| props | yes | yes | yes | yes | yes | yes | N/A | complete |
| state | yes | yes | yes | yes | yes | yes | N/A | complete |
| view | yes | yes | yes | yes | yes | yes | N/A | complete |
| slot | yes | yes | yes | yes | yes | yes | N/A | complete |
| include | yes | yes | yes | yes | yes | yes | N/A | complete |

## Actionable Gaps

1. **Runtime DOM `title` mapping** — Add `title` to `packages/runtime-dom/src/index.ts` `ELEMENT_TAGS`.
2. **Parser `declaration_kind()` component gap** — Add `component` to the match in `frame_parser/src/helpers.rs` or document it as a dedicated-rule keyword.
3. **Registry event classification** — `press`, `blur`, `focus`, `select`, `input` are treated as events by semantic/runtime but are classified as `Effect`, `StateKeyword`, or `Primitive` in the registry. Reclassify or add dual classification.
4. **Declaration shorthand** — `card`, `row`, `stack`, `grid`, `split`, `dock`, `overlay`, `center`, `button` are registry declarations that appear in UI blocks but cannot be used as single-line shorthand. Extend `looks_like_semantic_shorthand` or `is_semantic_ui_primitive` to cover declarations that are valid UI elements.
5. **Semantic value validation** — Most registry values are not individually validated by the semantic layer. Expand `semantic/constants.rs` and `semantic/ui.rs` to validate common value categories against the registry.
6. **Effect semantic validation** — Effect arguments (movement amounts, visual amounts, directions) are not exhaustively validated. Add structured validation for `lift`, `tilt`, `grow`, etc.

## Related Files

- Canonical registry: `crates/frame_core/src/language.rs`
- Parser helpers: `crates/frame_parser/src/helpers.rs`
- Semantic constants: `crates/frame_core/src/semantic/constants.rs`
- Semantic UI validation: `crates/frame_core/src/semantic/ui.rs`
- LSP completions: `crates/frame_lsp/src/completions/mod.rs`
- LSP hover: `crates/frame_lsp/src/hover/mod.rs`
- Zed grammar: `editors/zed/tree-sitter-frame/grammar.js`
- Zed highlights: `editors/zed/tree-sitter-frame/queries/highlights.scm`
- Runtime DOM: `packages/runtime-dom/src/index.ts`
- Grammar drift tests: `crates/frame_core/tests/grammar_drift.rs`
- Registry drift tests: `crates/frame_core/tests/registry_drift.rs`

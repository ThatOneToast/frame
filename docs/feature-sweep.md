# Feature Sweep Verification

Last verified: 2026-06-07 milestone continuation sweep — Phase 1-4 complete

## Runtime Maturity Sweep Status

| Area | Expected | Implemented | Tests |
| --- | --- | --- | --- |
| Accessibility semantics | Semantic primitives render with correct roles and ARIA | `action`→`button[type=button]`, `toggle`→`checkbox`, `image`/`avatar`→`alt`+`decoding=async`, `icon`→`aria-hidden`, `field`→`role=group`, `media`→`controls`, `composer`→`form[method=post]`, `label`→`aria-label` | 7 runtime tests |
| Keyboard activation | Interactive controls are keyboard reachable | `action` uses native `<button>` which supports Enter/Space activation | Button keyboard test |
| Disabled behavior | Disabled actions are reflected in DOM | Conditional `Property: disabled` sets `dom.disabled` via `setBooleanProperty` | Disabled action test |
| Event system | Events work correctly after conditional and list changes | Event listeners remain stable across text patches, conditional show/hide, and keyed list reorder | 4 runtime tests |
| Input/form behavior | `placeholder`, `readonly`, `disabled`, `label` work | `placeholder` and `label` via attributes, `readonly`/`disabled` via `booleanPropertyName` mapping, bindings sync both ways | 4 runtime tests |
| Component lifecycle | Mount, dispose, conditional show/hide, keyed reuse | `dispose` removes nodes/listeners, `Show` uses `hidden`, keyed lists preserve nodes/listeners | 3 runtime tests |
| Runtime diagnostics | Mount-time warnings for missing handlers, prop validation, debug output | `validateHandlers` warns in debug mode, `validateProps` throws on type mismatch, scheduler logs patch labels with component names | 3 runtime tests |
| Subscription cleanup | No duplicate subscriptions after rerender | Conditional render cleanup unsubscribes and re-subscribes correctly | Subscription test |
| Debug stats | Counters accurately reflect mounts, listeners, subscriptions, disposals | `getDebugStats` returns counters verified against actual behavior | Stats test |

## Validation Commands

This file should be updated with exact validation results after each sweep. The required full suite remains:

This tracker records the follow-up sweep after the compiler/LSP/Zed/CLI organization pass. Items are marked complete only when code and tests exist.

## Previous Sweep Status

| Area | Expected | Implemented | Missing before this pass | Fixed in this pass | Remaining TODO |
| --- | --- | --- | --- | --- | --- |
| Large file refactors | Split oversized compiler, parser, CSS, CLI, LSP modules | Compiler semantic/CSS/parser and LSP completion/hover modules are split | None found in the audited paths | No refactor needed | Continue keeping new modules small |
| CLI organization | `check`, `compile`, `build`, `dev`, `watch`, `format`, `emit-ir`, `emit-contracts`, `init`, `new`, `doctor` | Commands are present and routed through modular command files | `new web` built from the caller cwd; web starter imported JSON directly | `new web` now builds inside the new project root; `build` emits typed `app.ir.ts`; `check` validates multi-file projects | `dev` remains a watch-style command, not a full Vite launcher |
| CLI project setup | Starters use Frame-native UI syntax | Web starter uses `screen`, `card`, and `action`; Svelte starter keeps style-declaration flow | Runnable web starter had no typed IR module | Web starter imports `./generated/app.ir` | Add richer starter handler skeleton generation later |
| Compiler diagnostics | Source-mapped diagnostics teach Frame syntax | Browser words, unsafe sinks, URL attributes, missing styles, handlers, and accessibility cases diagnose | No critical compiler gap found | Added cross-file component validation, duplicate symbol, and shadow diagnostics | Continue improving diagnostic actionability |
| LSP diagnostics | Match compiler diagnostics | LSP returns parser and semantic diagnostics | Migration quick fixes lagged diagnostics | Added code actions for browser primitive/event migration, missing style skeletons, handler, state, and prop creation | Continue improving diagnostic actionability |
| LSP completions | UI primitives, events, refs, style words | Semantic primitive/event/ref completions exist | Primitive bodies returned broad view completions | Added primitive-aware body completions for `action`, `field`, inputs, and collections | Add more contextual cross-file completions |
| LSP hover docs | Explain Frame meaning | Hover docs cover primitives, events, unsafe sinks, style concepts | Some native concepts were shadowed by older knowledge docs | Added native-first hover docs for structural concepts and semantic primitives | Continue replacing old HTML/Svelte examples in secondary docs tables |
| LSP navigation | Definitions/references for styles and symbols | Same-file navigation exists for style, state, props, handlers | Cross-file include awareness remains limited | Added imported declaration, component, and grid navigation | Extend references across files |
| LSP Find All References | References for styles, handlers, state, props across includes | Same-file references only | No cross-file reference collection | Cross-file references for styles, handlers, state, props, and declarations; AST-aware filtering | None |
| Primitive property validation | Reject invalid properties per primitive | Global property validation only | `value bind` on text, `source` on panel, etc. accepted without error | Added `valid_properties_for_primitive` and `validate_primitive_specific_property` with teacher-like diagnostics | Extend to more primitives as usage grows |
| Golden IR tests | Structural IR assertions across multiple examples | Single runtime fixture only | No golden fixtures for common patterns | 9 `.frame` fixtures with 20 structural assertions covering nodes, bindings, events, styles, conditional rendering, and keyed lists | Add golden tests when new IR features are introduced |
| Workspace edits | Code actions create missing symbols in included files | Code actions generated in current file only | No workspace-level edits | `DocumentChanges` with `TextDocumentEdit` targets the correct included file for handlers, state, props, and styles | Extend to component creation when cross-file component refactoring is added |
| Zed grammar/highlights | Match current syntax | Grammar and sample set cover semantic syntax, refs, events, style bindings, advanced CSS | `field` and `style Style when $state` were not in grammar | Added `field` and conditional style alias grammar support | Add focused composer/feed/accessibility/unsafe samples |
| Markdown docs | Reflect current architecture | Core language, primitives, lowering, runtime, IR docs exist | IR docs did not explain typed TS module path | Updated IR spec and runtime README | Keep docs synchronized with implementation status |
| Tests | Rust and TS coverage for changed behavior | Existing Rust/TS tests cover parser, semantic, CLI, runtime | TypeScript drift fixture was narrow | Expanded runtime typed IR fixture; added parser/completion/hover/code-action coverage | Add generated schema/golden tests across more examples |

## IR and Runtime Type Contract

Expected:

- Rust IR serialization, generated JSON, generated TypeScript, and `packages/runtime-dom` types agree.
- Hand-written or generated fixtures should not use `as any`.
- TypeScript consumers should catch enum/default/capability drift.

Implemented:

- Rust serializes `value_type` as `"Text"`, `"Bool"`, `"Number"`, `"List"`, or `{ "Unknown": "..." }`.
- Rust serializes defaults as `{ "Text": "..." }`, `{ "Bool": false }`, `{ "Number": "0" }`, `"List"`, or `{ "Invalid": "..." }`.
- Runtime types mirror those shapes.
- `frame build` emits `app.ir.json` for stable serialization and `app.ir.ts` for TypeScript consumption.
- `app.ir.ts` uses `defineFrameIrDocument(... as const)` so TypeScript checks the literal IR shape.

Fixed in this pass:

- `implementations/frame-ide/src/main.ts` imports the generated typed IR module instead of importing JSON directly.
- Runtime IR arrays are read-only friendly so `as const` generated modules typecheck.
- Runtime DOM build includes `tests/generated-ir.fixture.ts`, which fails if props, state types, defaults, bindings, handlers, events, conditional rendering, style bindings, conditional styles, keyed lists, nested components, capabilities, or read-only arrays drift from `FrameIrDocument`.

Implemented:

- Rust serializes `value_type` as `"Text"`, `"Bool"`, `"Number"`, `"List"`, or `{ "Unknown": "..." }`.
- Rust serializes defaults as `{ "Text": "..." }`, `{ "Bool": false }`, `{ "Number": "0" }`, `"List"`, or `{ "Invalid": "..." }`.
- Runtime types mirror those shapes.
- `frame build` emits `app.ir.json` for stable serialization and `app.ir.ts` for TypeScript consumption.
- `app.ir.ts` uses `defineFrameIrDocument(... as const)` so TypeScript checks the literal IR shape.

Golden tests:

- `crates/frame_cli/tests/golden.rs` compiles 9 fixture `.frame` files and asserts IR structure.
- Fixtures cover: simple UI, field-input pattern, data collection, keyed list, conditional rendering, nested component, explicit style binding, action with event, and complete page.
- Each fixture validates primitive kinds, node names, style bindings, event names, state/prop types, bindings, conditional rendering branches, and list key expressions.
- These tests fail if the compiler changes IR shape without updating the fixtures.

Fixed in this pass:

- `implementations/frame-ide/src/main.ts` imports the generated typed IR module instead of importing JSON directly.
- Runtime IR arrays are read-only friendly so `as const` generated modules typecheck.
- Runtime DOM build includes `tests/generated-ir.fixture.ts`, which fails if props, state types, defaults, bindings, handlers, events, conditional rendering, style bindings, conditional styles, keyed lists, nested components, capabilities, or read-only arrays drift from `FrameIrDocument`.

Remaining TODO:

- Add a generated schema or stronger cross-language golden test when the IR stabilizes beyond version `1`.

## Semantic Syntax and Stylability

Implemented:

- Author-facing semantic primitives include `screen`, `panel`, `section`, `stack`, `row`, `grid`, `split`, `dock`, `overlay`, `scroll`, `action`, `link`, `menu`, `toolbar`, `tabs`, `field`, `input`, `editor`, `toggle`, `choice`, `select`, `composer`, `title`, `text`, `label`, `badge`, `avatar`, `icon`, `image`, `media`, `list`, `feed`, `data`, `item`, `empty`, `card`, `dialog`, and `popover`.
- Browser words in `view` diagnose with semantic alternatives.
- Each parsed UI element lowers with `kind`, `semantic_kind`, `render_kind`, `name`, and `style`.
- Automatic style lookup uses the node name.
- Explicit style binding uses `Name:StyleName`.
- Conditional style switching supports `style when $state = StyleName` and `style StyleName when $state`; both lower into `FrameIrCondition::Style` and runtime patches classes.
- Missing explicit style references produce semantic diagnostics.
- LSP navigation can resolve same-file style bindings.

Implemented:

- Property validation is now primitive-specific via `valid_properties_for_primitive` and `validate_primitive_specific_property`.
- Each primitive category has a whitelist of valid properties. Unknown properties produce teacher-like diagnostics.
- Common misuse patterns are caught: `value bind` on non-inputs, `source` on non-media, `placeholder` on actions, etc.
- Diagnostics explain the intent mismatch and suggest the correct primitive or child placement.
- Tests cover valid properties, invalid properties, and cross-file property validation.

Remaining TODO:

- Continue improving diagnostic actionability for cross-file errors.
- Add more contextual completions based on primitive kind and current cursor position.

## Runtime Examples

Fixed in this pass:

- Rewrote every file in `packages/runtime-dom/examples`.
- Renamed browser-shaped examples to `accessible-composer.frame`, `field-input.frame`, and `data-list.frame`.
- Added examples for `field`, keyed lists, nested components, automatic style lookup, explicit style binding, conditional style aliases, conditional rendering, bindings, and semantic action events.
- Verified every runtime example with `frame check`.
- Added regression tests that compile each example to IR and validate with `frame check`.

## Validation Commands

This file should be updated with exact validation results after each sweep. The required full suite remains:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Package checks should be run where scripts exist:

```bash
npm run build
npm test
```

## Test Summary — This Sweep

All commands passed.

- Rust: 71 LSP tests, 46 `frame_core` tests, 18 parser tests, 17 CLI tests (including 20 golden fixture assertions)
- TypeScript: 47 `runtime-dom` tests, 9 `frame-svelte` tests
- End-to-end: all 10 runtime examples verified with `frame check`; all examples build
- Tree-sitter: `generate`, `parse`, and highlight tests pass
- Zed extension: `cargo test` passes

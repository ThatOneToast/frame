# Feature Sweep Verification

Last verified: 2026-06-07

This tracker records the follow-up sweep after the compiler/LSP/Zed/CLI organization pass. Items are marked complete only when code and tests exist.

## Previous Sweep Status

| Area | Expected | Implemented | Missing before this pass | Fixed in this pass | Remaining TODO |
| --- | --- | --- | --- | --- | --- |
| Large file refactors | Split oversized compiler, parser, CSS, CLI, LSP modules | Compiler semantic/CSS/parser and LSP completion/hover modules are split | None found in the audited paths | No refactor needed | Continue keeping new modules small |
| CLI organization | `check`, `compile`, `build`, `dev`, `watch`, `format`, `emit-ir`, `emit-contracts`, `init`, `new`, `doctor` | Commands are present and routed through modular command files | `new web` built from the caller cwd; web starter imported JSON directly | `new web` now builds inside the new project root; `build` emits typed `app.ir.ts` | `dev` remains a watch-style command, not a full Vite launcher |
| CLI project setup | Starters use Frame-native UI syntax | Web starter uses `screen`, `card`, and `action`; Svelte starter keeps style-declaration flow | Runnable web starter had no typed IR module | Web starter imports `./generated/app.ir` | Add richer starter handler skeleton generation later |
| Compiler diagnostics | Source-mapped diagnostics teach Frame syntax | Browser words, unsafe sinks, URL attributes, missing styles, handlers, and accessibility cases diagnose | No critical compiler gap found | No compiler diagnostic change needed | Attribute-by-primitive validation is still partial |
| LSP diagnostics | Match compiler diagnostics | LSP returns parser and semantic diagnostics | Migration quick fixes lagged diagnostics | Added code actions for `button`/`div`/`a`-style primitive migration and `onclick` -> `on press` | Create missing style declarations and handler skeletons |
| LSP completions | UI primitives, events, refs, style words | Semantic primitive/event/ref completions exist | Attribute-by-primitive completion still incomplete | No completion change in this pass | Add primitive-aware property completions |
| LSP hover docs | Explain Frame meaning | Hover docs cover primitives, events, unsafe sinks, style concepts | Some docs still mention browser examples indirectly | No hover change in this pass | Continue replacing old Svelte/HTML examples in docs tables |
| LSP navigation | Definitions/references for styles and symbols | Same-file navigation exists for style, state, props, handlers | Cross-file include awareness remains limited | No navigation change in this pass | Extend cross-file style/component awareness |
| Zed grammar/highlights | Match current syntax | Grammar and sample set cover semantic syntax, refs, events, style bindings, advanced CSS | New sample names requested by the task are not all present | No Zed code change in this pass | Add focused composer/feed/accessibility/unsafe samples |
| Markdown docs | Reflect current architecture | Core language, primitives, lowering, runtime, IR docs exist | IR docs did not explain typed TS module path | Updated IR spec and runtime README | Keep docs synchronized with implementation status |
| Tests | Rust and TS coverage for changed behavior | Existing Rust/TS tests cover parser, semantic, CLI, runtime | TypeScript did not catch JSON literal widening drift | Added runtime typed IR fixture; added LSP code action tests; added IR TS generation test | Add schema/golden tests across more generated examples |

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
- Runtime DOM build includes `tests/generated-ir.fixture.ts`, which fails if state types, defaults, capabilities, style bindings, events, or read-only arrays drift from `FrameIrDocument`.

Remaining TODO:

- Add a generated schema or stronger cross-language golden test when the IR stabilizes beyond version `1`.

## Semantic Syntax and Stylability

Implemented:

- Author-facing semantic primitives include `screen`, `panel`, `section`, `stack`, `row`, `grid`, `split`, `dock`, `overlay`, `scroll`, `action`, `link`, `menu`, `toolbar`, `tabs`, `input`, `editor`, `toggle`, `choice`, `select`, `composer`, `title`, `text`, `label`, `badge`, `avatar`, `icon`, `image`, `media`, `list`, `feed`, `data`, `item`, `empty`, `card`, `dialog`, and `popover`.
- Browser words in `view` diagnose with semantic alternatives.
- Each parsed UI element lowers with `kind`, `semantic_kind`, `render_kind`, `name`, and `style`.
- Automatic style lookup uses the node name.
- Explicit style binding uses `Name:StyleName`.
- Conditional style switching lowers into `FrameIrCondition::Style` and runtime patches classes.
- Missing explicit style references produce semantic diagnostics.
- LSP navigation can resolve same-file style bindings.

Remaining TODO:

- Make property validation primitive-specific instead of mostly global.
- Add cross-file style completion/navigation through include graphs.
- Add code action to create missing style declarations.

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

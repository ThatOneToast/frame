# Frame IR Spec

IR version: `1`

Frame IR is the compiler contract consumed by renderers. It records Frame semantic primitives and renderer lowering metadata without making DOM elements the authoring model. `packages/runtime-dom` currently consumes IR version `1` for the Phase 4 browser renderer.

The stable serialized artifact is JSON from `frame emit-ir` or `frame build`:

```json
{
  "version": 1,
  "components": []
}
```

Runnable TypeScript projects should consume the generated `app.ir.ts` module from `frame build`, not import JSON directly. TypeScript widens JSON string literals to `string`, which loses the enum precision of fields such as `value_type: "Text"`. The generated TS module wraps the same JSON object with `defineFrameIrDocument(... as const)`, so compiler-emitted IR is checked against `FrameIrDocument` without `as any`.

## Document

`FrameIrDocument`

- `version`: IR version number
- `components`: component definitions

## Component

`FrameIrComponent`

- `name`: component name
- `props`: prop descriptors
- `state`: state descriptors
- `slots`: named slot descriptors with fallback nodes
- `nodes`: root view nodes
- `capabilities`: required renderer capabilities
- `source`: source span

## Props

`FrameIrProp`

- `name`
- `value_type`: `Text`, `Bool`, `Number`, `List`, or `Unknown`
- `readonly`: true for input props
- `binding`: `Input` or `TwoWayAllowed`
- `source`

## State

`FrameIrState`

- `name`
- `value_type`: `Text`, `Bool`, `Number`, `List`, or `Unknown`
- `default`: `Text`, `Bool`, `Number`, `List`, or `Invalid`
- `source`

## Slots

`FrameIrSlot`

- `name`
- `fallback`: fallback node list
- `source`

Slots are metadata until a renderer implements projection.

## Nodes

`FrameIrNode`

- `Element`
- `Text`
- `Component`
- `List`

## Element

`FrameIrElement`

- `kind`: abstract UI element kind
- `semantic_kind`: author-facing Frame primitive
- `render_kind`: renderer target hint, such as `button` for `action`
- `name`: node identity name
- `style`: automatic or explicit style binding
- `attributes`: literal or data-backed attributes
- `bindings`: two-way binding metadata
- `events`: event descriptors
- `conditions`: show, hidden, property, or style conditions
- `children`: child nodes
- `source`

Current Frame author-facing primitives in IR version `1`: `screen`, `panel`, `section`, `stack`, `row`, `grid`, `split`, `dock`, `overlay`, `scroll`, `action`, `link`, `menu`, `toolbar`, `tabs`, `field`, `input`, `editor`, `toggle`, `choice`, `select`, `composer`, `title`, `text`, `label`, `badge`, `avatar`, `icon`, `image`, `media`, `list`, `feed`, `data`, `item`, `empty`, `card`, `dialog`, and `popover`.

Browser element names are renderer lowering details. Author-facing `view` syntax diagnoses browser words such as `button`, `div`, and `a` and suggests semantic primitives such as `action`, `panel`, and `link`.

Current global attributes are represented as ordinary `attributes`: `id`, `class`, `title`, `hidden`, `tabindex`, `role`, `part`, `slot`, `contenteditable`, `draggable`, `spellcheck`, `translate`, `dir`, `lang`, `data-*`, and `aria-*`. URL attributes are also ordinary attributes with security diagnostics: `href`, `src`, `srcset`, `poster`, `action`, `download`, `target`, and `rel`.

## Text

`FrameIrText`

- `value`: literal text or data reference
- `source`

Text renderers must escape data-backed text by default.

## Component Invocation

`FrameIrComponentInvocation`

- `name`
- `arguments`: literal, data reference, or bind reference
- `source`

## List

`FrameIrList`

- `item`: scoped loop item name
- `collection`: data reference name
- `key`: optional data reference for stable identity
- `children`: repeated template nodes
- `source`

Keyed lists use identity reuse and may move existing item DOM into the requested order. Non-keyed lists use positional update behavior.

The DOM renderer preserves keyed item node identity, listeners, and nested component state when keys are reused. Non-keyed lists reuse positions and are documented as positional, not identity-preserving.

## Events

`FrameIrEvent`

- `event`
- `modifiers`
- `handler`
- `source`

Handlers are external references.

Current DOM event modifiers include key/control filters and DOM listener behavior modifiers: `enter`, `escape`, `tab`, `space`, `ctrl`, `shift`, `alt`, `meta`, arrow keys, `prevent`, `stop`, `once`, `capture`, and `passive`.

## Conditions

`FrameIrCondition`

- `Show`: conditional rendering
- `Hidden`: visibility intent
- `Property`: conditional boolean property
- `Style`: conditional style switch

## Capability Flags

- `ConditionalRendering`
- `ConditionalStyles`
- `EventBinding`
- `TwoWayBinding`
- `ComponentComposition`
- `SlotContent`
- `ListRendering`

Renderers should reject IR containing capabilities they do not support.

## TypeScript Mapping

`packages/runtime-dom/src/ir.ts` is the TypeScript mirror for IR version `1`. Arrays are typed as `readonly` so generated `as const` IR modules can typecheck directly. The runtime does not mutate IR.

State value types serialize as strings: `"Text"`, `"Bool"`, `"Number"`, `"List"`, or `{ "Unknown": "..." }`.

State defaults serialize as tagged values: `{ "Text": "..." }`, `{ "Bool": false }`, `{ "Number": "0" }`, `"List"`, or `{ "Invalid": "..." }`.

Capability flags serialize as strings such as `"EventBinding"` and `"ConditionalStyles"`.

## Runtime Diagnostics

IR `source` spans are used by the DOM runtime for contextual `FrameDomError` messages. Runtime diagnostics currently cover invalid component lookup, invalid state access, invalid list sources, missing handlers, handler exceptions, unsafe URL updates, and patch failures. These diagnostics do not add new IR node types; they consume existing `source` metadata.

## Golden Fixture Tests

The compiler maintains golden IR fixtures in `crates/frame_cli/tests/fixtures/` to prevent accidental IR drift. Each fixture is a `.frame` source file compiled to IR, and the test suite asserts structural properties such as:

- Primitive `kind`, `semantic_kind`, and `render_kind` per node
- Node `name` and `style` bindings
- Event `event` names and `handler` references
- State `value_type` and prop `binding` modes
- Two-way binding `path` expressions
- Conditional rendering `Show` branches and `Style` conditions
- Keyed list `key` expressions and `collection` references

These tests do not require exact JSON string snapshots. They assert the IR structure through typed fields, so formatting or field-order changes do not break tests. Add a new fixture when a new IR feature is introduced.

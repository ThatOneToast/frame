# Frame IR Spec

IR version: `1`

Frame IR is the compiler contract consumed by renderers. It is renderer-neutral and contains no DOM or browser concepts. `packages/runtime-dom` currently consumes IR version `1` for the Phase 4 browser renderer.

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
- `name`: node identity name
- `style`: automatic or explicit style binding
- `attributes`: literal or data-backed attributes
- `bindings`: two-way binding metadata
- `events`: event descriptors
- `conditions`: show, hidden, property, or style conditions
- `children`: child nodes
- `source`

Current DOM renderers support these element kinds in IR version `1`: `a`, `article`, `audio`, `button`, `canvas`, `caption`, `col`, `colgroup`, `dd`, `details`, `dialog`, `div`, `dl`, `dt`, `fieldset`, `footer`, `form`, `h1` through `h6`, `header`, `img`, `input`, `label`, `legend`, `li`, `main`, `meter`, `nav`, `ol`, `optgroup`, `option`, `output`, `p`, `path`, `picture`, `progress`, `section`, `select`, `source`, `span`, `summary`, `svg`, `table`, `tbody`, `td`, `textarea`, `tfoot`, `th`, `thead`, `tr`, `track`, `ul`, and `video`, plus Frame aliases such as `link`, `image`, `card`, `panel`, `row`, `stack`, `grid`, and `area`.

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

## Runtime Diagnostics

IR `source` spans are used by the DOM runtime for contextual `FrameDomError` messages. Runtime diagnostics currently cover invalid component lookup, invalid state access, invalid list sources, missing handlers, handler exceptions, unsafe URL updates, and patch failures. These diagnostics do not add new IR node types; they consume existing `source` metadata.

# Language Redesign

Frame's semantic UI milestone replaces browser-tag authoring with intent-first primitives. HTML and DOM remain important renderer targets, but Frame source now describes interface intent, accessibility, and developer goals first.

The canonical language registry in `crates/frame_core/src/language.rs` is the single source of truth for all Frame language concepts. Parser, semantic model, LSP, Zed extension, and diagnostics all consume this registry. To add a new primitive or property, update the registry first.

Runtime, SSR, hydration, routing, portals, suspense, and async component work are paused for this milestone.

## Goal

Frame should feel like a UI language, not HTML written with different punctuation.

The authoring model should prioritize:

- intent: what the user can do or understand
- semantics: what role a node has in the interface
- accessibility: what assistive technology should receive
- contracts: what external state and handlers must exist
- renderer independence: what the compiler means before a DOM renderer maps it

The DOM remains the first renderer target. The language should not expose DOM details unless a feature genuinely has no higher-level equivalent.

## Research Notes

### HTML

HTML is broad, durable, accessible when used correctly, and close to browser behavior. Its weakness as a primary Frame syntax is that it mixes author intent with platform mechanism. `button`, `a`, `textarea`, `section`, `nav`, `table`, `tr`, and `td` describe implementation elements more than product intent.

Lesson: keep HTML as a complete mapping target and escape hatch, but do not make HTML tag names the default vocabulary for app authors.

### JSX and React

JSX makes UI construction composable, but it inherits HTML names, exposes JavaScript expressions inline, and often pushes accessibility into attributes. React's component model is flexible, but the language center is still JavaScript.

Lesson: keep component composition and explicit event/data contracts. Do not adopt inline scripting or make framework component calls the internal model.

### Vue

Vue's template syntax gives structure, directives, and binding shorthands. It is approachable, but its directive layer still sits on top of DOM elements and framework-specific reactivity.

Lesson: Frame can use declarative conditions, loops, and bindings, but should express those as compiler-owned constructs rather than DOM directives.

### Svelte

Svelte is concise and compiler-driven. Its event, binding, and control-flow syntax is productive, but it remains a component framework around HTML templates plus script blocks.

Lesson: compiler ownership is valuable. Frame should keep external scripting contracts instead of embedding scripts in component declarations.

### SwiftUI

SwiftUI is intent-heavy and renderer-mediated. `Button`, `TextField`, `List`, `NavigationStack`, and modifiers describe what UI is, not how HTML is shaped. Its risk is deep host-language coupling and hidden platform differences.

Lesson: semantic primitives are a strong fit for Frame. Frame should document renderer mappings explicitly so behavior does not become magical.

### Jetpack Compose

Compose treats UI as composable functions with semantic modifiers, layout primitives, state, and accessibility support. It is expressive, but tied to Kotlin and imperative host-language control flow.

Lesson: layout and accessibility semantics can be first-class without exposing browser concepts. Frame should keep declarative source and generated contracts.

### Flutter

Flutter's widget catalog is semantic and cross-platform. It has strong layout primitives, but deeply nested widget trees can become verbose and style-heavy.

Lesson: Frame should provide high-level primitives and concise blocks, while avoiding nested wrapper noise.

### Slint

Slint is purpose-built UI syntax with properties, callbacks, layouts, and renderer targets. It demonstrates the value of a dedicated UI language and clear compiler contracts.

Lesson: Frame should keep `.frame` as the source of UI structure and generate renderer contracts instead of relying on host framework templates.

### Figma Concepts

Figma designers think in frames, components, variants, constraints, auto layout, overlays, and named interaction flows. These map better to developer goals than raw tags.

Lesson: Frame should borrow concept names where they clarify intent: panel, stack, variant, overlay, component slots, constraints, and reusable primitives.

## HTML Leakage Analysis

Frame currently exposes browser concepts in several places:

- raw element names: `div`, `span`, `a`, `button`, `input`, `textarea`, `select`, `form`, `nav`, `section`, `article`, `table`, `tr`, `td`, `th`, headings, media, SVG
- raw attributes: `href`, `target`, `rel`, `src`, `alt`, `type`, `value`, `checked`, `selected`, `placeholder`, `method`, `action`
- raw accessibility attributes: `aria-label`, `aria-labelledby`, `aria-*`, `role`
- raw table structure: `table`, `thead`, `tbody`, `tr`, `td`, `th`, `caption`
- raw form structure: `form`, `label`, `input`, `textarea`, `select`, `option`, submit/reset details
- raw event names and browser event modifiers: `click`, `keydown`, `submit`, `input`, `change`, `prevent`
- layout vocabulary that mirrors CSS: `grid`, `columns`, `rows`, `display flex`, `display grid`, `flex direction`, `flex grow`
- URL and DOM sink attributes that require platform-specific safety rules

### Keep

These remain valid author-facing Frame concepts:

- URL-bearing attribute modeling and validation
- `data-*` for integration points
- external handler references with `@handler`
- data references with `$value`
- explicit unsafe raw HTML syntax, when implemented as an unsafe capability
- renderer capability diagnostics

### Internal Only

These may still exist in compiler/runtime lowering, but should not be required in Frame source:

- DOM tags such as `button`, `a`, `textarea`, `div`, `span`, `table`, `tr`, and `td`
- DOM attributes such as `href`, `src`, `target`, and `rel`
- generated ARIA attributes and roles
- native form element behavior
- CSS flexbox and CSS Grid implementation details

### Remove

These are removed from author-facing UI syntax in Phase 1:

- raw HTML element names in `view`
- `href`, `src`, `target`, `rel`, `role`, and common `aria-*` attributes
- table row/cell authoring for data presentation
- form submission authoring through raw DOM form behavior
- `button`/`a`/`textarea` as the preferred action/link/editor model
- unsafe raw HTML syntax, explicitly named unsafe

### Abstracted

These should usually be expressed through higher-level Frame primitives:

- `button` as `action`
- `a` as `link`
- `textarea` as `editor`
- `input type checkbox` as `toggle`
- `select` as `select` or `choice`
- `nav` as `menu` or `toolbar` depending on intent
- `section` and `article` as `panel`, `card`, `feed`, or `content`
- `div` and `span` as `panel`, `text`, `stack`, `flow`, or renderer-neutral fragments
- labels and common ARIA naming as `label`, `description`, `hint`, and primitive names
- table row/cell structure as `data` or `table` fields and records

### Should Be Redesigned

These should not be the primary authoring experience:

- authoring layout through CSS terms such as flexbox/grid tracks
- requiring `aria-label`, `aria-labelledby`, and `role` for common controls
- requiring table internals for data presentation
- requiring low-level form element choice before describing the data being collected
- treating `href`, `target`, and `rel` as the only way to express navigation intent
- using generic containers as the default way to organize UI

## Proposed Direction

Frame source should prefer semantic primitives:

```frame
component SettingsPanel {
  state {
    enabled bool = true
    theme text = "system"
  }

  view {
    panel Settings {
      heading "Preferences"

      toggle Enabled {
        label "Enable notifications"
        checked bind $enabled
      }

      choice Theme {
        label "Theme"
        options "System", "Light", "Dark"
        selected bind $theme
      }

      action Save {
        label "Save changes"
        on press @saveSettings
      }
    }
  }
}
```

The compiler should store the primitive kind, semantic name, state reads, handler references, accessibility metadata, and renderer mapping separately. The DOM renderer can map `action` to `button`, `toggle` to checkbox input, and `choice` to select/radio/listbox depending on configuration and capabilities.

## Phase 1 Implementation

Frame has no public compatibility requirement yet, so Phase 1 does not preserve old HTML-like UI syntax as author-facing syntax. Existing style declarations may still use names such as `button PrimaryButton` as visual styles, but `button Send` inside `view` is no longer valid semantic UI.

The compiler records each primitive as semantic intent and lowers to target-specific render kinds later. See `docs/semantic-lowering.md`.

## LSP Requirements

The LSP should teach Frame intent:

- hover for `action`: "A user-triggered command. Maps to an accessible control for the target renderer."
- hover for `link`: "Navigation to another resource or route. Requires a destination or route contract."
- hover for `editor`: "Multi-line text entry with label and value contract."
- completions should list semantic primitives before raw HTML elements
- diagnostics should explain missing intent, not just missing attributes
- code actions should suggest `action Save` instead of `button Save` when appropriate

## Final Report

### HTML Leakage Analysis

Frame currently exposes raw HTML elements, DOM attributes, ARIA attributes, table structure, form controls, browser events, CSS layout terms, and unsafe DOM sinks. Full DOM coverage should remain available, but it should be a mapping layer and escape hatch rather than the primary language.

### Proposed UI Primitives

Use primitives such as `action`, `link`, `menu`, `panel`, `field`, `editor`, `input`, `dialog`, `card`, `list`, `feed`, `data`, `toolbar`, `tabs`, `stack`, `dock`, `grid`, `scroll`, `choice`, `select`, and `toggle`. Each primitive should define meaning, accessibility behavior, compiler IR, and default renderer mappings.

### Proposed Layout System

Use `dock`, `stack`, `flow`, `grid`, `overlay`, `scroll`, and `split` as intent-first layout primitives. Avoid exposing flexbox/grid terminology in the primary syntax. Keep advanced CSS layout controls in the styling layer and escape hatches.

### Proposed Form System

Use intent-based controls and groups: `field`, `input`, `editor`, `toggle`, `choice`, `select`, and `action`. Labels, hints, descriptions, validation messages, required state, disabled state, and bindings should be first-class and accessible by default.

## Style Inheritance

Frame supports style inheritance via the `extends` keyword. A child declaration inherits all properties from its base and can override specific ones.

### Syntax

```frame
card DashboardPanel {
  padding medium
  radius medium
  border soft
  color text-primary
}

card MetricCard extends DashboardPanel {
  gap small
}

card ChartPanel extends DashboardPanel {
  gap medium
}
```

### Rules
- Base and child must have the same declaration kind (card extends card, grid extends grid)
- Child facts override base facts by normalized property path (see the Semantic Styling Overhaul section)
- Multi-level inheritance is supported (A extends B extends C)
- Cycles are detected and reported as errors
- Unknown base styles are reported as errors
- Empty bodies are supported: `card Foo extends Bar { }`

### CSS Output
The generated CSS for a child includes both base and overridden properties:
```css
.fr-DashboardPanel { padding: 1rem; border-radius: 0.625rem; ... }
.fr-MetricCard { padding: 1rem; border-radius: 0.625rem; ... gap: 0.5rem; }
```

### Migration Strategy

Use semantic primitives now. Keep browser terms internal to compiler/runtime lowering and explicit unsafe escape hatches.

## Semantic Styling Overhaul (2026-06)

Frame is not CSS shorthand. Frame is a semantic UI language that compiles into
efficient platform output. The styling pipeline now runs through a normalized
style layer in `crates/frame_core/src/style/`:

```text
.frame source
  -> parser AST
  -> semantic validation
  -> normalized style facts (property paths)
  -> token/theme contract resolution
  -> motion expansion + recipes/variants
  -> backend selection (semantic | atomic)
  -> generated.css + generated.ts contracts
```

### Tokens are contracts

`tokens <namespace> { ... }` declares typed design values. The default
manifest (spacing, radii, surfaces, colors, shadows, glows, gradients,
breakpoints, containers) lives in `frame_core::style::tokens` and any entry
can be overridden:

```frame
tokens default {
  color text #f5f5f5
  surface panel #171722
  space md 1rem
  radius lg 1rem
  breakpoint tablet 48rem
  container content 64rem
}
```

Reference tokens anywhere a value is expected with `token(kind.name)`:

```frame
card Panel {
  background token(surface.panel)
  padding token(space.md)
  radius token(radius.lg)
}
```

Unknown tokens, kinds, and breakpoints produce did-you-mean diagnostics
("Unknown breakpoint `desktoop`. Did you mean `desktop`?").

### Themes are scoped

```frame
theme dark uses default {
  surface app #101014
  color main #f5f5f5
}

theme light uses default {
  surface app #ffffff
  color main #111111
}
```

The first theme binds to `:root` as the document default; every theme emits a
`[data-frame-theme="name"]` scope. Generated TypeScript exports `themes`,
`defaultTheme`, and `applyTheme()`.

### Layout is intent-based

```frame
layout DashboardShell {
  shell {
    sidebar left fixed 18rem
    main fluid
    inspector right clamp(20rem, 28vw, 28rem)
  }
  gap large
  density comfortable
  below tablet { shell stacked }
}
```

Shell regions lower to a grid with named areas; children attach by source
order or `data-frame-section="name"`. Advanced grids remain available through
`grid`/`tracks` for full track control — that path is the escape hatch, not
the default.

### Motion is semantic

```frame
motion Pressable {
  enter fade up soft
  hover lift sm
  active press
  focus ring accent
  duration normal
  easing smooth
}

button PrimaryButton {
  background accent
  motion Pressable
}
```

Motions expand at compile time into the declarations that reference them;
explicit state effects win by property path. Custom `keyframes` remain for
advanced cases.

### Recipes and variants

```frame
recipe Button {
  base {
    align center
    gap small
    radius medium
    motion Pressable
  }
  variant tone {
    primary { background token(color.accent) }
    ghost { background transparent }
  }
}
```

compiles to `.fr-Button`, `.fr-Button--tone-primary`, `.fr-Button--tone-ghost`
plus a typed `recipes` export in generated TypeScript.

### Property-path inheritance

`extends` now merges by hierarchical property path (`border`, `border.width`,
`layout.display`, `motion.transition.duration`) instead of the first statement
word. Re-declaring `border` supersedes an inherited `border.width`; declaring
`border width large` after an inherited `border accent` refines it instead of
clobbering it. `surface` and `background` share the `background` path, so they
override each other correctly.

### Resets and backends

Broad `[class*="fr-"]` selectors and `!important` are gone; resets live in an
explicit `@layer frame-reset` that targets generated classes. CSS generation
supports two backends:

```bash
frame build --css-backend semantic   # default: one rule per class
frame build --css-backend atomic     # experimental: dedupe shared declarations
```

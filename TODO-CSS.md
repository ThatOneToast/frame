# TODO-CSS.md

Frame styling should expose the power of CSS through structured, discoverable syntax. This tracker catalogs remaining styling work while preserving a clear advanced escape hatch for rare cases.

Do not mark an item complete until parser support, semantic validation, codegen, docs, and tests exist.

## Current Baseline

Existing Frame styling already covers a useful subset:

- [x] `grid`
- [x] `area`
- [x] `card`
- [x] `stack`
- [x] `row`
- [x] `button`
- [x] `text`
- [x] `center`
- [x] `split`
- [x] `overlay`
- [x] `dock`
- [x] tokens
- [x] colors
- [x] gradients
- [x] keyframes
- [x] animations
- [x] responsive blocks
- [x] container query blocks
- [x] advanced CSS escape hatch

## Display and Box Model

- [x] `display block`
- [x] `display inline`
- [x] `display inline-block`
- [x] `display flex`
- [x] `display inline-flex`
- [x] `display grid`
- [x] `display inline-grid`
- [x] `display contents`
- [x] `display none`
- [x] `box content`
- [x] `box border`
- [x] `visibility visible`
- [x] `visibility hidden`
- [x] `visibility collapse`

## Layout

- [x] grid layout
- [x] named columns
- [x] named areas
- [x] app shell tracks
- [x] responsive card grids
- [ ] full CSS grid line placement
- [x] `subgrid` through `supports subgrid { grid Name { columns subgrid } }`
- [ ] masonry-like layout strategy
- [x] flex grow/shrink/basis
- [x] flex wrap
- [x] flex direction
- [ ] order
- [ ] float and clear escape coverage
- [ ] multi-column layout
- [ ] containment helpers

## Positioning

- [x] relative positioning
- [x] absolute positioning presets
- [x] sticky positioning
- [x] z-layer presets
- [ ] fixed positioning
- [ ] logical inset values
- [ ] anchor positioning research
- [ ] scroll-driven positioning patterns

## Spacing and Sizing

- [x] padding
- [x] margin
- [x] targeted padding and margin
- [x] width/height presets
- [x] min/max sizing
- [x] percentage sizing
- [x] logical sizing: inline/block
- [ ] viewport units: `svh`, `lvh`, `dvh`, `svw`, `lvw`, `dvw`
- [ ] clamp/min/max helpers
- [ ] aspect ratio helpers
- [ ] intrinsic sizing helpers

## Typography

- [x] font family presets
- [x] font size presets
- [x] font weight presets
- [x] line height
- [x] letter spacing
- [x] text color
- [x] truncation
- [x] wrapping
- [x] casing
- [x] text align full coverage
- [x] text decoration
- [x] text transform full coverage
- [x] white-space controls
- [x] word-break controls
- [x] hyphenation
- [ ] font feature settings
- [ ] font variation settings
- [ ] web font declarations

## Colors and Surfaces

- [x] named color intent
- [x] custom color tokens
- [x] custom gradient tokens
- [x] surface presets
- [x] glass surfaces
- [x] layered gradients
- [ ] color spaces: `oklch`, `lab`, `lch`
- [ ] color mixing helpers
- [ ] light/dark theme variants
- [ ] high-contrast variants
- [ ] forced-colors behavior

## Borders and Outlines

- [x] border intent
- [x] directional borders
- [x] radius presets
- [x] outline none
- [x] focus rings
- [x] border style full coverage
- [x] border width scale
- [x] outline offset
- [ ] image borders

## Effects

- [x] shadow presets
- [x] glow presets
- [x] hover lift
- [x] brighten/dim
- [x] blur
- [x] press effect
- [ ] filter full coverage
- [ ] backdrop-filter full coverage
- [ ] blend modes
- [ ] masks and clipping
- [ ] opacity transitions

## Transforms, Transitions, and Animation

- [x] transition presets
- [x] duration
- [x] easing
- [x] keyframes
- [x] structured animation blocks
- [ ] transform origin
- [x] translate helpers through intent verbs (`lift`, `sink`, `shift`)
- [x] rotate helpers through intent verbs (`tilt left`, `tilt right`)
- [x] scale helpers through intent verbs (`grow`, `shrink`, `pop`)
- [ ] skew helpers
- [ ] 3D transform helpers
- [ ] scroll-driven animations
- [ ] view transitions research
- [ ] reduced motion variants

## Responsive and Environment

- [x] `below`
- [x] `above`
- [x] `between`
- [x] container query blocks
- [ ] orientation queries
- [ ] pointer/hover capability queries
- [ ] prefers-color-scheme
- [ ] prefers-reduced-motion
- [ ] prefers-contrast
- [ ] dynamic viewport units
- [ ] print styles

## Scrolling

- [x] overflow helpers
- [x] scroll y
- [x] dense scrollbar intent
- [ ] scroll snap
- [ ] overscroll behavior
- [ ] scroll behavior
- [ ] scrollbar styling full coverage
- [ ] scroll margin/padding

## Interaction Styling

- [x] `hover`
- [x] `focus`
- [x] `active`
- [x] `disabled`
- [x] `focus-visible`
- [x] `focus-within`
- [x] `checked`
- [x] `invalid`
- [x] `required`
- [ ] `placeholder`
- [ ] `selection`
- [x] `target`

`placeholder` and `selection` are deferred until Frame supports pseudo-element blocks with normal styling declarations, not only effect-style state blocks.

## Advanced CSS and Escape Hatches

- [x] scoped `advanced { css "property" value }`
- [ ] unsafe/global CSS escape hatch policy
- [x] typed `supports` predicates
- [x] style groups and `style-order` backed by `@layer`
- [ ] `@scope`
- [ ] custom property registration
- [ ] custom media aliases
- [x] cascade layer strategy through `style-group` and `style-order`

## Documentation Requirements

Each completed feature needs:

- [ ] syntax example
- [ ] generated CSS example
- [ ] semantic validation rules
- [ ] LSP completion/hover behavior
- [ ] at least one test

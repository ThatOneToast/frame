; Top-level declaration headers.
(declaration
  kind: (declaration_keyword) @keyword
  name: (declaration_name) @type)

; Imports.
(include
  keyword: (include_keyword) @keyword
  target: (include_target) @string.special)

; Nested interaction state blocks.
(state_block
  name: (state_keyword) @keyword)

(gradient_block
  "gradient" @keyword
  name: (identifier) @constant)

(section_block
  "section" @keyword
  name: (identifier) @constant)

(advanced_block
  name: (special_block_keyword) @keyword)

(animation_block
  "animation" @keyword
  name: (identifier) @constant)

(responsive_block
  kind: _ @keyword)

(responsive_block
  breakpoint: (identifier) @constant)

(responsive_block
  start: (identifier) @constant
  end: (identifier) @constant)

(container_block
  "container" @keyword
  name: (identifier) @constant)

(keyframe_block
  selector: (keyframe_selector) @keyword)

(string) @string

; Intent words and effects.
(statement
  property: (property_keyword) @property)

(statement
  property: (effect_keyword) @property)

; Statement arguments.
(statement value: (value (identifier) @constant))
(statement value: (value (raw_value) @constant))
(statement value: (value (percentage) @number))
(statement value: (value (number) @number))
(statement value: (value (color_literal) @string.special))

(comment) @comment

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

(string) @string

; Intent words and effects.
(statement
  property: (property_keyword) @property)

(statement
  property: (effect_keyword) @property)

; Statement arguments.
(statement value: (value (identifier) @constant))
(statement value: (value (percentage) @number))
(statement value: (value (number) @number))
(statement value: (value (color_literal) @string.special))

(comment) @comment

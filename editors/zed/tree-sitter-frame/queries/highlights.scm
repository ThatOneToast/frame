; Top-level declaration headers.
(declaration
  kind: (declaration_keyword) @keyword
  name: (declaration_name) @type)

; Nested interaction state blocks.
(state_block
  name: (state_keyword) @keyword)

; Intent words and effects.
(statement
  property: (property_keyword) @property)

(statement
  property: (effect_keyword) @property)

; Statement arguments.
(statement
  value: (identifier) @constant)

(comment) @comment

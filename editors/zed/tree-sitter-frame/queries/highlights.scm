; Highlight queries aligned with editors/zed/tree-sitter-frame/grammar.js.
; When grammar nodes change, keep both highlights.scm files in sync.
; Top-level declaration headers.
(declaration
  kind: (declaration_keyword) @keyword
  name: (declaration_name) @type)

(supports_block
  "supports" @keyword
  predicate: (support_predicate (identifier) @constant))

(style_group_block
  "style-group" @keyword
  name: (identifier) @constant)

(style_order
  "style-order" @keyword
  groups: (style_order_list (identifier) @constant))

; Experimental UI declarations.
(component_declaration
  "component" @keyword
  name: (declaration_name) @type)

(ui_state_block
  "state" @keyword)

(props_block
  "props" @keyword)

(prop_value
  name: (identifier) @variable
  type: (state_type) @type)

(state_value
  name: (identifier) @variable
  type: (state_type) @type)

(view_block
  "view" @keyword)

(slot_block
  "slot" @keyword
  name: (identifier) @type)

(ui_element
  kind: (ui_element_keyword) @keyword
  name: (ui_node_name) @type)

(ui_element
  style: (style_name) @constant)

(ui_element_shorthand
  kind: (ui_element_keyword) @keyword
  name: (ui_node_name) @type)

(ui_element_shorthand
  style: (style_name) @constant)

(component_invocation
  name: (component_invocation_name) @type)

(component_argument
  name: (identifier) @property)

(component_argument
  "bind" @keyword)

(for_loop
  "for" @keyword
  item: (identifier) @variable
  "in" @keyword)

(for_loop
  "key" @keyword
  key: (data_ref) @variable)

(ui_text
  "text" @keyword)

(event_binding
  "on" @keyword
  event: (event_name) @function)

(event_binding
  modifier: (event_modifier) @constant)

(value_binding
  "value" @property
  "bind" @keyword)

(conditional_flag
  property: (ui_attribute_name) @property
  "when" @keyword)

(conditional_style
  "style" @property
  "when" @keyword
  style: (style_name) @constant)

(conditional_style
  "style" @property
  style: (style_name) @constant
  "when" @keyword
  condition: (data_ref) @variable)

(ui_property
  property: (ui_attribute_name) @property)

(data_ref) @variable
(handler_ref) @function
(boolean) @constant.builtin
(list_literal) @constant.builtin

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

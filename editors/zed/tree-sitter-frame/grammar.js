// Keyword lists in this grammar should stay aligned with
// crates/frame_core/src/language.rs. When adding new Frame syntax,
// update the canonical registry first, then sync these arrays.
/// <reference types="tree-sitter-cli/dsl" />

const DECLARATION_KEYWORDS = [
  "action",
  "area",
  "avatar",
  "badge",
  "button",
  "card",
  "center",
  "choice",
  "composer",
  "data",
  "dialog",
  "dock",
  "editor",
  "empty",
  "feed",
  "field",
  "grid",
  "html",
  "icon",
  "image",
  "input",
  "item",
  "keyframes",
  "label",
  "link",
  "list",
  "media",
  "menu",
  "motion",
  "overlay",
  "page-body",
  "panel",
  "popover",
  "recipe",
  "row",
  "screen",
  "scroll",
  "section",
  "select",
  "split",
  "stack",
  "tabs",
  "text",
  "theme",
  "title",
  "toggle",
  "tokens",
  "toolbar",
];

// UI element keywords cover both primitives and declarations that may appear
// inside a view block. Aligned with declaration_keywords() in the canonical registry,
// excluding keywords that have dedicated top-level grammar rules.
const UI_ELEMENT_KEYWORDS = [
  "action",
  "area",
  "avatar",
  "badge",
  "button",
  "card",
  "center",
  "choice",
  "composer",
  "data",
  "dialog",
  "dock",
  "editor",
  "empty",
  "feed",
  "field",
  "grid",
  "icon",
  "image",
  "input",
  "item",
  "label",
  "link",
  "list",
  "media",
  "menu",
  "motion",
  "overlay",
  "panel",
  "popover",
  "recipe",
  "row",
  "screen",
  "scroll",
  "section",
  "select",
  "split",
  "stack",
  "tabs",
  "text",
  "theme",
  "title",
  "toggle",
  "toolbar",
];

// UI property keywords aligned with the UiKeyword kind in the canonical registry,
// excluding literals that are hard-coded in the grammar rules (for, in, key, on,
// props, slot, state, view).
const UI_PROPERTY_KEYWORDS = [
  "class",
  "data-test-id",
  "decorative",
  "description",
  "download",
  "draft",
  "goto",
  "hidden",
  "hint",
  "id",
  "kind",
  "new-window",
  "options",
  "placeholder",
  "poster",
  "readonly",
  "rel",
  "selected",
  "show",
  "source",
  "sources",
  "style",
  "value",
];

const UI_EVENT_NAMES = [
  "change",
  "click",
  "close",
  "keydown",
  "keyup",
  "mouseenter",
  "mouseleave",
  "open",
  "pointerdown",
  "pointermove",
  "pointerup",
  "reset",
  "send",
  "submit",
];

const UI_EVENT_MODIFIERS = [
  "alt",
  "capture",
  "ctrl",
  "down",
  "enter",
  "escape",
  "left",
  "meta",
  "once",
  "passive",
  "prevent",
  "right",
  "shift",
  "space",
  "stop",
  "tab",
  "up",
];

const STATE_KEYWORDS = [
  "active",
  "checked",
  "disabled",
  "focus",
  "focus-visible",
  "focus-within",
  "hover",
  "invalid",
  "required",
  "target",
];

// Aligned with the Property kind in the canonical registry.
const PROPERTY_KEYWORDS = [
  "advanced",
  "align",
  "align-text",
  "anchor",
  "angle",
  "areas",
  "at",
  "background",
  "block-size",
  "border",
  "box",
  "case",
  "col",
  "color",
  "columns",
  "control",
  "corner",
  "css",
  "decoration",
  "delay",
  "direction",
  "display",
  "fill",
  "filter",
  "flex",
  "flow",
  "font",
  "gap",
  "gradient",
  "height",
  "hyphenate",
  "inline-size",
  "interactive",
  "iteration",
  "justify",
  "layout",
  "letter",
  "line",
  "margin",
  "max-block-size",
  "max-height",
  "max-inline-size",
  "max-width",
  "min-block-size",
  "min-height",
  "min-inline-size",
  "min-width",
  "nudge",
  "offset",
  "opacity",
  "outline",
  "overflow",
  "padding",
  "palette",
  "place",
  "play-state",
  "position",
  "radius",
  "rows",
  "scrollbar",
  "self",
  "shadow",
  "shape",
  "size",
  "span",
  "square",
  "surface",
  "theme",
  "tone",
  "tracks",
  "transform",
  "truncate",
  "type",
  "visibility",
  "weight",
  "whitespace",
  "width",
  "word-break",
  "wrap",
  "z",
];

// Aligned with the Effect kind in the canonical registry.
const EFFECT_KEYWORDS = [
  "animate",
  "animation",
  "blur",
  "brighten",
  "dim",
  "duration",
  "ease",
  "fade",
  "glow",
  "grow",
  "lift",
  "pop",
  "press",
  "ring",
  "rotate",
  "scale",
  "shift",
  "shrink",
  "sink",
  "slide",
  "smooth",
  "tilt",
  "transition",
];

module.exports = grammar({
  name: "frame",

  extras: ($) => [/[ \t]/, $.comment],

  word: ($) => $.identifier,

  conflicts: ($) => [
    [$.animation_block, $.effect_keyword],
    [$.gradient_block, $.property_keyword],
  ],

  rules: {
    source_file: ($) => repeat(choice($.include, $.supports_block, $.style_group_block, $.style_order, $.component_declaration, $.declaration, $._newline)),

    include: ($) =>
      seq(
        field("keyword", $.include_keyword),
        field("target", $.include_target),
        $._newline,
      ),

    declaration: ($) =>
      seq(
        field("kind", $.declaration_keyword),
        field("name", $.declaration_name),
        optional(seq("extends", field("base", $.declaration_name))),
        $.block,
      ),

    component_declaration: ($) =>
      seq(
        "component",
        field("name", $.declaration_name),
        $.component_body,
      ),

    component_body: ($) =>
      seq(
        "{",
        repeat(choice($._newline, $.props_block, $.ui_state_block, $.view_block, $.slot_block)),
        "}",
      ),

    props_block: ($) =>
      seq("props", "{", repeat(choice($._newline, $.prop_value)), "}"),

    prop_value: ($) =>
      seq(
        field("name", $.identifier),
        field("type", $.state_type),
        $._newline,
      ),

    ui_state_block: ($) =>
      seq("state", "{", repeat(choice($._newline, $.state_value)), "}"),

    state_value: ($) =>
      seq(
        field("name", $.identifier),
        field("type", $.state_type),
        "=",
        field("default", choice($.string, $.boolean, $.number, $.list_literal)),
        $._newline,
      ),

    state_type: (_) => choice("text", "string", "bool", "number", "list"),

    boolean: (_) => choice("true", "false"),

    view_block: ($) => seq("view", $.ui_block),

    slot_block: ($) =>
      seq("slot", field("name", $.identifier), $.ui_block),

    ui_block: ($) =>
      seq(
        "{",
        repeat(choice(
          $._newline,
          $.ui_element,
          $.ui_element_shorthand,
          $.for_loop,
          $.component_invocation,
          $.ui_text,
          $.event_binding,
          $.value_binding,
          $.conditional_flag,
          $.conditional_style,
          $.ui_property,
        )),
        "}",
      ),

    for_loop: ($) =>
      seq(
        "for",
        field("item", $.identifier),
        "in",
        field("collection", $.data_ref),
        optional(seq("key", field("key", $.data_ref))),
        $.ui_block,
      ),

    ui_element: ($) =>
      seq(
        field("kind", $.ui_element_keyword),
        optional(seq(field("name", $.ui_node_name), optional(seq(":", field("style", $.style_name))))),
        $.ui_block,
      ),

    ui_element_shorthand: ($) =>
      prec(1, seq(
        field("kind", $.ui_element_keyword),
        field("name", choice($.ui_node_name, $.string, $.data_ref)),
        optional(seq(":", field("style", $.style_name))),
        $._newline,
      )),

    component_invocation: ($) =>
      seq(
        field("name", $.component_invocation_name),
        "(",
        optional(seq($.component_argument, repeat(seq(",", $.component_argument)))),
        ")",
        $._newline,
      ),

    component_argument: ($) =>
      choice(
        seq(field("name", $.identifier), ":", field("value", choice($.data_ref, $.string, $.number, $.boolean, $.identifier))),
        seq(field("name", $.identifier), "bind", field("value", $.data_ref)),
      ),

    ui_text: ($) =>
      prec(2, seq("text", field("value", choice($.string, $.data_ref)), $._newline)),

    event_binding: ($) =>
      seq(
        "on",
        field("event", $.event_name),
        repeat(seq(".", field("modifier", $.event_modifier))),
        field("handler", $.handler_ref),
        $._newline,
      ),

    value_binding: ($) =>
      prec(2, choice(
        seq("value", "bind", field("value", $.data_ref), $._newline),
        seq("bind", field("value", $.data_ref), $._newline),
      )),

    ui_attribute_name: ($) => choice($.ui_property_keyword, $.identifier),

    conditional_flag: ($) =>
      seq(field("property", $.ui_attribute_name), "when", field("condition", $.data_ref), $._newline),

    conditional_style: ($) =>
      prec(1, choice(
        seq("style", "when", field("condition", $.data_ref), "=", field("style", $.style_name), $._newline),
        seq("style", field("style", $.style_name), "when", field("condition", $.data_ref), $._newline),
      )),

    ui_property: ($) =>
      choice(
        seq(field("property", $.ui_attribute_name), "bind", field("value", $.data_ref), $._newline),
        seq(field("property", $.ui_attribute_name), field("value", choice($.string, $.data_ref, $.handler_ref, $.number, $.boolean, $.identifier)), $._newline),
      ),

    supports_block: ($) =>
      seq(
        "supports",
        field("predicate", $.support_predicate),
        $.supports_body,
      ),

    support_predicate: ($) => seq($.identifier, repeat($.identifier)),

    supports_body: ($) =>
      seq("{", repeat(choice($._newline, $.declaration)), "}"),

    style_group_block: ($) =>
      seq(
        "style-group",
        field("name", $.identifier),
        $.supports_body,
      ),

    style_order: ($) =>
      seq(
        "style-order",
        field("groups", $.style_order_list),
        $._newline,
      ),

    style_order_list: ($) => seq($.identifier, repeat(seq(",", $.identifier))),

    block: ($) =>
      seq(
        "{",
        repeat(choice(
          $._newline,
          $.state_block,
          $.gradient_block,
          $.section_block,
          $.advanced_block,
          $.animation_block,
          $.responsive_block,
          $.container_block,
          $.keyframe_block,
          $.statement,
        )),
        "}",
      ),

    state_block: ($) =>
      seq(field("name", $.state_keyword), $.block),

    gradient_block: ($) =>
      seq("gradient", field("name", $.identifier), $.block),

    section_block: ($) =>
      seq("section", field("name", $.identifier), $.block),

    advanced_block: ($) =>
      seq(field("name", $.special_block_keyword), $.block),

    animation_block: ($) =>
      seq("animation", field("name", $.identifier), $.block),

    responsive_block: ($) =>
      choice(
        seq(field("kind", choice("below", "above")), field("breakpoint", $.identifier), $.block),
        seq(
          field("kind", "between"),
          field("start", $.identifier),
          field("end", $.identifier),
          $.block,
        ),
      ),

    container_block: ($) =>
      seq("container", field("name", $.identifier), $.block),

    keyframe_block: ($) =>
      seq(field("selector", $.keyframe_selector), $.block),

    statement: ($) =>
      seq(
        field("property", choice($.property_keyword, $.effect_keyword, $.identifier)),
        repeat(field("value", $.value)),
        $._newline,
      ),

    declaration_name: ($) => $.identifier,

    ui_node_name: ($) => $.identifier,

    style_name: ($) => $.identifier,

    component_invocation_name: (_) => /[A-Z][A-Za-z0-9_]*/,

    declaration_keyword: (_) => choice(...DECLARATION_KEYWORDS),

    ui_element_keyword: (_) => choice(...UI_ELEMENT_KEYWORDS),

    ui_property_keyword: (_) => choice(...UI_PROPERTY_KEYWORDS),

    event_name: (_) => choice(...UI_EVENT_NAMES),

    event_modifier: (_) => choice(...UI_EVENT_MODIFIERS),

    state_keyword: (_) => choice(...STATE_KEYWORDS),

    special_block_keyword: (_) => "advanced",

    property_keyword: (_) => choice(...PROPERTY_KEYWORDS),

    effect_keyword: (_) => choice(...EFFECT_KEYWORDS),

    include_keyword: (_) => "#include",

    include_target: (_) => /[^ \t\r\n]+/,

    value: ($) => choice($.string, $.color_literal, $.percentage, $.number, $.identifier, $.raw_value),

    string: (_) => /"[^"\r\n]*"/,

    identifier: (_) => /[A-Za-z_][A-Za-z0-9_-]*/,

    percentage: (_) => /[0-9]+%/,

    keyframe_selector: (_) => choice("from", "to", /[0-9]+%/),

    number: (_) => /[0-9]+/,

    list_literal: (_) => "[]",

    raw_value: (_) => /[^ \t\r\n{}]+/,

    color_literal: (_) => /#[0-9a-fA-F]{3}([0-9a-fA-F]{3})?([0-9a-fA-F]{2})?/,

    data_ref: (_) => /\$[A-Za-z_][A-Za-z0-9_-]*(\.[A-Za-z_][A-Za-z0-9_-]*)*/,

    handler_ref: (_) => /@[A-Za-z_][A-Za-z0-9_-]*/,

    comment: (_) => token(seq("//", /.*/)),

    _newline: (_) => /\r?\n/,
  },
});

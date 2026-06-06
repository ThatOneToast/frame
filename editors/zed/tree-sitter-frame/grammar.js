/// <reference types="tree-sitter-cli/dsl" />

const DECLARATION_KEYWORDS = [
  "tokens",
  "grid",
  "area",
  "card",
  "stack",
  "row",
  "button",
  "text",
  "center",
  "split",
  "overlay",
  "dock",
  "keyframes",
];

const STATE_KEYWORDS = [
  "hover",
  "focus",
  "focus-visible",
  "focus-within",
  "active",
  "disabled",
  "checked",
  "invalid",
  "required",
  "target",
];

const PROPERTY_KEYWORDS = [
  "columns",
  "rows",
  "flow",
  "gap",
  "display",
  "height",
  "width",
  "min-height",
  "max-height",
  "min-width",
  "max-width",
  "inline-size",
  "block-size",
  "min-inline-size",
  "max-inline-size",
  "min-block-size",
  "max-block-size",
  "surface",
  "background",
  "theme",
  "text",
  "color",
  "palette",
  "tone",
  "opacity",
  "padding",
  "anchor",
  "margin",
  "radius",
  "border",
  "shadow",
  "outline",
  "box",
  "visibility",
  "flex",
  "square",
  "place",
  "in",
  "col",
  "row",
  "span",
  "position",
  "offset",
  "z",
  "align",
  "justify",
  "font",
  "size",
  "weight",
  "case",
  "align-text",
  "decoration",
  "whitespace",
  "word-break",
  "hyphenate",
  "line",
  "letter",
  "transition",
  "duration",
  "ease",
  "animation",
  "animate",
  "delay",
  "iteration",
  "direction",
  "fill",
  "play-state",
  "opacity",
  "transform",
  "filter",
  "translate",
  "type",
  "angle",
  "stop",
  "corner",
  "at",
  "shape",
  "css",
];

const EFFECT_KEYWORDS = [
  "lift",
  "sink",
  "shift",
  "grow",
  "shrink",
  "tilt",
  "glow",
  "brighten",
  "dim",
  "blur",
  "press",
  "pop",
  "ring",
  "smooth",
  "fade",
  "scale",
  "rotate",
  "slide",
];

module.exports = grammar({
  name: "frame",

  extras: ($) => [/[ \t]/, $.comment],

  word: ($) => $.identifier,

  conflicts: ($) => [
    [$.animation_block, $.property_keyword],
  ],

  rules: {
    source_file: ($) => repeat(choice($.include, $.supports_block, $.style_group_block, $.style_order, $.declaration, $._newline)),

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
        $.block,
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

    declaration_keyword: (_) => choice(...DECLARATION_KEYWORDS),

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

    raw_value: (_) => /[^ \t\r\n{}]+/,

    color_literal: (_) => /#[0-9a-fA-F]{3}([0-9a-fA-F]{3})?([0-9a-fA-F]{2})?/,

    comment: (_) => token(seq("//", /.*/)),

    _newline: (_) => /\r?\n/,
  },
});

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
];

const STATE_KEYWORDS = ["hover", "focus", "active"];

const PROPERTY_KEYWORDS = [
  "columns",
  "rows",
  "gap",
  "height",
  "width",
  "surface",
  "background",
  "theme",
  "padding",
  "margin",
  "radius",
  "border",
  "shadow",
  "place",
  "in",
  "col",
  "row",
  "span",
];

const EFFECT_KEYWORDS = [
  "lift",
  "glow",
  "brighten",
  "dim",
  "blur",
  "press",
  "ring",
  "smooth",
];

module.exports = grammar({
  name: "frame",

  extras: ($) => [/[ \t]/, $.comment],

  word: ($) => $.identifier,

  rules: {
    source_file: ($) => repeat(choice($.declaration, $._newline)),

    declaration: ($) =>
      seq(
        field("kind", $.declaration_keyword),
        field("name", $.declaration_name),
        $.block,
      ),

    block: ($) =>
      seq(
        "{",
        repeat(choice($._newline, $.state_block, $.statement)),
        "}",
      ),

    state_block: ($) =>
      seq(field("name", $.state_keyword), $.block),

    statement: ($) =>
      seq(
        field("property", choice($.property_keyword, $.effect_keyword, $.identifier)),
        repeat(field("value", $.identifier)),
        $._newline,
      ),

    declaration_name: ($) => $.identifier,

    declaration_keyword: (_) => choice(...DECLARATION_KEYWORDS),

    state_keyword: (_) => choice(...STATE_KEYWORDS),

    property_keyword: (_) => choice(...PROPERTY_KEYWORDS),

    effect_keyword: (_) => choice(...EFFECT_KEYWORDS),

    identifier: (_) => /[A-Za-z_][A-Za-z0-9_-]*/,

    comment: (_) => token(seq("//", /.*/)),

    _newline: (_) => /\r?\n/,
  },
});

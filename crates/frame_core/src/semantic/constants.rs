pub(crate) const SEMANTIC_UI_PRIMITIVES: &[&str] = &[
    "screen", "panel", "section", "stack", "row", "grid", "split", "dock", "overlay", "scroll",
    "action", "link", "menu", "toolbar", "tabs", "field", "input", "editor", "toggle", "choice",
    "select", "composer", "title", "text", "label", "badge", "avatar", "icon", "image", "media",
    "list", "feed", "data", "item", "empty", "card", "dialog", "popover",
];

pub(crate) const BROWSER_UI_WORDS: &[&str] = &[
    "a", "article", "audio", "button", "canvas", "caption", "col", "colgroup", "dd", "details",
    "div", "dl", "dt", "fieldset", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header",
    "img", "legend", "li", "main", "meter", "nav", "ol", "optgroup", "option", "output", "p",
    "path", "picture", "progress", "source", "span", "summary", "svg", "table", "tbody", "td",
    "textarea", "tfoot", "th", "thead", "tr", "track", "ul", "video", "area",
];

pub(crate) const UI_EVENTS: &[&str] = &[
    "press",
    "send",
    "open",
    "close",
    "select",
    "click",
    "input",
    "change",
    "submit",
    "reset",
    "keydown",
    "keyup",
    "focus",
    "blur",
    "pointerdown",
    "pointerup",
    "pointermove",
    "mouseenter",
    "mouseleave",
];

pub(crate) const UI_EVENT_MODIFIERS: &[&str] = &[
    "enter", "escape", "tab", "space", "ctrl", "shift", "alt", "meta", "left", "right", "up",
    "down", "prevent", "stop", "once", "capture", "passive",
];

pub(crate) const URL_ATTRIBUTES: &[&str] = &["goto", "sources", "poster"];
pub(crate) const REMOVED_HTML_ATTRIBUTES: &[&str] = &[
    "href",
    "src",
    "srcset",
    "action",
    "formaction",
    "target",
    "rel",
    "role",
    "aria-label",
    "aria-labelledby",
    "aria-hidden",
    "aria-describedby",
];
pub(crate) const UNSAFE_HTML_ATTRIBUTES: &[&str] = &["innerHTML", "outerHTML", "html"];

/// Maps browser implementation words to their Frame semantic alternatives.
/// Each entry is (browser_word, frame_alternative, explanation).
pub(crate) const BROWSER_TO_SEMANTIC: &[(&str, &str, &str)] = &[
    ("a", "link", "`link` describes navigation intent. The browser anchor element is a renderer detail."),
    ("article", "panel", "`panel` describes a content region. The article tag is an HTML structural element."),
    ("audio", "media", "`media` describes audio or video playback intent, not the specific element type."),
    ("button", "action", "`action` describes a user-triggered command. The button element is the DOM lowering target."),
    ("canvas", "image", "`image` describes graphical content. For drawing surfaces, use the advanced CSS escape hatch."),
    ("div", "panel", "`panel` or `stack` describe layout containers. `div` is a generic DOM box with no semantic meaning."),
    ("fieldset", "panel", "`panel` groups related controls. The fieldset element is a browser form grouping detail."),
    ("footer", "dock", "`dock` anchors a region to an edge. The footer element is a browser structural convention."),
    ("form", "composer", "`composer` describes input collection and submission intent, not the form element."),
    ("h1", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("h2", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("h3", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("h4", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("h5", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("h6", "title", "`title` describes a heading. H1-H6 are browser presentation levels, not UI intent."),
    ("header", "toolbar", "`toolbar` describes a command region. The header element is a browser structural convention."),
    ("img", "image", "`image` describes visual content. The img element is the renderer target."),
    ("input", "input", "`input` describes text entry intent. The input element is the DOM lowering target."),
    ("label", "label", "`label` is a property that describes a visible name, not a standalone element."),
    ("li", "item", "`item` describes a list entry. The li element is the renderer lowering target."),
    ("main", "screen", "`screen` describes the primary view region. The main element is a browser landmark."),
    ("nav", "menu", "`menu` describes navigation choices. The nav element is a browser landmark."),
    ("ol", "list", "`list` describes an ordered sequence. The ol element is the DOM lowering target."),
    ("p", "text", "`text` describes a paragraph of content. The p element is a generic text container."),
    ("select", "choice", "`choice` describes a selection control. The select element is the DOM lowering target."),
    ("span", "text", "`text` or `badge` describe inline content. The span element has no semantic meaning."),
    ("table", "data", "`data` describes structured records. The table element is the DOM lowering target."),
    ("tbody", "data", "`data` describes record collections. The tbody element is a browser table detail."),
    ("td", "item", "`item` describes a cell of data. The td element is a browser table detail."),
    ("textarea", "editor", "`editor` describes multi-line text entry. The textarea element is the DOM lowering target."),
    ("th", "title", "`title` describes a column header. The th element is a browser table detail."),
    ("thead", "toolbar", "`toolbar` or `panel` describe a header region. The thead element is a browser table detail."),
    ("tr", "row", "`row` describes a horizontal sequence. The tr element is a browser table detail."),
    ("ul", "list", "`list` describes an unordered sequence. The ul element is the DOM lowering target."),
    ("video", "media", "`media` describes video playback intent, not the specific element type."),
];

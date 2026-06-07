pub(crate) const SEMANTIC_UI_PRIMITIVES: &[&str] = &[
    "screen", "panel", "section", "stack", "row", "grid", "split", "dock", "overlay", "scroll",
    "action", "link", "menu", "toolbar", "tabs", "input", "editor", "toggle", "choice", "select",
    "composer", "title", "text", "label", "badge", "avatar", "icon", "image", "list", "feed",
    "data", "item", "empty", "card", "dialog", "popover",
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

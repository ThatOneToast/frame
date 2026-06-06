pub const SPACING: &[&str] = &["none", "small", "medium", "large", "xlarge"];
pub const EDGES: &[&str] = &[
    "top", "right", "bottom", "left", "x", "y", "inline", "block",
];
pub const RADII: &[&str] = &["none", "small", "medium", "large", "xlarge", "pill", "full"];
pub const SURFACES: &[&str] = &[
    "panel", "main", "glass", "flat", "raised", "overlay", "inset", "sunken", "gradient",
];
pub const EFFECTS: &[&str] = &[
    "lift",
    "sink",
    "shift",
    "grow",
    "shrink",
    "tilt",
    "glow",
    "brighten",
    "dim",
    "press",
    "pop",
    "ring",
    "blur",
    "smooth",
    "fade",
    "scale",
    "rotate",
    "slide",
    "transition",
    "duration",
    "ease",
    "animation",
    "animate",
];
pub const MOVEMENT_AMOUNTS: &[&str] = &["tiny", "small", "medium", "large", "huge"];
pub const VISUAL_AMOUNTS: &[&str] = &["slight", "subtle", "normal", "strong", "dramatic"];
pub const SHADOWS: &[&str] = &[
    "none", "soft", "small", "medium", "large", "deep", "floating",
];
pub const GLOWS: &[&str] = &[
    "none", "accent", "danger", "success", "warning", "soft", "strong",
];
pub const SIZES: &[&str] = &[
    "screen", "fill", "content", "auto", "sidebar", "narrow", "wide", "small", "medium", "large",
    "xlarge", "zero", "modal", "icon",
];
pub const TRACKS: &[&str] = &[
    "rail", "panel", "side", "header", "composer", "fill", "auto", "content",
];
pub const LAYOUTS: &[&str] = &[
    "icon-content-action",
    "avatar-content",
    "header",
    "composer",
    "center",
];
pub const OVERFLOWS: &[&str] = &["hidden", "visible", "auto", "clip"];
pub const SCROLL_AXES: &[&str] = &["x", "y", "both"];
pub const SCROLLBARS: &[&str] = &["dense", "normal"];
pub const TEXT_WRAPS: &[&str] = &["anywhere", "normal"];
pub const TEXT_CASES: &[&str] = &["uppercase", "lowercase", "capitalize", "normal"];
pub const TEXT_ALIGN: &[&str] = &[
    "left",
    "center",
    "right",
    "start",
    "end",
    "justify",
    "match-parent",
];
pub const TEXT_DECORATIONS: &[&str] = &["none", "underline", "overline", "line-through"];
pub const WHITE_SPACE: &[&str] = &[
    "normal",
    "nowrap",
    "pre",
    "pre-wrap",
    "pre-line",
    "break-spaces",
];
pub const WORD_BREAKS: &[&str] = &["normal", "break-all", "keep-all", "break-word"];
pub const HYPHENS: &[&str] = &["none", "manual", "auto"];
pub const LINES: &[&str] = &["normal", "relaxed", "tight"];
pub const LETTERS: &[&str] = &["normal"];
pub const CONTROLS: &[&str] = &["reset"];
pub const BOX_SIZING: &[&str] = &["border", "content"];
pub const DISPLAY: &[&str] = &[
    "block",
    "inline",
    "inline-block",
    "flex",
    "inline-flex",
    "grid",
    "inline-grid",
    "contents",
    "none",
];
pub const VISIBILITY: &[&str] = &["visible", "hidden", "collapse"];
pub const FLEX_DIRECTIONS: &[&str] = &["row", "row-reverse", "column", "column-reverse"];
pub const FLEX_WRAPS: &[&str] = &["nowrap", "wrap", "wrap-reverse"];
pub const SQUARES: &[&str] = &["server", "avatar", "icon", "presence", "unread"];
pub const SELF_ALIGN: &[&str] = &["center", "start", "end", "stretch"];
pub const NUDGES: &[&str] = &["top-right"];
pub const COLORS: &[&str] = &[
    "main",
    "accent",
    "muted",
    "danger",
    "success",
    "warning",
    "info",
    "primary",
    "secondary",
    "bright",
    "white",
    "black",
    "gray",
    "slate",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "purple",
    "pink",
    "cyan",
    "transparent",
    "dusk",
    "midnight",
    "aurora",
    "ember",
    "ocean",
    "forest",
    "subtle",
];
pub const ALIGN: &[&str] = &["start", "center", "end", "stretch"];
pub const JUSTIFY: &[&str] = &["start", "center", "end", "between", "around", "evenly"];
pub const POSITIONS: &[&str] = &[
    "relative",
    "absolute",
    "sticky",
    "fixed",
    "center",
    "top",
    "bottom",
    "top-left",
    "top-right",
    "bottom-left",
    "bottom-right",
];
pub const ANCHORS: &[&str] = &[
    "top",
    "bottom",
    "left",
    "right",
    "top-left",
    "top-right",
    "bottom-left",
    "bottom-right",
];
pub const GRADIENT_TYPES: &[&str] = &["linear", "radial", "conic", "layered"];
pub const GRADIENT_CORNERS: &[&str] = &["top-left", "top-right", "bottom-left", "bottom-right"];
pub const GRID_FLOWS: &[&str] = &["horizontal", "vertical"];
pub const TRANSITIONS: &[&str] = &["smooth", "fast", "slow", "none"];
pub const DURATIONS: &[&str] = &["fast", "normal", "slow"];
pub const EASES: &[&str] = &["linear", "smooth", "bounce", "sharp"];
pub const ANIMATIONS: &[&str] = &["fade-in", "slide-up", "pop-in", "pulse", "none"];
pub const ANIMATION_PROPERTIES: &[&str] = &[
    "duration",
    "delay",
    "iteration",
    "direction",
    "fill",
    "play-state",
    "ease",
];
pub const ANIMATION_FILLS: &[&str] = &["none", "forwards", "backwards", "both"];
pub const ANIMATION_DIRECTIONS: &[&str] = &["normal", "reverse", "alternate", "alternate-reverse"];
pub const ANIMATION_PLAY_STATES: &[&str] = &["running", "paused"];
pub const BREAKPOINTS: &[&str] = &["mobile", "tablet", "desktop", "wide"];
pub const CONTAINERS: &[&str] = &["narrow", "content", "wide"];
pub const KEYFRAME_PROPERTIES: &[&str] = &[
    "opacity",
    "transform",
    "filter",
    "scale",
    "translate",
    "rotate",
];
pub const BORDER_STYLES: &[&str] = &[
    "none", "soft", "strong", "accent", "muted", "danger", "success", "warning", "width", "radius",
    "style",
];
pub const BORDER_LINE_STYLES: &[&str] = &[
    "none", "solid", "dashed", "dotted", "double", "groove", "ridge", "inset", "outset",
];
pub const Z_LAYERS: &[&str] = &[
    "base", "above", "dropdown", "sticky", "overlay", "modal", "toast",
];
